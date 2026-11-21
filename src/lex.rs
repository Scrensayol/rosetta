// A Luau tokenizer and a light structural reading of the token stream: block nesting, declared
// names, statement-shaped line ranges. Enough to measure a decompiler's output next to the
// original source and to find places to mutate, without a full parser. Everything it reads
// compiles, so it only has to be right on valid Luau.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Kind {
    Name,
    Keyword,
    Number,
    Str,
    Op,
}

#[derive(Clone, Copy, Debug)]
pub struct Token<'a> {
    pub kind: Kind,
    pub text: &'a str,
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

impl Token<'_> {
    pub fn is(&self, kind: Kind, text: &str) -> bool {
        self.kind == kind && self.text == text
    }

    pub fn kw(&self, text: &str) -> bool {
        self.is(Kind::Keyword, text)
    }

    pub fn op(&self, text: &str) -> bool {
        self.is(Kind::Op, text)
    }
}

const KEYWORDS: [&str; 21] = [
    "and", "break", "do", "else", "elseif", "end", "false", "for", "function", "if", "in", "local", "nil", "not", "or", "repeat",
    "return", "then", "true", "until", "while",
];

const OPS: [&str; 17] = ["...", "//=", "..=", "..", "==", "~=", "<=", ">=", "+=", "-=", "*=", "/=", "%=", "^=", "->", "::", "//"];

pub fn tokenize(src: &str) -> Vec<Token<'_>> {
    let mut lexer = Lexer { src, b: src.as_bytes(), i: 0, line: 1, out: Vec::new() };
    lexer.run(false);
    lexer.out
}

struct Lexer<'a> {
    src: &'a str,
    b: &'a [u8],
    i: usize,
    line: usize,
    out: Vec<Token<'a>>,
}

impl<'a> Lexer<'a> {
    fn peek(&self, ahead: usize) -> Option<u8> {
        self.b.get(self.i + ahead).copied()
    }

    // every boundary is at an ASCII byte or the end, so slicing never splits a character
    fn push(&mut self, kind: Kind, start: usize, line: usize) {
        self.out.push(Token { kind, text: &self.src[start..self.i], line, start, end: self.i });
    }

    // with brace, returns after the } that closes an interpolated string's expression
    fn run(&mut self, brace: bool) {
        let mut depth = 0usize;
        while self.i < self.b.len() {
            let c = self.b[self.i];
            let (start, line) = (self.i, self.line);
            match c {
                b'\n' => {
                    self.line += 1;
                    self.i += 1;
                }
                b' ' | b'\t' | b'\r' | 0x0b | 0x0c => self.i += 1,
                b'-' if self.peek(1) == Some(b'-') => {
                    self.i += 2;
                    if self.long_level().is_some() {
                        self.long_bracket();
                    } else {
                        while self.i < self.b.len() && self.b[self.i] != b'\n' {
                            self.i += 1;
                        }
                    }
                }
                b'"' | b'\'' => {
                    self.quoted(c);
                    self.push(Kind::Str, start, line);
                }
                b'\x60' => self.interpolated(),
                b'[' if self.long_level().is_some() => {
                    self.long_bracket();
                    self.push(Kind::Str, start, line);
                }
                b'0'..=b'9' => self.number(),
                b'.' if self.peek(1).is_some_and(|d| d.is_ascii_digit()) => self.number(),
                c if c == b'_' || c.is_ascii_alphabetic() || c >= 0x80 => {
                    while self.i < self.b.len() && (self.b[self.i] == b'_' || self.b[self.i].is_ascii_alphanumeric() || self.b[self.i] >= 0x80) {
                        self.i += 1;
                    }
                    let kind = if KEYWORDS.contains(&&self.src[start..self.i]) { Kind::Keyword } else { Kind::Name };
                    self.push(kind, start, line);
                }
                _ => {
                    if brace {
                        if c == b'{' {
                            depth += 1;
                        } else if c == b'}' {
                            if depth == 0 {
                                self.i += 1;
                                return;
                            }
                            depth -= 1;
                        }
                    }
                    let rest = &self.b[self.i..];
                    let len = OPS.iter().find(|op| rest.starts_with(op.as_bytes())).map_or(1, |op| op.len());
                    self.i += len;
                    self.push(Kind::Op, start, line);
                }
            }
        }
    }

    // at [, the level of a long bracket ([[ or [==[) if one starts here
    fn long_level(&self) -> Option<usize> {
        if self.peek(0) != Some(b'[') {
            return None;
        }
        let level = self.b[self.i + 1..].iter().take_while(|&&c| c == b'=').count();
        (self.peek(1 + level) == Some(b'[')).then_some(level)
    }

    fn long_bracket(&mut self) {
        let level = self.long_level().unwrap();
        self.i += level + 2;
        while self.i < self.b.len() {
            match self.b[self.i] {
                b'\n' => self.line += 1,
                b']' if self.b[self.i + 1..].iter().take_while(|&&c| c == b'=').count() == level && self.peek(1 + level) == Some(b']') => {
                    self.i += level + 2;
                    return;
                }
                _ => {}
            }
            self.i += 1;
        }
    }

