// How a passing decompile reads next to the source it came from. Only passes are measured: a
// wrong decompile has no readability worth scoring. Every number compares the output with the
// original program, so a decompiler is never graded on style choices the original didn't make.

use std::{collections::BTreeSet, time::Duration};

use serde::Serialize;
use similar::{Algorithm, capture_diff_slices_deadline, get_diff_ratio};

use crate::lex::{self, Kind, Token};

#[derive(Clone, Debug)]
pub struct Quality {
    // 0..1, how much of the original's token sequence (names, numbers and strings stood in for
    // by their kind) the output keeps in order
    pub similarity: f64,
    pub tokens: (usize, usize),
    pub locals: (usize, usize),
    pub depth: (usize, usize),
    pub endless: (usize, usize),
    // original names the output declares, out of the original's: only at debug level 2,
    // where the bytecode carries them
    pub names: Option<(usize, usize)>,
    pub cpu: Duration,
}

fn shape<'a>(tokens: &[Token<'a>]) -> Vec<&'a str> {
    tokens
        .iter()
        .map(|t| match t.kind {
            Kind::Name => "n",
            Kind::Number => "0",
            Kind::Str => "s",
            Kind::Keyword | Kind::Op => t.text,
        })
        .collect()
}

pub fn measure(original: &str, output: &str, debug_names: bool, cpu: Duration) -> Quality {
    let (a, b) = (lex::tokenize(original), lex::tokenize(output));
    let (sa, sb) = (shape(&a), shape(&b));
    let deadline = std::time::Instant::now() + Duration::from_secs(2);
    let ops = capture_diff_slices_deadline(Algorithm::Myers, &sa, &sb, Some(deadline));
    let similarity = get_diff_ratio(&ops, sa.len(), sb.len()) as f64;
    let (da, db) = (lex::declared(&a), lex::declared(&b));
    let names = debug_names.then(|| {
        let want = da.iter().map(|t| t.text).collect::<BTreeSet<_>>();
        let have = db.iter().map(|t| t.text).collect::<BTreeSet<_>>();
        (want.intersection(&have).count(), want.len())
    });
    Quality {
        similarity,
        tokens: (b.len(), a.len()),
        locals: (db.len(), da.len()),
        depth: (lex::nesting(&b).max, lex::nesting(&a).max),
        endless: (lex::endless_loops(&b), lex::endless_loops(&a)),
        names,
        cpu,
    }
}

// One decompiler over a set of round trips. Ratios are output over original, averaged
// geometrically so a big program doesn't drown the small ones
#[derive(Serialize, Clone, Debug)]
pub struct Summary {
    pub round_trips: usize,
    // mean, 0..100
    pub similarity: f64,
    pub size: f64,
    pub locals: f64,
    // mean of output depth minus original depth
    pub nesting: f64,
    // endless loops the output adds over the original's, in total
    pub endless: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<f64>,
    pub cpu_ms_median: f64,
    pub cpu_ms_total: f64,
}

impl Summary {
    pub fn of(qualities: &[&Quality]) -> Summary {
        let n = qualities.len().max(1) as f64;
        let geo = |f: &dyn Fn(&Quality) -> (usize, usize)| {
            (qualities.iter().map(|q| {
                let (out, orig) = f(q);
                ((out + 1) as f64 / (orig + 1) as f64).ln()
            }).sum::<f64>() / n).exp()
        };
        let (found, total) = qualities.iter().filter_map(|q| q.names).fold((0, 0), |(f, t), (a, b)| (f + a, t + b));
        let mut cpu = qualities.iter().map(|q| q.cpu.as_secs_f64() * 1000.0).collect::<Vec<_>>();
        cpu.sort_by(f64::total_cmp);
        Summary {
            round_trips: qualities.len(),
            similarity: qualities.iter().map(|q| q.similarity).sum::<f64>() / n * 100.0,
            size: geo(&|q| q.tokens),
            locals: geo(&|q| q.locals),
            nesting: qualities.iter().map(|q| q.depth.0 as f64 - q.depth.1 as f64).sum::<f64>() / n,
            endless: qualities.iter().map(|q| q.endless.0.saturating_sub(q.endless.1)).sum(),
            names: (total > 0).then(|| found as f64 / total as f64 * 100.0),
            cpu_ms_median: cpu.get(cpu.len() / 2).copied().unwrap_or(0.0),
            cpu_ms_total: cpu.iter().sum(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_output_scores_perfectly() {
        let src = "local function add(a, b)\n\treturn a + b\nend\nprint(add(1, 2))\n";
        let q = measure(src, src, true, Duration::ZERO);
        assert_eq!(q.similarity, 1.0);
        assert_eq!(q.names, Some((3, 3)));
        assert_eq!(q.depth, (1, 1));
    }

    #[test]
    fn renamed_output_keeps_shape_but_loses_names() {
        let q = measure("local x = 1 print(x)", "local v1 = 1 print(v1)", true, Duration::ZERO);
        assert_eq!(q.similarity, 1.0);
        assert_eq!(q.names, Some((0, 1)));
    }
}
