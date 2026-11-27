// The obfuscated tier: a program is obfuscated as source before it's compiled, the way scripts
// in the wild are, and the decompiler has to give back something that still does what the
// obfuscated program did. Nobody has to deobfuscate anything: the obfuscator's VM, decoders and
// flattened control flow all have to survive the round trip.
//
// Two steps, both in-process so a run needs nothing installed:
// - darklua (as a library) lowers Luau syntax the obfuscator can't read to Lua 5.1: types,
//   interpolated strings, compound assignment, if-expressions, continue and floor division.
//   Generalized iteration (`for k, v in t do`) has no darklua rule, so `lower_iteration` adds one.
// - Prometheus (vendor/prometheus, pure Lua) runs inside Rosetta's own Luau VM at one of three
//   levels, from a fixed seed. Based on Prometheus by Elias Oelschner,
//   https://github.com/prometheus-lua/Prometheus
//
// Prometheus's anti-tamper step is left out: it trips on any change to the program's line layout,
// which every decompiler makes, so it would fail all of them for nothing. Strong's number
// representation mutation (hex, binary and scientific literals) is off too: under Luau it breaks
// nearly every program it touches, so the strong level would have had almost nothing left in it.

use std::{
    collections::HashMap,
    fs,
    path::Path,
    sync::{Arc, Mutex, OnceLock},
};

use darklua_core::{
    Parser, Resources,
    generator::{LuaGenerator, ReadableLuaGenerator},
    nodes::{Arguments, Block, Expression, FunctionCall, GenericForStatement, TupleArguments},
    process::{DefaultVisitor, NodeProcessor, NodeVisitor},
    rules::{ContextBuilder, Rule},
};
use mlua::{Lua, LuaOptions, StdLib};

// Bump when the lowering, the vendored Prometheus or the settings below change what a program
// obfuscates to
pub const VERSION: &str = "obf-1 (prometheus a98c501, darklua 0.19.0)";
pub const LEVELS: u8 = 3;
const SEED: u32 = 12345;
// Prometheus's presets, weakest first
const PRESETS: [&str; 3] = ["Weak", "Medium", "Strong"];

const LOWERING: [&str; 6] = [
    "remove_types",
    "remove_interpolated_string",
    "remove_compound_assignment",
    "remove_if_expression",
    "remove_continue",
    "remove_floor_division",
];

// Luau's generalized iteration, for a Lua 5.1 that only has iterator functions: a function passes
// through, an __iter metamethod is called, a callable table is called the way 5.1 would, and a
// plain table is walked with next (the order Luau walks it in too: array part, then hash part)
const ITER_HELPER: &str = "local __rosetta_iter = function(f, s, c)
    if type(f) == \"function\" then return f, s, c end
    local mt = getmetatable(f)
    if type(mt) == \"table\" and mt.__iter then return mt.__iter(f) end
    if type(mt) == \"table\" and mt.__call then return f, s, c end
    if type(f) == \"table\" then return next, f, nil end
    return f, s, c
end
";

struct WrapIteration;

impl NodeProcessor for WrapIteration {
    fn process_generic_for_statement(&mut self, statement: &mut GenericForStatement) {
        let values = std::mem::take(statement.mutate_expressions());
        let call = FunctionCall::from_name("__rosetta_iter").with_arguments(Arguments::Tuple(TupleArguments::new(values)));
        statement.mutate_expressions().push(Expression::from(call));
    }
}

// Luau source to Lua 5.1 source Prometheus can parse
pub fn lower(source: &str) -> Result<String, String> {
    let mut block: Block = Parser::default().parse(source).map_err(|e| format!("darklua can't parse it: {e:?}"))?;
    let resources = Resources::from_memory();
    let context = ContextBuilder::new("program.luau", &resources, source).build();
    for name in LOWERING {
        let rule: Box<dyn Rule> = name.parse()?;
        rule.process(&mut block, &context).map_err(|e| format!("{name}: {e}"))?;
    }
    DefaultVisitor::visit_block(&mut block, &mut WrapIteration);
    let mut generator = ReadableLuaGenerator::default();
    generator.write_block(&block);
    Ok(format!("{ITER_HELPER}{}", generator.into_string()))
}

fn vendor() -> &'static Path {
    static DIR: OnceLock<std::path::PathBuf> = OnceLock::new();
    DIR.get_or_init(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("vendor/prometheus"))
}

