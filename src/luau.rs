use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    fmt,
    rc::Rc,
    time::{Duration, Instant},
};

use mlua::{
    Lua, MultiValue, VmState,
    chunk::{ChunkMode, Compiler},
};
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
pub struct Profile {
    pub opt: u8,
    pub debug: u8,
    pub extra: Extra,
}

// Compiler options past -O and -g, which only the mutation fuzzer's --compile-options turns on.
// Real scripts are compiled without them, so they're kept out of the default runs
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
pub struct Extra {
    // COVERAGE instructions: 1 per statement, 2 per expression too
    pub coverage: u8,
    // type information for every function, not just native ones
    pub type_info: u8,
    // a mask over MUTABLE_GLOBALS: no GETIMPORT for fields reached through these
    pub mutable: u8,
    // a mask over DISABLED_BUILTINS: these compile to plain calls instead of FASTCALL
    pub disabled: u16,
    // not a compiler option: the bytecode is rewritten at this level after compiling (see
    // transform.rs)
    pub transform: u8,
    // not a compiler option either: the source is obfuscated at this level before compiling (see
    // obfuscate.rs)
    pub obfuscate: u8,
}

pub const MUTABLE_GLOBALS: [&str; 8] = ["math", "string", "table", "bit32", "buffer", "utf8", "os", "coroutine"];
pub const DISABLED_BUILTINS: [&str; 16] = [
    "assert", "type", "typeof", "rawequal", "rawget", "select", "tostring", "tonumber", "math.floor", "math.abs", "math.max", "math.min",
    "string.sub", "string.byte", "string.len", "table.insert",
];

impl Extra {
    pub fn is_default(&self) -> bool {
        *self == Extra::default()
    }

    // "+c2+t1+m5+d300+x2", the inverse of Display
    pub fn parse(mut text: &str) -> Option<Extra> {
        let mut extra = Extra::default();
        while let Some(rest) = text.strip_prefix('+') {
            let end = rest[1..].find('+').map_or(rest.len(), |i| i + 1);
            let (flag, value) = rest.get(..1).zip(rest.get(1..end))?;
            match flag {
                "c" => extra.coverage = value.parse().ok().filter(|v| *v <= 2)?,
                "t" => extra.type_info = value.parse().ok().filter(|v| *v <= 1)?,
                "m" => extra.mutable = value.parse().ok()?,
                "d" => extra.disabled = value.parse().ok()?,
                "x" => extra.transform = value.parse().ok().filter(|v| (1..=crate::transform::LEVELS).contains(v))?,
                "o" => extra.obfuscate = value.parse().ok().filter(|v| (1..=crate::obfuscate::LEVELS).contains(v))?,
                _ => return None,
            }
            text = &rest[end..];
        }
        text.is_empty().then_some(extra)
    }
}

impl fmt::Display for Extra {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (flag, value) in [("c", self.coverage as u32), ("t", self.type_info as u32), ("m", self.mutable as u32), ("d", self.disabled as u32), ("x", self.transform as u32), ("o", self.obfuscate as u32)] {
            if value != 0 {
                write!(f, "+{flag}{value}")?;
            }
        }
        Ok(())
    }
}

impl Profile {
    pub const ALL: [Profile; 6] = [
        Profile { opt: 0, debug: 1, extra: Extra { coverage: 0, type_info: 0, mutable: 0, disabled: 0, transform: 0, obfuscate: 0 } },
        Profile { opt: 1, debug: 1, extra: Extra { coverage: 0, type_info: 0, mutable: 0, disabled: 0, transform: 0, obfuscate: 0 } },
        Profile { opt: 2, debug: 1, extra: Extra { coverage: 0, type_info: 0, mutable: 0, disabled: 0, transform: 0, obfuscate: 0 } },
        Profile { opt: 0, debug: 2, extra: Extra { coverage: 0, type_info: 0, mutable: 0, disabled: 0, transform: 0, obfuscate: 0 } },
        Profile { opt: 1, debug: 2, extra: Extra { coverage: 0, type_info: 0, mutable: 0, disabled: 0, transform: 0, obfuscate: 0 } },
        Profile { opt: 2, debug: 2, extra: Extra { coverage: 0, type_info: 0, mutable: 0, disabled: 0, transform: 0, obfuscate: 0 } },
    ];

