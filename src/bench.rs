// The all-in-one run: the hand-written cases, a fixed range of generated programs and a fixed set
// of mutants, every one at every profile, written as one JSON result per decompiler. Everything
// that decides which programs run is pinned under the suite name and fingerprinted, so two
// results with the same suite hash were graded on exactly the same programs. The website reads
// these files.

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use rayon::prelude::*;
use serde::Serialize;

use crate::{
    decompiler::{Decompiler, External},
    generator::{self, Grammar},
    luau::{Extra, Profile},
    mutate,
    quality::Summary,
    roundtrip::{self, Kind},
};

// Bump when anything below changes which programs run or how they're graded
pub const SUITE: &str = "rosetta-2";
pub const LUAU: &str = "0.747";
const MUTATION_SEED: u64 = 1;
const FUZZ_START: u64 = 1000;
// generated programs and mutants in the full tier
const FULL: usize = 1000;

#[derive(Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Tier {
    /// The cases, plus the 50 generated programs and the 50 mutants of the full tier whose bytecode
    /// does the most: about 750 round trips
    Quick,
    /// The cases, 1000 generated programs and 1000 mutants: about 12,000 round trips
    Full,
    /// Quick's programs with their bytecode rewritten before the decompiler sees it (see
    /// transform.rs), each profile at one of the three levels: robustness, scored on its own
    HardQuick,
    /// Full's programs, rewritten the same way: about 12,000 round trips
    HardFull,
}

impl Tier {
    fn name(self) -> &'static str {
        match self {
            Tier::Quick => "quick",
            Tier::Full => "full",
            Tier::HardQuick => "hard-quick",
            Tier::HardFull => "hard-full",
        }
    }

    // how many generated programs and mutants the tier keeps
    fn size(self) -> usize {
        match self {
            Tier::Quick | Tier::HardQuick => 50,
            Tier::Full | Tier::HardFull => 1000,
        }
    }
}

// the profiles a tier runs every program at. Hard pairs each with a transform level, so every
// optimization level meets two levels and every level two optimization levels
pub fn profiles(tier: Tier) -> Vec<Profile> {
    match tier {
        Tier::Quick | Tier::Full => Profile::ALL.to_vec(),
        Tier::HardQuick | Tier::HardFull => Profile::ALL
            .iter()
            .zip([1, 2, 3, 2, 3, 1])
            .map(|(p, x)| Profile { extra: Extra { transform: x, ..p.extra }, ..*p })
            .collect(),
    }
}

struct Program {
    name: String,
    about: Option<String>,
    source: String,
    extra: Extra,
}

struct Section {
    id: &'static str,
    title: &'static str,
    about: String,
    programs: Vec<Program>,
}

#[derive(Serialize, Clone, Copy, Default)]
struct Score {
    passed: usize,
    total: usize,
}

#[derive(Serialize)]
struct SuiteInfo {
    name: &'static str,
    tier: &'static str,
    hash: String,
}

#[derive(Serialize)]
struct DecompilerInfo {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
}

#[derive(Serialize)]
struct TestOut {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    about: Option<String>,
    // one letter per profile, in the order of `profiles`: see LETTERS
    results: String,
    // profile -> the first line of why it didn't pass
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    details: BTreeMap<String, String>,
}

#[derive(Serialize)]
struct SectionOut {
    id: &'static str,
    title: &'static str,
    about: String,
    score: Score,
    tests: Vec<TestOut>,
}

#[derive(Serialize)]
struct BenchResult {
    format: &'static str,
    schema: u32,
    rosetta: &'static str,
    luau: &'static str,
    suite: SuiteInfo,
    decompiler: DecompilerInfo,
    created: String,
    seconds: f64,
    threads: usize,
    profiles: Vec<String>,
    // letter -> verdict, so the file explains itself
    letters: BTreeMap<char, Kind>,
    score: Score,
    totals: BTreeMap<Kind, usize>,
    sections: Vec<SectionOut>,
    // readability over this decompiler's own passes (a run with several decompilers in the README
    // measures it over the passes they share instead)
    #[serde(skip_serializing_if = "Option::is_none")]
    quality: Option<Summary>,
}

const LETTERS: [(char, Kind); 6] = [
    ('p', Kind::Pass),
    ('w', Kind::Mismatch),
    ('c', Kind::RecompileFailed),
    ('f', Kind::DecompileFailed),
    ('r', Kind::Refused),
    ('u', Kind::Unusable),
];

fn letter(kind: Kind) -> char {
    LETTERS.iter().find(|(_, k)| *k == kind).unwrap().0
}

