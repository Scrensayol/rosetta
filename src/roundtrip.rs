use serde::Serialize;

use crate::{
    decompiler::Decompiler,
    luau::{self, Ending, Library, Observation, Profile},
    obfuscate,
    quality::{self, Quality},
    transform,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Pass,
    // the input itself didn't compile or ran out of steps, so it says nothing about the decompiler
    Unusable,
    // the decompiler declined, saying the input is unsupported: no output, but no wrong one
    Refused,
    DecompileFailed,
    RecompileFailed,
    Mismatch,
}

impl Kind {
    pub fn is_failure(self) -> bool {
        matches!(self, Kind::DecompileFailed | Kind::RecompileFailed | Kind::Mismatch)
    }
}

pub struct Verdict {
    pub kind: Kind,
    pub detail: String,
    pub output: Option<String>,
    pub want: Option<Observation>,
    pub got: Option<Observation>,
    // only on a pass
    pub quality: Option<Quality>,
    // only on a pass: whether the output also made the same calls that change things, in the same
    // order with the same arguments, and if not, where they part ways
    pub calls: Option<Result<(), String>>,
}

impl Verdict {
    fn new(kind: Kind, detail: impl Into<String>) -> Verdict {
        Verdict { kind, detail: detail.into(), output: None, want: None, got: None, quality: None, calls: None }
    }
}

// The reference is the original bytecode's behavior, never the source text: the output
// has to recompile, at the same profile, into something that prints the same thing. A library
// case's spec runs after the program both times
pub fn check(source: &str, library: Option<&Library>, profile: Profile, decompiler: &dyn Decompiler) -> Verdict {
    // the obfuscated tier: the original program sets what has to be printed, and the obfuscated
    // one, which is what gets compiled and decompiled, has to print exactly that. If it doesn't,
    // the obfuscator broke it, and it counts for nobody
    let obfuscated;
    let (source, original) = match profile.extra.obfuscate {
        0 => (source, None),
        level => match obfuscate::apply(source, level) {
            Ok(text) => {
                obfuscated = text;
                (obfuscated.as_str(), Some(source))
            }
            Err(e) => return Verdict::new(Kind::Unusable, format!("the obfuscator can't take it: {e}")),
        },
    };
    let bytecode = match luau::compile(source, profile) {
        Ok(b) => b,
        Err(e) => return Verdict::new(Kind::Unusable, format!("input doesn't compile: {e}")),
    };
    let want = luau::run_with(&bytecode, library);
    if want.ending == Ending::TimedOut {
        return Verdict::new(Kind::Unusable, "input exhausts the step budget");
    }
    if let Some(original) = original {
        let reference = match luau::compile(original, profile) {
            Ok(b) => luau::run_with(&b, library),
            Err(e) => return Verdict::new(Kind::Unusable, format!("input doesn't compile: {e}")),
        };
        if reference != want {
            return Verdict::new(Kind::Unusable, format!("obfuscating changed what the program does: {}", first_difference(&reference, &want)));
        }
    }
    // where an allocation finally fails depends on temporaries no decompiler has to preserve
    if want.ending == Ending::Errored(luau::OUT_OF_MEMORY.into()) {
        return Verdict::new(Kind::Unusable, "input exhausts the memory limit");
    }
    // the transform tier: the decompiler gets rewritten bytecode, which has to do exactly what
    // the compiler's did. If it doesn't, that's Rosetta's bug, and the program counts for nobody
    let bytecode = match profile.extra.transform {
        0 => bytecode,
        level => match transform::apply(&bytecode, level) {
            Ok(t) => {
                let seen = luau::run_with(&t, library);
                if seen != want {
                    return Verdict::new(Kind::Unusable, format!("the transform changed what the program does: {}", first_difference(&want, &seen)));
                }
                t
            }
            Err(e) => return Verdict::new(Kind::Unusable, format!("the transform can't read the bytecode: {e}")),
        },
    };

    let (output, cpu) = decompiler.decompile(&bytecode);
    let output = match output {
        Ok(o) => o,
        Err(e) => {
            let kind = if e.starts_with("unsupported") { Kind::Refused } else { Kind::DecompileFailed };
            return Verdict { want: Some(want), ..Verdict::new(kind, e) };
        }
    };
    let recompiled = match luau::compile(&output, profile) {
        Ok(b) => b,
        Err(e) => {
            return Verdict { output: Some(output), want: Some(want), ..Verdict::new(Kind::RecompileFailed, e) };
        }
    };
    let got = luau::run_with(&recompiled, library);
    let (kind, detail) = if got == want { (Kind::Pass, String::new()) } else { (Kind::Mismatch, first_difference(&want, &got)) };
    let quality = (kind == Kind::Pass).then(|| quality::measure(source, &output, profile.debug >= 2, cpu));
    // calls that differ from run to run of the original itself (an order that comes from where
    // tables sit in memory) say nothing about the decompiler: those passes are left out of it
    let calls = (kind == Kind::Pass)
        .then(|| match want.calls.first_difference(&got.calls) {
            None => Some(Ok(())),
            Some(why) => (luau::run_with(&bytecode, library).calls == want.calls).then_some(Err(why)),
        })
        .flatten();
    Verdict { kind, detail, output: Some(output), want: Some(want), got: Some(got), quality, calls }
}

fn first_difference(want: &Observation, got: &Observation) -> String {
    for (i, (w, g)) in want.log.iter().zip(&got.log).enumerate() {
        if w != g {
            return format!("line {}: want `{w}` got `{g}`", i + 1);
        }
    }
    if want.log.len() != got.log.len() {
        return format!("printed {} lines, want {}", got.log.len(), want.log.len());
    }
    if want.ending != got.ending {
        return format!("ended {:?}, want {:?}", got.ending, want.ending);
    }
    // printed the same and ended the same, but left something else behind
    let clip = |s: &str| if s.len() > 160 { format!("{}...", &s[..s.floor_char_boundary(160)]) } else { s.to_string() };
    match want.state.iter().zip(&got.state).find(|(w, g)| w != g) {
        Some((w, g)) => format!("left behind `{}`, want `{}`", clip(g), clip(w)),
        None if want.state.len() > got.state.len() => format!("didn't leave behind `{}`", clip(&want.state[got.state.len()])),
        None => format!("also left behind `{}`", clip(got.state.get(want.state.len()).map_or("", |s| s))),
    }
}
