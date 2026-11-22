// Mutation fuzzing, after Bin2Wrong: take programs that already exist (the hand-written cases,
// generated seeds, any corpus), mutate their source over and over, and keep each mutant whose
// bytecode shows the decompilers something the pool hadn't. Which mutants are kept depends only
// on the bytecode, never on how any decompiler did, so a seed and a pool give every decompiler
// the same programs, in the same order, forever.

use std::collections::HashSet;

use rayon::prelude::*;

use crate::{
    bytecode,
    lex::{self, Kind, Token},
    luau::{self, DISABLED_BUILTINS, Ending, Extra, MUTABLE_GLOBALS, Profile},
    rng::Rng,
};

pub const MUTATOR_VERSION: &str = "m1";

// a mutant past this is mostly duplicated code, slow to run and to read
const MAX_SOURCE: usize = 16 << 10;
const BATCH: usize = 64;

#[derive(Clone)]
pub struct Mutant {
    pub name: String,
    pub source: String,
    pub extra: Extra,
    // the pool program it descends from, and every mutation on the way
    pub origin: String,
    pub ops: Vec<String>,
}

impl Mutant {
    // a file \`check\` can run again with the same compile options
    pub fn file_text(&self) -> String {
        let mut text = format!("-- rosetta mutant {} of {}: {}\n", self.name, self.origin, self.ops.join(", "));
        if !self.extra.is_default() {
            text.push_str(&format!("--!rosetta-options {}\n", self.extra));
        }
        text.push_str(&self.source);
        text
    }
}

pub struct Stats {
    pub attempts: usize,
    pub usable: usize,
    pub features: usize,
}

// The statement-shaped line ranges of a program: lines that start a statement, and for each the
// block depth it sits at
struct Lines<'a> {
    text: Vec<&'a str>,
    // per line: Some(depth) when a statement starts it
    stmt: Vec<Option<usize>>,
    // per line: the depth of its first token, None for blank lines and lines inside a string
    depth: Vec<Option<usize>>,
    // per line: names declared on lines before it
    names_before: Vec<usize>,
    names: Vec<&'a str>,
}

const STATEMENT_KEYWORDS: [&str; 9] = ["local", "if", "for", "while", "repeat", "do", "return", "break", "function"];