// the comment block a case starts with, as one line
fn about(source: &str) -> Option<String> {
    let text = source
        .lines()
        .skip_while(|l| l.starts_with("--!"))
        .map_while(|l| l.strip_prefix("--"))
        .map(str::trim)
        .collect::<Vec<_>>()
        .join(" ");
    (!text.is_empty()).then_some(text)
}

fn sections(tier: Tier) -> Vec<Section> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("cases");
    let mut files = Vec::new();
    crate::collect_luau(&dir, &mut files);
    files.sort();
    let cases = files
        .iter()
        .map(|p| {
            let source = fs::read_to_string(p).expect("read case");
            Program {
                name: p.file_stem().unwrap().to_string_lossy().into_owned(),
                about: about(&source),
                extra: crate::options_directive(&source).unwrap_or_default(),
                source,
            }
        })
        .collect::<Vec<_>>();

    // quick isn't the first 50 of each: it's the 50 of full's with the most bytecode features,
    // picked without looking at any decompiler, so it stays a subset of full and still separates
    // decompilers that pass all the easy programs
    let seeds = FUZZ_START..FUZZ_START + FULL as u64;
    let generated = seeds
        .clone()
        .into_par_iter()
        .map(|seed| Program {
            name: format!("seed{seed}"),
            about: None,
            source: generator::generate(seed, Grammar::Full).render(),
            extra: Extra::default(),
        })
        .collect::<Vec<_>>();

    let pool = cases.iter().map(|c| (c.name.clone(), c.source.clone())).collect::<Vec<_>>();
    let pool_hash = mutate::pool_hash(&pool);
    let (mutants, _) = mutate::evolve(pool, MUTATION_SEED, FULL, false);
    let mutants = mutants
        .into_iter()
        .map(|m| {
            let source = m.file_text();
            Program { name: m.name.clone(), about: Some(format!("{} of {}: {}", m.name, m.origin, m.ops.join(", "))), extra: m.extra, source }
        })
        .collect::<Vec<_>>();

    let generated = hardest(generated, tier.size());
    let mutants = hardest(mutants, tier.size());
    let picked = if tier.size() == FULL { String::new() } else { format!(", the {} with the most bytecode features", tier.size()) };

    vec![
        Section {
            id: "cases",
            title: "hand-written cases",
            about: format!("the {} programs in cases/, each a shape that has tripped decompilers up", cases.len()),
            programs: cases,
        },
        Section {
            id: "generated",
            title: "generated programs",
            about: format!("fuzz --seeds {}..{}{picked} (generator {})", seeds.start, seeds.end, generator::GENERATOR_VERSION),
            programs: generated,
        },
        Section {
            id: "mutants",
            title: "mutants",
            about: format!(
                "mutate --count {FULL} --seed {MUTATION_SEED} over the cases{picked} (pool {pool_hash}, mutator {})",
                mutate::MUTATOR_VERSION
            ),
            programs: mutants,
        },
    ]
}

// the n programs with the most bytecode features, kept in their original order
fn hardest(programs: Vec<Program>, n: usize) -> Vec<Program> {
    if programs.len() <= n {
        return programs;
    }
    let counts = programs.par_iter().map(|p| mutate::feature_count(&p.source, p.extra)).collect::<Vec<_>>();
    let mut order = (0..programs.len()).collect::<Vec<_>>();
    order.sort_by(|&a, &b| counts[b].cmp(&counts[a]).then(a.cmp(&b)));
    let mut keep = vec![false; programs.len()];
    for &i in &order[..n] {
        keep[i] = true;
    }
    programs.into_iter().zip(keep).filter_map(|(p, k)| k.then_some(p)).collect()
}

// FNV-1a over everything that decides what runs. A fingerprint, not a signature
fn fingerprint(tier: Tier, sections: &[Section]) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut feed = |s: &str| {
        for b in s.bytes().chain([0]) {
            h = (h ^ b as u64).wrapping_mul(0x100_0000_01b3);
        }
    };
    feed(SUITE);
    feed(tier.name());
    feed(LUAU);
    if matches!(tier, Tier::HardQuick | Tier::HardFull) {
        feed(crate::transform::VERSION);
    }
    for p in profiles(tier) {
        feed(&p.to_string());
    }
    for s in sections {
        feed(s.id);
        for p in &s.programs {
            feed(&p.name);
            feed(&p.source);
            feed(&p.extra.to_string());
        }
    }
    format!("{h:016x}")
}