    fn quoted(&mut self, quote: u8) {
        self.i += 1;
        while self.i < self.b.len() {
            match self.b[self.i] {
                b'\\' => {
                    if self.peek(1) == Some(b'\n') {
                        self.line += 1;
                    }
                    self.i += 2;
                    continue;
                }
                b'\n' => {
                    self.line += 1;
                    break;
                }
                c if c == quote => {
                    self.i += 1;
                    break;
                }
                _ => {}
            }
            self.i += 1;
        }
        self.i = self.i.min(self.b.len());
    }

    // one Str token for the literal; the expressions inside {} are tokenized in place
    fn interpolated(&mut self) {
        let (start, line) = (self.i, self.line);
        let mut pushed = false;
        self.i += 1;
        while self.i < self.b.len() {
            match self.b[self.i] {
                b'\\' => {
                    self.i += 2;
                    continue;
                }
                b'\n' => self.line += 1,
                b'\x60' => {
                    self.i += 1;
                    break;
                }
                b'{' => {
                    if !pushed {
                        self.push(Kind::Str, start, line);
                        pushed = true;
                    }
                    self.i += 1;
                    self.run(true);
                    continue;
                }
                _ => {}
            }
            self.i += 1;
        }
        self.i = self.i.min(self.b.len());
        if !pushed {
            self.push(Kind::Str, start, line);
        }
    }

    fn number(&mut self) {
        let (start, line) = (self.i, self.line);
        let hex = self.b[self.i..].starts_with(b"0x") || self.b[self.i..].starts_with(b"0X");
        while self.i < self.b.len() {
            let c = self.b[self.i];
            if !hex && (c == b'e' || c == b'E') && matches!(self.peek(1), Some(b'+' | b'-')) {
                self.i += 2;
            } else if c.is_ascii_alphanumeric() || c == b'_' || c == b'.' {
                self.i += 1;
            } else {
                break;
            }
        }
        self.push(Kind::Number, start, line);
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Frame {
    Function,
    If,
    // while/for, until its do
    Loop { body: bool },
    Do,
    Repeat,
    // `if a then b else c`: no end, and it's over once its else is seen
    ExprIf,
}

// What a token can be followed by: true when a statement may start after it. Decides whether
// an `if` is a statement or an if-expression.
fn ends_statement(prev: Option<&Token>, owned_by_expr_if: bool) -> bool {
    let Some(t) = prev else { return true };
    match t.kind {
        Kind::Name | Kind::Number | Kind::Str => true,
        Kind::Op => matches!(t.text, ")" | "]" | "}" | ";" | "..."),
        Kind::Keyword => match t.text {
            "then" | "else" => !owned_by_expr_if,
            "do" | "end" | "break" | "true" | "false" | "nil" | "repeat" => true,
            _ => false,
        },
    }
}

pub struct Nesting {
    // blocks open around each token; a block's opening and closing keywords count as outside it
    pub depth: Vec<usize>,
    pub max: usize,
}

pub fn nesting(tokens: &[Token]) -> Nesting {
    let mut stack: Vec<Frame> = Vec::new();
    let mut depth = Vec::with_capacity(tokens.len());
    let mut max = 0;
    let mut owned_by_expr_if = false;
    let blocks = |stack: &[Frame]| stack.iter().filter(|f| **f != Frame::ExprIf).count();
    for (i, t) in tokens.iter().enumerate() {
        let before = blocks(&stack);
        let mut at = before;
        if t.kind == Kind::Keyword {
            match t.text {
                "function" => stack.push(Frame::Function),
                "if" => {
                    let prev = i.checked_sub(1).map(|p| &tokens[p]);
                    stack.push(if ends_statement(prev, owned_by_expr_if) { Frame::If } else { Frame::ExprIf });
                }
                "then" | "elseif" => {
                    owned_by_expr_if = stack.last() == Some(&Frame::ExprIf);
                    if !owned_by_expr_if {
                        at = before.saturating_sub(1);
                    }
                }
                "else" => {
                    owned_by_expr_if = stack.last() == Some(&Frame::ExprIf);
                    if owned_by_expr_if {
                        stack.pop();
                    } else {
                        at = before.saturating_sub(1);
                    }
                }
                "while" | "for" => stack.push(Frame::Loop { body: false }),
                "do" => match stack.last_mut() {
                    Some(Frame::Loop { body }) if !*body => {
                        *body = true;
                        at = before.saturating_sub(1);
                    }
                    _ => stack.push(Frame::Do),
                },
                "repeat" => stack.push(Frame::Repeat),
                "end" | "until" => {
                    while stack.pop() == Some(Frame::ExprIf) {}
                    at = blocks(&stack);
                }
                _ => {}
            }
        }
        max = max.max(blocks(&stack));
        depth.push(at);
    }
    Nesting { depth, max }
}

// The name tokens a program declares: locals, local functions, parameters, loop variables, in
// source order. `_` is skipped
pub fn declared<'a>(tokens: &[Token<'a>]) -> Vec<Token<'a>> {
    let mut names = Vec::new();
    let name_at = |j: usize| tokens.get(j).filter(|t| t.kind == Kind::Name);
    // a name list: a, b: T, c ... up to whatever follows it
    let list = |mut j: usize, names: &mut Vec<Token<'a>>| {
        while let Some(t) = name_at(j) {
            names.push(*t);
            j += 1;
            if tokens.get(j).is_some_and(|t| t.op(":")) {
                j = skip_type(tokens, j + 1);
            }
            if tokens.get(j).is_some_and(|t| t.op(",")) {
                j += 1;
            } else {
                break;
            }
        }
    };
    for (i, t) in tokens.iter().enumerate() {
        let declaration = t.kw("local") || (t.is(Kind::Name, "const") && name_at(i + 1).is_some());
        if declaration {
            if tokens.get(i + 1).is_some_and(|n| n.kw("function")) {
                if let Some(n) = name_at(i + 2) {
                    names.push(*n);
                }
            } else {
                list(i + 1, &mut names);
            }
        } else if t.kw("for") {
            list(i + 1, &mut names);
        } else if t.kw("function") {
            let mut j = i + 1;
            while j < tokens.len() && !tokens[j].op("(") && j - i < 64 {
                j += 1;
            }
            list(j + 1, &mut names);
        }
    }
    names.retain(|n| n.text != "_");
    names
}