// Sets up a VM with Prometheus loaded, and returns its obfuscate(code, level) function
fn prometheus(lua: &Lua) -> mlua::Result<mlua::Function> {
    let globals = lua.globals();
    // what Prometheus reads from a stand-alone Lua that Luau doesn't have
    globals.set("arg", lua.create_table()?)?;
    let require = lua.create_function(|lua, name: String| {
        let loaded: mlua::Table = lua.named_registry_value("rosetta_loaded")?;
        if let Some(module) = loaded.get::<Option<mlua::Value>>(name.as_str())? {
            return Ok(module);
        }
        let path = vendor().join(format!("{}.lua", name.replace('.', "/")));
        let code = fs::read_to_string(&path).map_err(|e| mlua::Error::runtime(format!("{}: {e}", path.display())))?;
        let module: mlua::Value = lua.load(code).set_name(format!("={name}")).call(())?;
        let module = if module.is_nil() { mlua::Value::Boolean(true) } else { module };
        loaded.set(name.as_str(), module.clone())?;
        Ok(module)
    })?;
    lua.set_named_registry_value("rosetta_loaded", lua.create_table()?)?;
    globals.set("require", require)?;
    lua.load(BOOT).set_name("=rosetta_boot").call(())
}

const BOOT: &str = r#"
-- Prometheus asks math.random for ranges past 2^31, which Luau refuses: the fix its CLI applies
if not pcall(function() return math.random(1, 2^40) end) then
    local random = math.random
    math.random = function(a, b)
        if not a then return random() end
        if not b then return math.random(1, a) end
        if a > b then a, b = b, a end
        if b - a > 2^31 - 1 then return math.floor(random() * (b - a) + a) end
        return random(a, b)
    end
end
_G.newproxy = _G.newproxy or function(arg)
    if arg then return setmetatable({}, {}) end
    return {}
end
local logger = require("logger")
logger.logLevel = logger.LogLevel.Error
logger.errorCallback = function(...) error(table.concat({...}, " "), 0) end
logger.warnCallback = function() end
local Pipeline = require("prometheus.pipeline")
local presets = require("presets")
return function(code, preset, seed)
    local config = table.clone(presets[preset])
    config.LuaVersion = "LuaU"
    config.Seed = seed
    config.PrettyPrint = false
    local steps = {}
    for _, step in ipairs(config.Steps) do
        if step.Name == "NumbersToExpressions" then
            step = table.clone(step)
            step.Settings = table.clone(step.Settings)
            step.Settings.NumberRepresentationMutation = false
        end
        if step.Name ~= "AntiTamper" then table.insert(steps, step) end
    end
    config.Steps = steps
    return Pipeline:fromConfig(config):apply(code, "program")
end
"#;

// A program obfuscated at a level (1 to LEVELS). The same source and level always give the same
// text, so it's worked out once per run
pub fn apply(source: &str, level: u8) -> Result<String, String> {
    static CACHE: OnceLock<Mutex<HashMap<(String, u8), Arc<Result<String, String>>>>> = OnceLock::new();
    let key = (source.to_string(), level);
    if let Some(hit) = CACHE.get_or_init(Default::default).lock().unwrap().get(&key) {
        return (**hit).clone();
    }
    let result = obfuscate(source, level);
    let mut cache = CACHE.get().unwrap().lock().unwrap();
    // a run needs a few hundred; reducing a failure tries thousands of programs once each
    if cache.len() >= 1000 {
        cache.clear();
    }
    cache.insert(key, Arc::new(result.clone()));
    result
}

// darklua's parser and Prometheus both recurse deeply on big programs: more than a rayon worker's
// stack holds, so the work gets a thread of its own
fn obfuscate(source: &str, level: u8) -> Result<String, String> {
    let source = source.to_string();
    std::thread::Builder::new()
        .stack_size(256 << 20)
        .spawn(move || obfuscate_here(&source, level))
        .map_err(|e| e.to_string())?
        .join()
        .unwrap_or_else(|_| Err("the obfuscator panicked".into()))
}

fn obfuscate_here(source: &str, level: u8) -> Result<String, String> {
    let preset = *PRESETS.get(usize::from(level).wrapping_sub(1)).ok_or("no such obfuscation level")?;
    let lowered = lower(source)?;
    // a fresh VM each time: Prometheus keeps state in its modules, and the seed has to start clean
    let lua = Lua::new_with(StdLib::ALL_SAFE, LuaOptions::default()).map_err(|e| e.to_string())?;
    let obfuscate = prometheus(&lua).map_err(|e| format!("loading Prometheus: {e}"))?;
    obfuscate.call::<String>((lowered, preset, SEED)).map_err(|e| format!("Prometheus: {}", e.to_string().lines().next().unwrap_or_default()))
}