// Checks the transform tier against a tier's programs without any decompiler: that reading and
// writing every program's bytecode gives back the same bytes, and that the bytecode rewritten at
// every level prints exactly what the compiler's did, at every profile
pub fn transform_check(tier: Tier, show: usize, only: Option<&str>) -> ExitCode {
    let started = Instant::now();
    let sections = sections(tier);
    let programs = sections.iter().flat_map(|s| &s.programs).filter(|p| only.is_none_or(|o| p.name == o)).collect::<Vec<_>>();
    let jobs = programs.iter().flat_map(|p| Profile::ALL.into_iter().map(move |base| (*p, base))).collect::<Vec<_>>();
    #[derive(Default)]
    struct Tally {
        runs: usize,
        unusable: usize,
        identity: Vec<String>,
        changed: Vec<String>,
        unreadable: Vec<String>,
        functions: usize,
        rewritten: usize,
    }
    let tallies = jobs
        .par_iter()
        .map(|&(program, base)| {
            let mut t = Tally::default();
            let profile = Profile { extra: program.extra, ..base };
            let Ok(bytecode) = crate::luau::compile(&program.source, profile) else {
                t.unusable += 1;
                return t;
            };
            match crate::chunk::Chunk::parse(&bytecode) {
                Ok(c) if c.write() == bytecode => {}
                Ok(_) => t.identity.push(format!("{} {profile}: written back differently", program.name)),
                Err(e) => t.identity.push(format!("{} {profile}: {e}", program.name)),
            }
            let want = crate::luau::run(&bytecode);
            if want.ending == crate::luau::Ending::TimedOut {
                t.unusable += 1;
                return t;
            }
            for level in 1..=crate::transform::LEVELS {
                t.runs += 1;
                if std::env::var_os("ROSETTA_TRACE_TRANSFORM").is_some() {
                    eprintln!("run {} {profile}+x{level}", program.name);
                }
                match crate::transform::apply_counted(&bytecode, level) {
                    Ok((rewritten, done, total)) => {
                        t.functions += total;
                        t.rewritten += done;
                        if let Err(e) = crate::transform::verify(&rewritten) {
                            t.unreadable.push(format!("{} {profile}+x{level}: rewritten badly: {e}", program.name));
                            continue;
                        }
                        let seen = crate::luau::run(&rewritten);
                        if seen != want {
                            t.changed.push(format!("{} {profile}+x{level}: {:?} vs {:?}", program.name, seen.log.len(), want.log.len()));
                        }
                    }
                    Err(e) => t.unreadable.push(format!("{} {profile}+x{level}: {e}", program.name)),
                }
            }
            t
        })
        .reduce(Tally::default, |mut a, b| {
            a.runs += b.runs;
            a.unusable += b.unusable;
            a.identity.extend(b.identity);
            a.changed.extend(b.changed);
            a.unreadable.extend(b.unreadable);
            a.functions += b.functions;
            a.rewritten += b.rewritten;
            a
        });
    println!(
        "{} programs, {} rewrites in {:.1}s: {} not written back byte for byte, {} the transform couldn't read, {} that changed behavior ({} inputs unusable); {} of {} functions rewritten",
        programs.len(),
        tallies.runs,
        started.elapsed().as_secs_f64(),
        tallies.identity.len(),
        tallies.unreadable.len(),
        tallies.changed.len(),
        tallies.unusable,
        tallies.rewritten,
        tallies.functions
    );
    for line in tallies.identity.iter().chain(&tallies.unreadable).chain(&tallies.changed).take(show) {
        println!("  {line}");
    }
    if tallies.identity.is_empty() && tallies.unreadable.is_empty() && tallies.changed.is_empty() { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}

// UTC, to the second, without pulling in a date crate
fn now_utc() -> String {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0) as i64;
    let (days, rem) = (secs.div_euclid(86400), secs.rem_euclid(86400));
    // Howard Hinnant's civil_from_days
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z", rem / 3600, rem % 3600 / 60, rem % 60)
}

fn first_line(detail: &str) -> String {
    let line = detail.lines().find(|l| !l.trim().is_empty()).unwrap_or_default().trim();
    if line.len() > 240 { format!("{}...", &line[..line.floor_char_boundary(240)]) } else { line.to_string() }
}

