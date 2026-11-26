// The failure gallery: for every round trip a decompiler got wrong (wrong output, output that
// doesn't recompile, a crash; not refusals, whose one line already says why), what it was given
// and what it gave back. bench --gallery writes it beside the result, and `rosetta gallery`
// rebuilds it for a result that already exists by rerunning only the failed round trips. The
// website loads it when a failed test is opened. It's a separate file so the result itself (what
// gets hashed and signed) stays the same, and so the leaderboard never downloads it.

use std::collections::BTreeMap;

use serde::Serialize;

use crate::{
    decompiler::Decompiler,
    generator::{self, Grammar},
    luau::{Ending, Library, Observation, Profile},
    mutate, reduce,
    roundtrip::{self, Kind, Verdict},
};

// entries kept per section, in test order: a decompiler that fails thousands of programs shows
// what goes wrong without a file nobody can download
pub const PER_SECTION: usize = 150;
// sources and outputs are cut here
const CLIP: usize = 16 * 1024;
const LINE_CLIP: usize = 300;
// reduction attempts per failure, as fuzz --reduce
const REDUCE_BUDGET: usize = 400;

#[derive(Serialize)]
pub struct Entry {
    pub test: String,
    pub section: &'static str,
    pub profile: String,
    pub kind: Kind,
    pub detail: String,
    // the program as the suite has it (left out for libraries, which link to their directory, and
    // when there's a reduced one)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    // a smaller program that fails the same way; output and compare are then its own
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduced: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compare: Option<Compare>,
    // set by `rosetta gallery` when the rerun didn't end the way the result says it did
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recorded: Option<Kind>,
}

// what the original printed and what the recompiled output printed, around the first line where
// they part
#[derive(Serialize)]
pub struct Compare {
    // the line number (from 1) of want[0] and got[0]
    pub from: usize,
    // the line number (from 1) of the first difference, or 0 when only the endings differ
    pub first: usize,
    pub want: Vec<String>,
    pub got: Vec<String>,
    pub want_lines: usize,
    pub got_lines: usize,
    pub want_ending: String,
    pub got_ending: String,
}

pub fn clip(text: &str, limit: usize) -> String {
    if text.len() <= limit {
        return text.to_string();
    }
    let cut = text.floor_char_boundary(limit);
    format!("{}\n-- [rosetta: cut here, {} more bytes]", &text[..cut], text.len() - cut)
}

fn ending(e: &Ending) -> String {
    match e {
        Ending::Returned => "returned".into(),
        Ending::Errored(m) => format!("error: {}", clip(m, LINE_CLIP)),
        Ending::TimedOut => "ran out of steps".into(),
    }
}

fn compare(want: &Observation, got: &Observation) -> Compare {
    let first = want.log.iter().zip(&got.log).position(|(w, g)| w != g).or_else(|| (want.log.len() != got.log.len()).then(|| want.log.len().min(got.log.len())));
    let at = first.unwrap_or(want.log.len().min(got.log.len()));
    let from = at.saturating_sub(3);
    let window = |log: &[String]| log.iter().skip(from).take(at - from + 8).map(|l| clip(l, LINE_CLIP)).collect::<Vec<_>>();
    Compare {
        from: from + 1,
        first: first.map_or(0, |i| i + 1),
        want: window(&want.log),
        got: window(&got.log),
        want_lines: want.log.len(),
        got_lines: got.log.len(),
        want_ending: ending(&want.ending),
        got_ending: ending(&got.ending),
    }
}

// One gallery entry for a failed round trip. With `reduce`, generated programs, mutants and cases
// are first shrunk to a smaller program the decompiler fails the same way (each attempt is a
// decompile, so this is for local decompilers, not rate-limited web APIs)
pub fn entry(
    section: &'static str,
    name: &str,
    source: &str,
    library: Option<&Library>,
    profile: Profile,
    decompiler: &dyn Decompiler,
    verdict: Verdict,
    reduce: bool,
) -> Entry {
    let kind = verdict.kind;
    let mut reduced = None;
    if reduce && library.is_none() {
        let same = |text: &str| roundtrip::check(text, None, profile, decompiler).kind == kind;
        let smaller = match name.strip_prefix("seed").and_then(|s| s.parse::<u64>().ok()) {
            Some(seed) if section == "generated" => {
                let (program, _) = reduce::reduce(&generator::generate(seed, Grammar::Full), |p| same(&p.render()), REDUCE_BUDGET);
                program.render()
            }
            _ => mutate::reduce(source, same, REDUCE_BUDGET),
        };
        if smaller.len() < source.len() {
            let v = roundtrip::check(&smaller, None, profile, decompiler);
            if v.kind == kind {
                reduced = Some((smaller, v));
            }
        }
    }
    let shown = reduced.as_ref().map_or(&verdict, |(_, v)| v);
    let compare = match (&shown.want, &shown.got) {
        (Some(w), Some(g)) if kind == Kind::Mismatch => Some(compare(w, g)),
        _ => None,
    };
    Entry {
        test: name.to_string(),
        section,
        profile: profile.to_string(),
        kind,
        detail: clip(&shown.detail, 4096),
        // a reduced program stands in for the original, which the suite can always rebuild
        source: (library.is_none() && reduced.is_none()).then(|| clip(source, CLIP)),
        output: shown.output.as_deref().map(|o| clip(o, CLIP)),
        compare,
        reduced: reduced.map(|(s, _)| s),
        recorded: None,
    }
}

// Keeps the first PER_SECTION failures of each section (in the order given), counting the rest,
// so only the ones that make it into the file are rerun or reduced
pub fn cap<T>(failures: Vec<(&'static str, T)>) -> (Vec<T>, BTreeMap<&'static str, usize>) {
    let mut kept = BTreeMap::<&'static str, usize>::new();
    let mut omitted = BTreeMap::new();
    let mut out = Vec::new();
    for (section, f) in failures {
        let n = kept.entry(section).or_default();
        if *n < PER_SECTION {
            *n += 1;
            out.push(f);
        } else {
            *omitted.entry(section).or_default() += 1;
        }
    }
    (out, omitted)
}