fn lines<'a>(src: &'a str, tokens: &[Token<'a>]) -> Lines<'a> {
    let text = src.split_inclusive('\n').collect::<Vec<_>>();
    let n = text.len();
    let nesting = lex::nesting(tokens);
    let mut stmt = vec![None; n];
    let mut depth = vec![None; n];
    let mut covered = vec![false; n + 2];
    let mut brackets = 0i32;
    let mut prev: Option<&Token> = None;
    for (i, t) in tokens.iter().enumerate() {
        let line = t.line - 1;
        if line < n && depth[line].is_none() && !covered[line] {
            depth[line] = Some(nesting.depth[i]);
            let starts = match t.kind {
                Kind::Name => true,
                Kind::Keyword => STATEMENT_KEYWORDS.contains(&t.text),
                _ => false,
            };
            let after = match prev {
                None => true,
                Some(p) => match p.kind {
                    Kind::Name | Kind::Number | Kind::Str => true,
                    Kind::Op => matches!(p.text, ")" | "]" | "}" | "..."),
                    Kind::Keyword => matches!(p.text, "do" | "then" | "else" | "end" | "break" | "true" | "false" | "nil" | "repeat"),
                },
            };
            if starts && after && brackets == 0 {
                stmt[line] = Some(nesting.depth[i]);
            }
        }
        if t.kind == Kind::Str {
            for extra in 1..=t.text.matches('\n').count() {
                covered[(line + extra).min(n + 1)] = true;
            }
        }
        match t.text {
            "(" | "{" | "[" if t.kind == Kind::Op => brackets += 1,
            ")" | "}" | "]" if t.kind == Kind::Op => brackets -= 1,
            _ => {}
        }
        prev = Some(t);
    }
    let declared = lex::declared(tokens);
    // declared() is in source order, so this is a running count
    let mut names_before = vec![0; n + 1];
    let mut k = 0;
    for (line, slot) in names_before.iter_mut().enumerate() {
        while k < declared.len() && declared[k].line - 1 < line {
            k += 1;
        }
        *slot = k;
    }
    let names = declared.iter().map(|t| t.text).collect();
    Lines { text, stmt, depth, names_before, names }
}

impl Lines<'_> {
    // where a run of statements starting at line i may end: before each later statement at the
    // same depth, and at the line that closes the enclosing block (or the end of the file)
    fn ends(&self, i: usize) -> Vec<usize> {
        let Some(d) = self.stmt[i] else { return Vec::new() };
        let mut ends = Vec::new();
        for k in i + 1..self.text.len() {
            match self.depth[k] {
                Some(dk) if dk < d => {
                    ends.push(k);
                    return ends;
                }
                Some(dk) if dk == d && self.stmt[k].is_some() => ends.push(k),
                _ => {}
            }
        }
        ends.push(self.text.len());
        ends
    }

    fn starts(&self) -> Vec<usize> {
        (0..self.text.len()).filter(|&i| self.stmt[i].is_some()).collect()
    }

    // a statement run [i, j): up to three statements long
    fn pick_range(&self, rng: &mut Rng) -> Option<(usize, usize)> {
        let starts = self.starts();
        if starts.is_empty() {
            return None;
        }
        let i = *rng.pick(&starts);
        let ends = self.ends(i);
        let j = ends[rng.below(ends.len().min(3))];
        Some((i, j))
    }

    fn join(&self, from: usize, to: usize) -> String {
        self.text[from..to].concat()
    }

    fn splice(&self, i: usize, j: usize, middle: &str) -> String {
        let mut out = self.join(0, i);
        out.push_str(middle);
        out.push_str(&self.join(j, self.text.len()));
        out
    }
}

const NUMBERS: [&str; 16] =
    ["0", "1", "2", "(-1)", "(0/0)", "(1/0)", "(-1/0)", "(-0)", "2^53", "0.1", "1e308", "(-2147483648)", "4294967296", "255", "0x7FFFFFFF", "1.5"];
const STRINGS: [&str; 10] = ["\"\"", "\"\\0\"", "\"é\"", "\"\\255\"", "\"%d\"", "\"a b\"", "\"end\"", "\"nil\"", "\"1e3\"", "\" 7 \""];
const FLIPS: [&[&str]; 5] = [&["<", "<=", ">", ">="], &["==", "~="], &["+", "-", "*"], &["and", "or"], &["..", "+"]];