pub fn run(specs: &[String], tier: Tier, versions: &[String], out: &Path, upload: bool, sign_results: bool) -> ExitCode {
    if specs.is_empty() {
        eprintln!("give at least one --decompiler name=command");
        return ExitCode::from(2);
    }
    let decompilers = match specs.iter().map(|s| External::parse(s)).collect::<Result<Vec<_>, _>>() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("--decompiler: {e}");
            return ExitCode::from(2);
        }
    };
    let mut version_of = BTreeMap::new();
    for v in versions {
        let Some((name, version)) = v.split_once('=') else {
            eprintln!("--version takes name=version");
            return ExitCode::from(2);
        };
        version_of.insert(name.to_string(), version.to_string());
    }
    if let Err(e) = fs::create_dir_all(out) {
        eprintln!("can't create {}: {e}", out.display());
        return ExitCode::from(2);
    }

    let started = Instant::now();
    let sections = sections(tier);
    let hash = fingerprint(tier, &sections);
    let tier_profiles = profiles(tier);
    let per_program = tier_profiles.len();
    let programs = sections.iter().map(|s| s.programs.len()).sum::<usize>();
    println!(
        "suite {SUITE} {} ({hash}): {programs} programs at {} profiles, ready in {:.1}s",
        tier.name(),
        per_program,
        started.elapsed().as_secs_f64()
    );

    let jobs = sections
        .iter()
        .enumerate()
        .flat_map(|(s, section)| (0..section.programs.len()).flat_map(move |p| (0..per_program).map(move |q| (s, p, q))))
        .collect::<Vec<_>>();
    for decompiler in &decompilers {
        let started = Instant::now();
        let outcomes = jobs
            .par_iter()
            .map(|&(s, p, q)| {
                let program = &sections[s].programs[p];
                // (a program's own compiler options, and the tier's transform level)
                let base = tier_profiles[q];
                let profile = Profile { extra: Extra { transform: base.extra.transform, ..program.extra }, ..base };
                let verdict = roundtrip::check(&program.source, profile, decompiler);
                (verdict.kind, verdict.detail, verdict.quality)
            })
            .collect::<Vec<_>>();
        let seconds = (started.elapsed().as_secs_f64() * 100.0).round() / 100.0;

        let mut next = outcomes.iter();
        let mut totals = BTreeMap::<Kind, usize>::new();
        let mut overall = Score::default();
        let mut passes = Vec::new();
        let mut out_sections = Vec::new();
        for section in &sections {
            let mut score = Score::default();
            let tests = section
                .programs
                .iter()
                .map(|program| {
                    let mut results = String::new();
                    let mut details = BTreeMap::new();
                    for base in &tier_profiles {
                        let (kind, detail, quality) = next.next().unwrap();
                        results.push(letter(*kind));
                        *totals.entry(*kind).or_default() += 1;
                        if *kind != Kind::Unusable {
                            score.total += 1;
                        }
                        if *kind == Kind::Pass {
                            score.passed += 1;
                        } else {
                            details.insert(base.to_string(), first_line(detail));
                        }
                        passes.extend(quality.as_ref());
                    }
                    TestOut { name: program.name.clone(), about: program.about.clone(), results, details }
                })
                .collect();
            overall.passed += score.passed;
            overall.total += score.total;
            out_sections.push(SectionOut { id: section.id, title: section.title, about: section.about.clone(), score, tests });
        }

        let name = decompiler.name().to_string();
        let result = BenchResult {
            format: "rosetta-result",
            schema: 1,
            rosetta: env!("CARGO_PKG_VERSION"),
            luau: LUAU,
            suite: SuiteInfo { name: SUITE, tier: tier.name(), hash: hash.clone() },
            decompiler: DecompilerInfo { version: version_of.get(&name).cloned(), name: name.clone() },
            created: now_utc(),
            seconds,
            threads: rayon::current_num_threads(),
            profiles: tier_profiles.iter().map(Profile::to_string).collect(),
            letters: LETTERS.into_iter().collect(),
            score: overall,
            totals,
            sections: out_sections,
            quality: (!passes.is_empty()).then(|| Summary::of(&passes)),
        };
        let file_name: String = name.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '-' }).collect();
        let path: PathBuf = out.join(format!("{file_name}-{}.json", tier.name()));
        fs::write(&path, serde_json::to_string(&result).unwrap() + "\n").expect("write result");
        let parts = result.sections.iter().map(|s| format!("{} {}/{}", s.id, s.score.passed, s.score.total)).collect::<Vec<_>>();
        println!(
            "{name}: {}/{} ({:.1}%) | {} | {seconds:.1}s -> {}",
            overall.passed,
            overall.total,
            overall.passed as f64 * 100.0 / overall.total.max(1) as f64,
            parts.join(", "),
            path.display()
        );
        if sign_results && let Err(e) = sign(&path) {
            eprintln!("  couldn't sign {}: {e}", path.display());
        }
        if upload {
            match share(&path, &format!("rosetta {} {name} {}", SUITE, tier.name())) {
                Ok(link) => println!("  shared: {link}"),
                Err(e) => eprintln!("  couldn't upload {}: {e}", path.display()),
            }
        }
    }
    ExitCode::SUCCESS
}