// Past a type annotation; stops at a , = ) or in at depth 0, or where the next statement starts
fn skip_type(tokens: &[Token], mut j: usize) -> usize {
    let mut depth = 0i32;
    let mut complete = false;
    while let Some(t) = tokens.get(j) {
        if depth == 0 {
            if t.op(",") || t.op("=") || t.op(")") || t.kw("in") || (t.kind == Kind::Keyword && !t.kw("nil")) {
                return j;
            }
            if complete && (t.kind == Kind::Name || t.kind == Kind::Number) {
                return j;
            }
        }
        match t.text {
            "(" | "{" | "[" | "<" => depth += 1,
            ")" | "}" | "]" | ">" => depth -= 1,
            _ => {}
        }
        if depth < 0 {
            return j;
        }
        complete = depth == 0 && (matches!(t.kind, Kind::Name | Kind::Str) || matches!(t.text, ")" | "}" | "]" | ">" | "?" | "nil"));
        j += 1;
    }
    j
}

// `while true do` and `repeat ... until false`: the loops a decompiler falls back to when it
// can't find a loop's real condition
pub fn endless_loops(tokens: &[Token]) -> usize {
    let at = |i: usize| tokens.get(i);
    (0..tokens.len())
        .filter(|&i| {
            let (a, b, c) = (&tokens[i], at(i + 1), at(i + 2));
            let while_true = a.kw("while") && b.is_some_and(|t| t.kw("true")) && c.is_some_and(|t| t.kw("do"));
            let until_false = a.kw("until") && b.is_some_and(|t| t.kw("false")) && !c.is_some_and(|t| matches!(t.text, "or" | "and" | "==" | "~="));
            while_true || until_false
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn texts(src: &str) -> Vec<&str> {
        tokenize(src).iter().map(|t| t.text).collect()
    }

    #[test]
    fn strings_and_comments() {
        assert_eq!(texts("local s = [==[a]]b]==] -- c\n--[[ d\n]] x"), ["local", "s", "=", "[==[a]]b]==]", "x"]);
        assert_eq!(texts("a = 'x\\'y' .. \"z\""), ["a", "=", "'x\\'y'", "..", "\"z\""]);
        assert_eq!(texts("f(\x60a{b + {c = 1}}d\x60, 1e-3, 0x1F, .5)"), ["f", "(", "\x60a", "b", "+", "{", "c", "=", "1", "}", ",", "1e-3", ",", "0x1F", ",", ".5", ")"]);
        assert_eq!(tokenize("a\n--[[\n\n]]\nb")[1].line, 5);
    }

    #[test]
    fn nesting_depths() {
        let src = "local x = if a then 1 else 2\nfor i = 1, 2 do\n  if if b then c else d then\n    while true do break end\n  elseif e then\n  else\n  end\nend\nreturn function() end";
        let tokens = tokenize(src);
        let n = nesting(&tokens);
        assert_eq!(n.max, 3);
        let at = |line: usize| n.depth[tokens.iter().position(|t| t.line == line).unwrap()];
        assert_eq!((1..=9).map(at).collect::<Vec<_>>(), [0, 0, 1, 2, 1, 1, 1, 0, 0]);
    }

    #[test]
    fn declared_names() {
        let src = "local a, b: {number} = 1\nlocal function f(x, y: string, ...) end\nfor k, _ in t do end\nfunction M.g(self) end\nlocal c: number\nprint(c)";
        assert_eq!(declared(&tokenize(src)).iter().map(|t| t.text).collect::<Vec<_>>(), ["a", "b", "f", "x", "y", "k", "self", "c"]);
    }

    #[test]
    fn endless() {
        assert_eq!(endless_loops(&tokenize("while true do end while x do end repeat until false")), 2);
    }
}