// One mutation somewhere in src, or None when the one picked found no place to go
fn mutate(src: &str, rng: &mut Rng) -> Option<(String, String)> {
    let tokens = lex::tokenize(src);
    let lines = lines(src, &tokens);
    let tag = rng.below(1_000_000);
    let wrap = |rng: &mut Rng, open: &str, close: &str| {
        let (i, j) = lines.pick_range(rng)?;
        Some(lines.splice(i, j, &format!("{open}\n{}{close}\n", lines.join(i, j))))
    };
    let op = rng.below(19);
    let name = match op {
        0 => "wrap do",
        1 => "wrap if",
        2 => "wrap else",
        3 => "wrap for",
        4 => "wrap for twice",
        5 => "wrap while",
        6 => "wrap repeat",
        7 => "wrap local function",
        8 => "wrap closure",
        9 => "duplicate",
        10 => "delete",
        11 => "swap",
        12 => "negate",
        13 => "flip",
        14 => "number",
        15 => "string",
        16 => "print",
        17 => "early return",
        _ => "wrap local function with args",
    };
    let out = match op {
        0 => wrap(rng, "do", "end")?,
        // __rosetta is a global nothing sets, so these stay opaque to the compiler
        1 => wrap(rng, "if not __rosetta then", "end")?,
        2 => wrap(rng, "if __rosetta then\nelse", "end")?,
        3 => wrap(rng, "for _ = 1, 1 do", "end")?,
        4 => wrap(rng, "for _ = 1, 2 do", "end")?,
        5 => wrap(rng, "while true do", "break\nend")?,
        6 => wrap(rng, "repeat", "until true")?,
        // -O2 inlines these, so they exercise de-inlining
        7 => wrap(rng, &format!("local function __m{tag}()"), &format!("end\n__m{tag}()"))?,
        8 => wrap(rng, "local _ = (function()", "end)()")?,
        9 => {
            let (i, j) = lines.pick_range(rng)?;
            let run = lines.join(i, j);
            lines.splice(i, j, &format!("{run}{run}"))
        }
        10 => {
            let (i, j) = lines.pick_range(rng)?;
            lines.splice(i, j, "")
        }
        11 => {
            let (i, j) = lines.pick_range(rng)?;
            if j >= lines.text.len() || lines.stmt[j] != lines.stmt[i] {
                return None;
            }
            let next = lines.ends(j)[0];
            lines.splice(i, next, &format!("{}{}", lines.join(j, next), lines.join(i, j)))
        }
        12 => {
            let heads = tokens.iter().enumerate().filter(|(_, t)| t.kw("if") || t.kw("elseif") || t.kw("while")).map(|(i, _)| i).collect::<Vec<_>>();
            let &h = pick(rng, &heads)?;
            // the then/do that ends this condition: every if in between brings its own then
            let mut pending = 1;
            let close = tokens[h + 1..].iter().position(|t| {
                if t.kw("if") || t.kw("while") || t.kw("for") {
                    pending += 1;
                } else if t.kw("then") || t.kw("do") {
                    pending -= 1;
                }
                pending == 0
            })? + h + 1;
            let (a, b) = (tokens[h].end, tokens[close].start);
            format!("{} not ({}) {}", &src[..a], &src[a..b], &src[b..])
        }
        13 => {
            let sites = tokens
                .iter()
                .filter_map(|t| FLIPS.iter().find(|group| group.contains(&t.text) && matches!(t.kind, Kind::Op | Kind::Keyword)).map(|g| (t, *g)))
                .collect::<Vec<_>>();
            let (t, group) = *pick(rng, &sites)?;
            let other = group.iter().filter(|o| **o != t.text).collect::<Vec<_>>();
            format!("{} {} {}", &src[..t.start], rng.pick(&other), &src[t.end..])
        }
        14 => {
            let sites = tokens.iter().filter(|t| t.kind == Kind::Number).collect::<Vec<_>>();
            let t = *pick(rng, &sites)?;
            format!("{}{}{}", &src[..t.start], rng.pick(&NUMBERS), &src[t.end..])
        }
        15 => {
            let sites = tokens.iter().filter(|t| t.kind == Kind::Str && (t.text.starts_with('"') || t.text.starts_with('\''))).collect::<Vec<_>>();
            let t = *pick(rng, &sites)?;
            format!("{}{}{}", &src[..t.start], rng.pick(&STRINGS), &src[t.end..])
        }
        // print a variable declared earlier: one more value the round trip has to get right
        16 => {
            let starts = lines.starts().into_iter().filter(|&i| lines.names_before[i] > 0).collect::<Vec<_>>();
            let &i = pick(rng, &starts)?;
            let name = lines.names[rng.below(lines.names_before[i])];
            lines.splice(i, i, &format!("print(\"m{tag}\", {name})\n"))
        }
        17 => {
            let starts = lines.starts();
            let &i = pick(rng, &starts)?;
            lines.splice(i, i, "if __rosetta then return end\n")
        }
        _ => {
            // a helper taking variables from the scope as parameters: inlined at -O2
            let (i, j) = lines.pick_range(rng)?;
            let known = lines.names_before[i];
            if known == 0 {
                return None;
            }
            let mut args = (0..1 + rng.below(3)).map(|_| lines.names[rng.below(known)]).collect::<Vec<_>>();
            args.sort();
            args.dedup();
            let list = args.join(", ");
            lines.splice(i, j, &format!("local function __m{tag}({list})\n{}end\n__m{tag}({list})\n", lines.join(i, j)))
        }
    };
    Some((out, name.to_string()))
}