const SITE: &str = "https://rosetta.wtf/";

// The private half of the key the website checks signed results with. It never goes in the repo:
// ROSETTA_SIGNING_KEY names it, or it's ~/.config/rosetta/signing-key.pem
fn signing_key() -> PathBuf {
    std::env::var_os("ROSETTA_SIGNING_KEY").map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".config/rosetta/signing-key.pem")
    })
}

// Signs a result file in place: an ECDSA P-256 signature (SHA-256, r and s as 64 raw bytes, the
// form the browser's WebCrypto checks) over the file exactly as bench wrote it, added as a last
// "signature" field. The viewer takes that field back off and checks the rest. Needs openssl
pub fn sign(path: &Path) -> Result<(), String> {
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    let text = String::from_utf8(bytes.clone()).map_err(|_| "not UTF-8")?;
    if text.contains("\"signature\":") {
        return Err("already signed".into());
    }
    let Some(body) = text.strip_suffix("}\n") else { return Err("doesn't end the way bench writes a result".into()) };
    let key = signing_key();
    if !key.exists() {
        return Err(format!("no signing key at {}", key.display()));
    }
    let mut child = std::process::Command::new("openssl")
        .args(["dgst", "-sha256", "-binary", "-sign"])
        .arg(&key)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("openssl: {e}"))?;
    use std::io::Write;
    child.stdin.take().unwrap().write_all(&bytes).map_err(|e| e.to_string())?;
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    let raw = der_to_raw(&out.stdout).ok_or("openssl's signature isn't the DER openssl writes")?;
    fs::write(path, format!("{body},\"signature\":\"{}\"}}\n", base64(&raw))).map_err(|e| e.to_string())
}

// DER's SEQUENCE { INTEGER r, INTEGER s } as r and s, 32 bytes each
fn der_to_raw(der: &[u8]) -> Option<[u8; 64]> {
    let mut at = 2;
    if der.first() != Some(&0x30) {
        return None;
    }
    let mut out = [0u8; 64];
    for half in 0..2 {
        if der.get(at) != Some(&0x02) {
            return None;
        }
        let len = *der.get(at + 1)? as usize;
        let int = der.get(at + 2..at + 2 + len)?;
        let int = &int[int.iter().position(|&b| b != 0).unwrap_or(int.len())..];
        if int.len() > 32 {
            return None;
        }
        out[half * 32 + 32 - int.len()..half * 32 + 32].copy_from_slice(int);
        at += 2 + len;
    }
    Some(out)
}

fn base64(bytes: &[u8]) -> String {
    const A: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut s = String::new();
    for chunk in bytes.chunks(3) {
        let n = (chunk[0] as u32) << 16 | (*chunk.get(1).unwrap_or(&0) as u32) << 8 | *chunk.get(2).unwrap_or(&0) as u32;
        for k in 0..4 {
            s.push(if k <= chunk.len() { A[(n >> (18 - 6 * k) & 63) as usize] as char } else { '=' });
        }
    }
    s
}

// Uploads a result to Rubiš (https://rubis.app), a paste service with an open API, as a public
// scrap, and returns a link that opens it in the website's viewer. Sharing, not verification:
// the viewer still checks it against the official results
fn share(path: &Path, title: &str) -> Result<String, String> {
    let encoded = title.bytes().map(|b| if b.is_ascii_alphanumeric() || b"-_.".contains(&b) { (b as char).to_string() } else { format!("%{b:02X}") }).collect::<String>();
    let url = format!("https://api.rubis.app/v2/scrap?public=true&title={encoded}");
    let output = std::process::Command::new("curl")
        .args(["-sS", "-m", "60", "-X", "POST", "-H", "Content-Type: text/plain", "--data-binary"])
        .arg(format!("@{}", path.display()))
        .arg(&url)
        .output()
        .map_err(|e| format!("curl: {e}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    let reply: serde_json::Value = serde_json::from_slice(&output.stdout).map_err(|_| format!("unexpected reply: {}", String::from_utf8_lossy(&output.stdout).chars().take(200).collect::<String>()))?;
    let id = reply["scrapID"].as_str().filter(|id| !id.is_empty() && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')).ok_or_else(|| format!("no scrap id in the reply: {reply}"))?;
    Ok(format!("{SITE}result.html?rubis={id}"))
}