    pub fn new(opt: u8, debug: u8) -> Profile {
        Profile { opt, debug, extra: Extra::default() }
    }

    pub fn parse(text: &str) -> Option<Profile> {
        let rest = text.strip_prefix('O')?;
        let (opt, rest) = rest.split_once('g')?;
        let digits = rest.find('+').unwrap_or(rest.len());
        let profile = Profile { opt: opt.parse().ok()?, debug: rest[..digits].parse().ok()?, extra: Extra::parse(&rest[digits..])? };
        (profile.opt <= 2 && profile.debug <= 2).then_some(profile)
    }
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "O{}g{}{}", self.opt, self.debug, self.extra)
    }
}

pub fn compile(source: &str, profile: Profile) -> Result<Vec<u8>, String> {
    let pick = |names: &[&'static str], mask: u32| names.iter().enumerate().filter(|(i, _)| mask >> i & 1 == 1).map(|(_, n)| *n).collect::<Vec<_>>();
    let mut compiler = Compiler::new().set_optimization_level(profile.opt).set_debug_level(profile.debug);
    let extra = profile.extra;
    if !extra.is_default() {
        compiler = compiler
            .set_coverage_level(extra.coverage)
            .set_type_info_level(extra.type_info)
            .set_mutable_globals(pick(&MUTABLE_GLOBALS, extra.mutable as u32))
            .set_disabled_builtins(pick(&DISABLED_BUILTINS, extra.disabled as u32));
    }
    let bytecode = compiler.compile(source).map_err(|e| e.to_string())?;
    // luau_compile reports a syntax error as a chunk whose first byte is 0
    match bytecode.split_first() {
        Some((0, message)) => Err(String::from_utf8_lossy(message).into_owned()),
        _ => Ok(bytecode),
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Ending {
    Returned,
    Errored(String),
    TimedOut,
}

// What a run did: what it printed and how it ended, and what it left behind in the global
// environment and returned. A decompiler's output has to match all of it to pass. The calls it
// made to standard library functions that change things (see TRACER) are recorded too, and scored
// on their own: whether a pass also changed the same things in the same order
#[derive(Clone, Debug, Serialize)]
pub struct Observation {
    pub log: Vec<String>,
    pub ending: Ending,
    // "name = value" for every global the run created, changed or removed, and "lib.key = value"
    // for every field it changed in a table that was already there, sorted; then "return: value"
    // for what the main chunk returned
    pub state: Vec<String>,
    #[serde(skip)]
    pub calls: Calls,
}

impl PartialEq for Observation {
    fn eq(&self, other: &Observation) -> bool {
        self.log == other.log && self.ending == other.ending && self.state == other.state
    }
}

// Every traced call, in order: `table.insert({...}, 3)`. Only the first
// CALLS_KEPT are kept as text, the rest go into the count and the hash
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Calls {
    pub count: u64,
    pub hash: u64,
    pub first: Vec<String>,
}

const CALLS_KEPT: usize = 2000;

impl Calls {
    fn push(&mut self, call: String) {
        self.count += 1;
        if self.hash == 0 {
            self.hash = 0xcbf2_9ce4_8422_2325;
        }
        for b in call.bytes().chain([0]) {
            self.hash = (self.hash ^ b as u64).wrapping_mul(0x100_0000_01b3);
        }
        if self.first.len() < CALLS_KEPT {
            self.first.push(call);
        }
    }

    // where two runs' calls part ways, for a gallery or a log
    pub fn first_difference(&self, other: &Calls) -> Option<String> {
        if self == other {
            return None;
        }
        for (i, (a, b)) in self.first.iter().zip(&other.first).enumerate() {
            if a != b {
                return Some(format!("call {}: want `{a}` got `{b}`", i + 1));
            }
        }
        Some(format!("made {} calls, want {}", other.count, self.count))
    }
}

impl Observation {
    pub fn render(&self) -> String {
        let mut text = self.log.join("\n");
        text.push_str(&format!("\n== {:?}\n", self.ending));
        text
    }
}

// Runs once in every VM, before the prelude and the program: the standard library functions that
// change something (a table, a metatable, a buffer, which coroutine runs) are wrapped so calling
// one is recorded first. Pure functions are left alone: what they return already shows in what the
// program does, and the compiler folds some of them away when their arguments are constants, so
// two faithful programs can call them a different number of times. Calling getfenv turns off the
// VM's fast paths for builtins (FASTCALL, imports resolved when the chunk loads), so a call
// reaches its wrapper whatever shape the code making it has. math.random is seeded the same way
// every time, so a program drawing random numbers does the same thing run after run.
const TRACER: &str = r#"
local record = ...
local env = getfenv(1)
math.randomseed(1)
local traced = {
	table = { "insert", "remove", "sort", "clear", "move", "freeze" },
	coroutine = { "create", "resume", "yield", "wrap", "close" },
	buffer = {
		"writei8", "writeu8", "writei16", "writeu16", "writei32", "writeu32", "writef32", "writef64",
		"writestring", "writebits", "fill", "copy",
	},
	_G = { "setmetatable", "rawset" },
}
for lib, names in traced do
	local t = if lib == "_G" then env else env[lib]
	if type(t) == "table" then
		for _, name in names do
			local f = t[name]
			if type(f) == "function" then
				local label = if lib == "_G" then name else lib .. "." .. name
				t[name] = function(...)
					record(label, ...)
					return f(...)
				end
			end
		end
	end
end
"#;

pub const STEP_BUDGET: u64 = 2_000_000;
// real libraries doing real work (LibDeflate compressing a few kilobytes at every level) need
// more, and their specs are fixed, so nothing generated can wander into the extra room
pub const LIBRARY_STEP_BUDGET: u64 = 40_000_000;
// A net under the step budget for programs that spend their steps in library calls doing
// unbounded work, like table.insert(t, 1, v) on a table that keeps growing. Only programs that
// are broken anyway get near it
pub const WALL_LIMIT: Duration = Duration::from_secs(5);
pub const OUT_OF_MEMORY: &str = "memory limit";

// A library case: the program is an open-source library, and its spec (behind the busted shim)
// runs after it in the same VM, reaching it through require(name). Only the library is ever
// decompiled; the spec is the judge
pub struct Library {
    pub name: String,
    pub spec: String,
    tests: Vec<u8>,
}

const BUSTED: &str = include_str!("busted.luau");

impl Library {
    pub fn new(name: &str, spec: &str) -> Result<Library, String> {
        let source = format!("{BUSTED}\n{spec}\nbusted_run(busted_root, \"\", {{}}, {{}})\n");
        let tests = compile(&source, Profile::new(1, 1)).map_err(|e| format!("{name}'s spec doesn't compile: {e}"))?;
        Ok(Library { name: name.to_string(), spec: spec.to_string(), tests })
    }
}

// Runs bytecode in a fresh VM. print is captured, and anything that differs between two
// faithful compilations of the same program (line numbers in error positions, heap
// addresses) is normalized away.
pub fn run(bytecode: &[u8]) -> Observation {
    run_with(bytecode, None)
}

pub fn run_with(bytecode: &[u8], library: Option<&Library>) -> Observation {
    let lua = Lua::new();
    let _ = lua.set_memory_limit(64 << 20);

    let log = Rc::new(RefCell::new(Vec::new()));
    let steps = Rc::new(Cell::new(0u64));

    let print = {
        let log = log.clone();
        lua.create_function(move |_, args: MultiValue| {
            let parts = args
                .iter()
                .map(|v| match v {
                    // bytes as they are: decompilers get compared on values, not on UTF-8
                    mlua::Value::String(s) => String::from_utf8_lossy(&s.as_bytes()).into_owned(),
                    v => v.to_string().unwrap_or_else(|e| format!("<{e}>")),
                })
                .collect::<Vec<_>>();
            log.borrow_mut().push(parts.join("\t"));
            Ok(())
        })
        .unwrap()
    };
    lua.globals().set("print", print).unwrap();

    let budget = if library.is_some() { LIBRARY_STEP_BUDGET } else { STEP_BUDGET };
    {
        let steps = steps.clone();
        let deadline = Instant::now() + WALL_LIMIT;
        lua.set_interrupt(move |_| {
            let mut n = steps.get() + 1;
            if n % 4096 == 0 && Instant::now() > deadline {
                n = budget + 1;
            }
            steps.set(n);
            if n > budget {
                Err(mlua::Error::runtime("rosetta: step budget exhausted"))
            } else {
                Ok(VmState::Continue)
            }
        });
    }

    let calls = Rc::new(RefCell::new(Calls::default()));
    {
        static TRACER_CODE: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
        let tracer = TRACER_CODE.get_or_init(|| compile(TRACER, Profile::new(1, 1)).expect("tracer compiles"));
        let calls = calls.clone();
        // its own numbering of addresses, like the log's: an error message passed along carries
        // a line number, a table's tostring an address
        let mut addresses = HashMap::new();
        let record = lua
            .create_function_mut(move |_, (name, args): (String, MultiValue)| {
                // line numbers and addresses normalized before anything is cut short
                let args = args.iter().map(|v| clip(&normalize(&shallow_full(v), &mut addresses))).collect::<Vec<_>>();
                calls.borrow_mut().push(format!("{name}({})", args.join(", ")));
                Ok(())
            })
            .unwrap();
        lua.load(&tracer[..]).set_mode(ChunkMode::Binary).set_name("=tracer").call::<()>(record).expect("tracer runs");
    }
    static PRELUDE: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
    let prelude = PRELUDE.get_or_init(|| compile(crate::generator::PRELUDE, Profile::new(1, 1)).expect("prelude compiles"));
    lua.load(&prelude[..]).set_mode(ChunkMode::Binary).set_name("=prelude").exec().expect("prelude runs");


    let mut returned = Vec::new();
    let before = Snapshot::take(&lua);
    let result = match library {
        None => lua.load(bytecode).set_mode(ChunkMode::Binary).set_name("=prog").call::<MultiValue>(()).map(|values| {
            returned = values.into_iter().collect();
        }),
        Some(library) => (|| {
            // require knows only the library, so a library probing for optional modules
            // (inspect looks for string.buffer) gets the error a missing module gives
            let modules = lua.create_table()?;
            let require = {
                let modules = modules.clone();
                lua.create_function(move |_, name: String| match modules.raw_get::<mlua::Value>(name.as_str())? {
                    mlua::Value::Nil => Err(mlua::Error::runtime(format!("module '{name}' not found"))),
                    module => Ok(module),
                })?
            };
            lua.globals().set("require", require)?;
            let module = lua.load(bytecode).set_mode(ChunkMode::Binary).set_name("=prog").call::<mlua::Value>(())?;
            modules.raw_set(library.name.as_str(), module)?;
            lua.load(&library.tests[..]).set_mode(ChunkMode::Binary).set_name("=tests").exec()
        })(),
    };
    let ending = if steps.get() > budget {
        Ending::TimedOut
    } else {
        match result {
            Ok(()) => Ending::Returned,
            Err(e) => Ending::Errored(error_text(&e)),
        }
    };

    let mut addresses = HashMap::new();
    let log = log.borrow().iter().map(|line| normalize(line, &mut addresses)).collect();
    let ending = match ending {
        Ending::Errored(text) => Ending::Errored(normalize(&text, &mut addresses)),
        other => other,
    };
    // a run that hit the budget or ran out of memory is cut off somewhere arbitrary: what it left
    // behind says nothing
    let state = match &ending {
        Ending::TimedOut => Vec::new(),
        Ending::Errored(e) if e == OUT_OF_MEMORY => Vec::new(),
        _ => {
            let mut state = before.changes(&lua);
            state.extend(returned.iter().map(|v| format!("return: {}", describe(v))));
            state
        }
    };
    let calls = calls.borrow().clone();
    Observation { log, ending, state, calls }
}

// a call argument, uncut
fn shallow_full(value: &mlua::Value) -> String {
    match value {
        mlua::Value::String(s) => format!("{:?}", String::from_utf8_lossy(&s.as_bytes())),
        other => shallow(other),
    }
}

fn clip(text: &str) -> String {
    if text.len() > 80 { format!("{}...", &text[..text.floor_char_boundary(80)]) } else { text.to_string() }
}

// a value as one call argument: no deeper than its type for anything that isn't plain data
fn shallow(value: &mlua::Value) -> String {
    match value {
        mlua::Value::String(s) => {
            let text = String::from_utf8_lossy(&s.as_bytes()).into_owned();
            if text.len() > 60 { format!("{:?}...", &text[..text.floor_char_boundary(60)]) } else { format!("{text:?}") }
        }
        mlua::Value::Nil | mlua::Value::Boolean(_) | mlua::Value::Integer(_) | mlua::Value::Number(_) | mlua::Value::Vector(_) => number_text(value),
        other => other.type_name().to_string(),
    }
}

// numbers the way the prelude's describe writes them: nan and -0 spelled out
fn number_text(value: &mlua::Value) -> String {
    match value {
        mlua::Value::Number(n) if n.is_nan() => "nan".into(),
        mlua::Value::Number(n) if *n == 0.0 && n.is_sign_negative() => "-0".into(),
        other => other.to_string().unwrap_or_else(|_| other.type_name().into()),
    }
}

// a value all the way down, tables with their keys sorted and their metatables, a table seen
// before as a back-reference; nothing that depends on where things are in memory
fn describe(value: &mlua::Value) -> String {
    let mut seen = HashMap::new();
    let mut budget = DESCRIBE_BUDGET;
    let mut out = String::new();
    describe_into(value, &mut seen, &mut budget, &mut out);
    out
}

// tables visited per value: past it, the rest is "..."
const DESCRIBE_BUDGET: usize = 2000;

fn describe_into(value: &mlua::Value, seen: &mut HashMap<usize, usize>, budget: &mut usize, out: &mut String) {
    let mlua::Value::Table(table) = value else {
        out.push_str(&match value {
            mlua::Value::String(_) | mlua::Value::Nil | mlua::Value::Boolean(_) | mlua::Value::Integer(_) | mlua::Value::Number(_) | mlua::Value::Vector(_) => shallow(value),
            other => other.type_name().to_string(),
        });
        return;
    };
    let id = table.to_pointer() as usize;
    if let Some(n) = seen.get(&id) {
        out.push_str(&format!("<table {n}>"));
        return;
    }
    if *budget == 0 {
        out.push_str("...");
        return;
    }
    *budget -= 1;
    let n = seen.len() + 1;
    seen.insert(id, n);
    // keys in a fixed order: a table's own walk order depends on how it was built
    let mut entries = table.pairs::<mlua::Value, mlua::Value>().flatten().map(|(k, v)| (describe_key(&k), v)).collect::<Vec<_>>();
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    out.push('{');
    for (i, (key, v)) in entries.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(key);
        out.push('=');
        describe_into(v, seen, budget, out);
    }
    if let Some(meta) = table.metatable() {
        out.push_str(if entries.is_empty() { "<meta>=" } else { ", <meta>=" });
        describe_into(&mlua::Value::Table(meta), seen, budget, out);
    }
    out.push('}');
}

// a key goes no deeper than its type: a table used as a key is the same key whatever is in it
fn describe_key(key: &mlua::Value) -> String {
    match key {
        mlua::Value::Table(_) => "<table>".into(),
        other => shallow(other),
    }
}

// a global's name as the program wrote it
fn global_name(key: &mlua::Value) -> String {
    match key {
        mlua::Value::String(s) => String::from_utf8_lossy(&s.as_bytes()).into_owned(),
        other => format!("[{}]", describe_key(other)),
    }
}

// The globals and the fields of the tables among them, by identity, before the program runs
struct Snapshot {
    globals: HashMap<String, (mlua::Value, Option<HashMap<String, mlua::Value>>)>,
}

impl Snapshot {
    fn take(lua: &Lua) -> Snapshot {
        let mut globals = HashMap::new();
        let env = lua.globals();
        for (key, value) in env.pairs::<mlua::Value, mlua::Value>().flatten() {
            let fields = match &value {
                // _G is the globals themselves, already gone through one by one
                mlua::Value::Table(t) if t.to_pointer() == env.to_pointer() => None,
                mlua::Value::Table(t) => Some(t.pairs::<mlua::Value, mlua::Value>().flatten().map(|(k, v)| (describe_key(&k), v)).collect()),
                _ => None,
            };
            globals.insert(global_name(&key), (value, fields));
        }
        Snapshot { globals }
    }

    // what changed since: a new or replaced global in full, a field changed in a table that was
    // already a global, and a global that's gone
    fn changes(&self, lua: &Lua) -> Vec<String> {
        let same = |a: &mlua::Value, b: &mlua::Value| match (a, b) {
            (mlua::Value::Number(x), mlua::Value::Number(y)) => x.to_bits() == y.to_bits() || (x.is_nan() && y.is_nan()),
            _ => a == b,
        };
        let mut lines = Vec::new();
        let mut present = std::collections::HashSet::new();
        for (key, value) in lua.globals().pairs::<mlua::Value, mlua::Value>().flatten() {
            let name = global_name(&key);
            present.insert(name.clone());
            match self.globals.get(&name) {
                Some((old, fields)) if same(old, &value) => {
                    let (Some(fields), mlua::Value::Table(t)) = (fields, &value) else { continue };
                    let mut now = std::collections::HashSet::new();
                    for (k, v) in t.pairs::<mlua::Value, mlua::Value>().flatten() {
                        let field = describe_key(&k);
                        if !fields.get(&field).is_some_and(|old| same(old, &v)) {
                            lines.push(format!("{name}[{field}] = {}", describe(&v)));
                        }
                        now.insert(field);
                    }
                    lines.extend(fields.keys().filter(|f| !now.contains(*f)).map(|f| format!("{name}[{f}] = nil")));
                }
                _ => lines.push(format!("{name} = {}", describe(&value))),
            }
        }
        lines.extend(self.globals.keys().filter(|k| !present.contains(*k)).map(|k| format!("{k} = nil")));
        lines.sort();
        lines
    }
}

fn error_text(error: &mlua::Error) -> String {
    match error {
        mlua::Error::RuntimeError(text) => text.lines().next().unwrap_or_default().to_string(),
        mlua::Error::CallbackError { cause, .. } => error_text(cause),
        mlua::Error::MemoryError(_) => OUT_OF_MEMORY.to_string(),
        other => other.to_string().lines().next().unwrap_or_default().to_string(),
    }
}

// "prog:12: msg" -> "prog:?: msg" and "0x00007f..." -> "0x#1" numbered by first appearance
// (a map, not a list: a program stuck printing a fresh table every iteration has a million of them)
pub fn normalize(text: &str, addresses: &mut HashMap<String, usize>) -> String {
    let bytes = text.as_bytes();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < bytes.len() {
        if text[i..].starts_with("prog:") {
            let digits = bytes[i + 5..].iter().take_while(|b| b.is_ascii_digit()).count();
            if digits > 0 && bytes.get(i + 5 + digits) == Some(&b':') {
                out.push_str("prog:?:");
                i += 6 + digits;
                continue;
            }
        }
        if text[i..].starts_with("0x") {
            let hex = bytes[i + 2..].iter().take_while(|b| b.is_ascii_hexdigit()).count();
            if hex >= 6 {
                let address = &text[i..i + 2 + hex];
                let next = addresses.len() + 1;
                let index = *addresses.entry(address.to_string()).or_insert(next);
                out.push_str(&format!("0x#{index}"));
                i += 2 + hex;
                continue;
            }
        }
        let c = text[i..].chars().next().unwrap();
        out.push(c);
        i += c.len_utf8();
    }
    out
}