fn pick<'a, T>(rng: &mut Rng, items: &'a [T]) -> Option<&'a T> {
    (!items.is_empty()).then(|| rng.pick(items))
}

fn mutate_extra(extra: Extra, rng: &mut Rng) -> Extra {
    let mut e = extra;
    match rng.below(4) {
        0 => e.coverage = rng.below(3) as u8,
        1 => e.type_info ^= 1,
        2 => e.mutable ^= 1 << rng.below(MUTABLE_GLOBALS.len()),
        _ => e.disabled ^= 1 << rng.below(DISABLED_BUILTINS.len()),
    }
    e
}

// The novelty signal for a program: features of its bytecode at every optimization level
// How much a program exercises: distinct opcode pairs, triples and jump landings over -O0..-O2.
// The benchmark's quick tier picks the programs with the most
pub fn feature_count(source: &str, extra: Extra) -> usize {
    features(source, extra).map_or(0, |f| f.len())
}

fn features(source: &str, extra: Extra) -> Option<HashSet<u32>> {
    let mut out = HashSet::new();
    for opt in 0..=2u8 {
        let bytecode = luau::compile(source, Profile { opt, debug: 1, extra }).ok()?;
        if opt == 1 {
            let run = luau::run(&bytecode);
            match run.ending {
                Ending::TimedOut => return None,
                Ending::Errored(e) if e == luau::OUT_OF_MEMORY || run.log.is_empty() => return None,
                _ => {}
            }
        }
        bytecode::features(&bytecode, opt as u32, &mut out);
    }
    Some(out)
}

fn candidate(pool: &[Mutant], rng: &mut Rng, compile_options: bool) -> Option<(Mutant, HashSet<u32>)> {
    let parent = rng.pick(pool);
    let mut source = parent.source.clone();
    let mut ops = parent.ops.clone();
    for _ in 0..1 + rng.below(3) {
        let applied = (0..8).find_map(|_| mutate(&source, rng));
        let (next, op) = applied?;
        source = next;
        ops.push(op);
    }
    let mut extra = parent.extra;
    if compile_options && rng.chance(30) {
        extra = mutate_extra(extra, rng);
        ops.push(format!("options {extra}"));
    }
    if source.len() > MAX_SOURCE {
        return None;
    }
    let features = features(&source, extra)?;
    Some((Mutant { name: String::new(), source, extra, origin: parent.origin.clone(), ops }, features))
}

