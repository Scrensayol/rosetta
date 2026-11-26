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
                _ => return None,
            }
            text = &rest[end..];
        }
        text.is_empty().then_some(extra)
    }
}

impl fmt::Display for Extra {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (flag, value) in [("c", self.coverage as u32), ("t", self.type_info as u32), ("m", self.mutable as u32), ("d", self.disabled as u32), ("x", self.transform as u32)] {
            if value != 0 {
                write!(f, "+{flag}{value}")?;
            }
        }
        Ok(())
    }
}

impl Profile {
    pub const ALL: [Profile; 6] = [
        Profile { opt: 0, debug: 1, extra: Extra { coverage: 0, type_info: 0, mutable: 0, disabled: 0, transform: 0 } },
        Profile { opt: 1, debug: 1, extra: Extra { coverage: 0, type_info: 0, mutable: 0, disabled: 0, transform: 0 } },
        Profile { opt: 2, debug: 1, extra: Extra { coverage: 0, type_info: 0, mutable: 0, disabled: 0, transform: 0 } },
        Profile { opt: 0, debug: 2, extra: Extra { coverage: 0, type_info: 0, mutable: 0, disabled: 0, transform: 0 } },
        Profile { opt: 1, debug: 2, extra: Extra { coverage: 0, type_info: 0, mutable: 0, disabled: 0, transform: 0 } },
        Profile { opt: 2, debug: 2, extra: Extra { coverage: 0, type_info: 0, mutable: 0, disabled: 0, transform: 0 } },
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Observation {
    pub log: Vec<String>,
    pub ending: Ending,
}

impl Observation {
    pub fn render(&self) -> String {
        let mut text = self.log.join("\n");
        text.push_str(&format!("\n== {:?}\n", self.ending));
        text
    }
}

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

    static PRELUDE: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
    let prelude = PRELUDE.get_or_init(|| compile(crate::generator::PRELUDE, Profile::new(1, 1)).expect("prelude compiles"));
    lua.load(&prelude[..]).set_mode(ChunkMode::Binary).set_name("=prelude").exec().expect("prelude runs");

    let result = match library {
        None => lua.load(bytecode).set_mode(ChunkMode::Binary).set_name("=prog").exec(),
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
    Observation { log, ending }
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