// Grows the pool until count mutants have been kept or the attempts run out. Candidates are
// made in parallel batches but judged in order, so the result doesn't depend on thread count
pub fn evolve(pool: Vec<(String, String)>, seed: u64, count: usize, compile_options: bool) -> (Vec<Mutant>, Stats) {
    let mut seen = HashSet::new();
    let mut pool = pool
        .into_iter()
        .filter_map(|(name, source)| {
            let source = if source.ends_with('\n') { source } else { source + "\n" };
            let f = features(&source, Extra::default())?;
            seen.extend(f);
            Some(Mutant { name: name.clone(), source, extra: Extra::default(), origin: name, ops: Vec::new() })
        })
        .collect::<Vec<_>>();
    let mut kept = Vec::new();
    let mut stats = Stats { attempts: 0, usable: 0, features: 0 };
    if pool.is_empty() {
        return (kept, stats);
    }
    let budget = count.saturating_mul(50).max(BATCH);
    let mut batch = 0u64;
    while kept.len() < count && stats.attempts < budget {
        let results = (0..BATCH as u64)
            .into_par_iter()
            .map(|slot| {
                let mut rng = Rng::new(seed.wrapping_mul(0x2545_F491_4F6C_DD1D) ^ (batch * BATCH as u64 + slot).wrapping_mul(0x9E37_79B9_7F4A_7C15));
                candidate(&pool, &mut rng, compile_options)
            })
            .collect::<Vec<_>>();
        batch += 1;
        stats.attempts += BATCH;
        for (mut mutant, features) in results.into_iter().flatten() {
            stats.usable += 1;
            if kept.len() < count && features.iter().any(|f| !seen.contains(f)) {
                seen.extend(features);
                mutant.name = format!("mut{seed}-{}", kept.len() + 1);
                pool.push(mutant.clone());
                kept.push(mutant);
            }
        }
    }
    stats.features = seen.len();
    (kept, stats)
}

// Deletes statement runs for as long as the program keeps failing the same way, biggest first
pub fn reduce(source: &str, still_fails: impl Fn(&str) -> bool, budget: usize) -> String {
    let mut current = source.to_string();
    let mut attempts = 0;
    'outer: while attempts < budget {
        let tokens = lex::tokenize(&current);
        let lines = lines(&current, &tokens);
        let mut ranges = lines.starts().into_iter().flat_map(|i| lines.ends(i).into_iter().map(move |j| (i, j))).collect::<Vec<_>>();
        ranges.sort_by_key(|&(i, j)| (std::cmp::Reverse(j - i), i));
        ranges.dedup();
        for (i, j) in ranges {
            if attempts >= budget {
                break 'outer;
            }
            attempts += 1;
            let candidate = lines.splice(i, j, "");
            if still_fails(&candidate) {
                current = candidate;
                continue 'outer;
            }
        }
        break;
    }
    current
}

pub fn pool_hash(pool: &[(String, String)]) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for (name, source) in pool {
        for b in name.bytes().chain([0]).chain(source.bytes()).chain([0]) {
            h = (h ^ b as u64).wrapping_mul(0x100_0000_01b3);
        }
    }
    format!("{:08x}", h >> 32)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SRC: &str = "local t = {}\nfor i = 1, 3 do\n\tif i % 2 == 0 then\n\t\tt[#t + 1] = i\n\telse\n\t\tprint(i)\n\tend\nend\nlocal function f(a, b)\n\treturn a .. b\nend\nprint(f(\"x\", #t))\n";

    #[test]
    fn statement_lines() {
        let tokens = lex::tokenize(SRC);
        let l = lines(SRC, &tokens);
        let starts = l.starts();
        assert_eq!(starts, [0, 1, 2, 3, 5, 8, 9, 11]);
        assert_eq!(l.ends(0), [1, 8, 11, 12]);
        assert_eq!(l.ends(3), [4]);
        assert_eq!(l.ends(5), [6]);
    }

    #[test]
    fn mutants_mostly_compile_and_are_reproducible() {
        let mut compiled = 0;
        for seed in 0..200 {
            let mut rng = Rng::new(seed);
            if let Some((out, _)) = mutate(SRC, &mut rng) {
                compiled += luau::compile(&out, Profile::new(1, 1)).is_ok() as usize;
            }
        }
        assert!(compiled > 120, "{compiled}");
        let pool = vec![("t".to_string(), SRC.to_string())];
        let a = evolve(pool.clone(), 7, 10, true).0.into_iter().map(|m| m.source).collect::<Vec<_>>();
        let b = evolve(pool, 7, 10, true).0.into_iter().map(|m| m.source).collect::<Vec<_>>();
        assert!(!a.is_empty());
        assert_eq!(a, b);
    }
}
