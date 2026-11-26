mod bench;
mod bytecode;
mod chunk;
mod decompiler;
mod gallery;
mod generator;
mod lex;
mod luau;
mod mutate;
mod quality;
mod reduce;
mod rng;
mod roundtrip;
mod transform;

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

use clap::{Args, Parser, Subcommand};
use decompiler::{Decompiler, External};
use generator::Grammar;
use luau::{Extra, Profile};
use quality::{Quality, Summary};
use rayon::prelude::*;
use roundtrip::{Kind, Verdict};
use serde::Serialize;

#[derive(Parser)]
#[command(about = "Rosetta: round-trip tests for Luau decompilers. Compile, run, decompile, recompile, run again, compare")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Round-trip hand-written .luau programs and libraries (files or directories)
    Check {
        #[command(flatten)]
        common: Common,
        /// Defaults to the cases and libraries directories
        paths: Vec<PathBuf>,
    },
    /// Round-trip generated programs
    Fuzz {
        #[command(flatten)]
        common: Common,
        /// A count (500) or a range of seeds (1000..1500)
        #[arg(long, default_value = "200")]
        seeds: String,
        /// Shrink each failing program to a minimal one that fails the same way
        #[arg(long)]
        reduce: bool,
        #[arg(long, value_enum, default_value = "full")]
        grammar: Grammar,
    },
    /// Round-trip mutants of existing programs (Bin2Wrong-style), each kept only when its bytecode
    /// shows something no program before it did
    Mutate {
        #[command(flatten)]
        common: Common,
        /// Programs to mutate: .luau files or directories. Defaults to the cases directory
        #[arg(long = "pool")]
        pool: Vec<PathBuf>,
        /// Also mutate the programs these seeds generate (a count or a range like 100..200)
        #[arg(long)]
        pool_seeds: Option<String>,
        #[arg(long, value_enum, default_value = "full")]
        grammar: Grammar,
        /// Mutants to keep
        #[arg(long, default_value = "200")]
        count: usize,
        #[arg(long, default_value = "1")]
        seed: u64,
        /// Also mutate compiler options real scripts aren't built with: coverage, type info,
        /// mutable globals, disabled builtins
        #[arg(long)]
        compile_options: bool,
        /// Write every kept mutant here as a .luau file that `check` reruns as it was
        #[arg(long)]
        dump: Option<PathBuf>,
        /// Shrink each failing mutant to a minimal one that fails the same way
        #[arg(long)]
        reduce: bool,
    },
    /// The all-in-one benchmark: the cases, generated programs and mutants, pinned as one suite.
    /// Writes one JSON result per decompiler for the website
    Bench {
        /// name=command, as for check
        #[arg(long = "decompiler")]
        decompilers: Vec<String>,
        #[arg(long, value_enum, default_value = "quick")]
        tier: bench::Tier,
        /// name=version, shown with that decompiler's result. Repeat for each
        #[arg(long = "version")]
        versions: Vec<String>,
        /// Directory for the result files
        #[arg(long, default_value = ".")]
        out: PathBuf,
        /// Also upload each result to Rubiš (rubis.app, public) and print a link to it on the
        /// website's result viewer. Needs curl
        #[arg(long)]
        upload: bool,
        /// Sign each result with the key at ROSETTA_SIGNING_KEY (or ~/.config/rosetta/signing-key.pem),
        /// so the website shows it as verified wherever it's opened from. Only for whoever holds
        /// the key the website trusts. Needs openssl
        #[arg(long)]
        sign: bool,
        /// Also write each result's failure gallery (<result>.gallery.json): every wrong round
        /// trip with its program, the decompiler's output and where the printed lines part
        #[arg(long)]
        gallery: bool,
        /// With --gallery, shrink each failing program first (each attempt is a decompile, so
        /// this is for local decompilers, not rate-limited web APIs)
        #[arg(long, requires = "gallery")]
        reduce: bool,
    },
    /// Write the failure gallery for an existing result file, rerunning only its failed round
    /// trips with the same decompiler
    Gallery {
        result: PathBuf,
        /// name=command, the decompiler the result is for
        #[arg(long)]
        decompiler: String,
        /// Shrink each failing program first
        #[arg(long)]
        reduce: bool,
    },
    /// Sign result files in place, as bench --sign does
    Sign { files: Vec<PathBuf> },
    /// Vet library cases without a decompiler: the spec has to pass in full against the compiler's
    /// own bytecode, the same way at every profile
    LibraryCheck {
        /// Library directories; defaults to every one in libraries/
        dirs: Vec<PathBuf>,
        /// Print every test line
        #[arg(long)]
        verbose: bool,
    },
    /// Check the transform tier without a decompiler: every program's bytecode reads and writes
    /// back byte for byte, and rewritten at every level still prints what it printed
    TransformCheck {
        #[arg(long, value_enum, default_value = "quick")]
        tier: bench::Tier,
        /// Failures to print
        #[arg(long, default_value = "20")]
        show: usize,
        /// Only programs with this name
        #[arg(long)]
        only: Option<String>,
    },
    /// Run a bytecode file with Rosetta's Luau and print what it prints
    #[command(hide = true)]
    Run { bytecode: PathBuf },
    /// Print the program a seed generates
    Gen {
        seed: u64,
        #[arg(long, value_enum, default_value = "full")]
        grammar: Grammar,
    },
    /// Compile a .luau file (or a seed's program) with Rosetta's Luau, to reproduce a case
    Compile {
        /// a .luau file, or seed<N> for a generated program
        input: String,
        #[arg(long, default_value = "O1g1")]
        profile: String,
        #[arg(long, value_enum, default_value = "full")]
        grammar: Grammar,
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Check that every .lua file under a directory compiles
    Syntax { dir: PathBuf },
    /// Check that generated programs compile, finish and don't error at top level
    Selfcheck {
        #[arg(long, default_value = "500")]
        seeds: String,
        #[arg(long, value_enum, default_value = "full")]
        grammar: Grammar,
    },
}

#[derive(Args)]
struct Common {
    /// name=command, where {} is the path of a raw bytecode file and the source comes out on stdout,
    /// e.g. tovek='/path/luau-lifter {}'. Repeat to compare several.
    #[arg(long = "decompiler")]
    decompilers: Vec<String>,
    /// Comma-separated profiles such as O1g1,O2g2, or "all"
    #[arg(long, default_value = "all")]
    profiles: String,
    /// Where to write failing cases
    #[arg(long)]
    work: Option<PathBuf>,
    /// Write a JSON summary here
    #[arg(long)]
    report: Option<PathBuf>,
    /// Failures to print per decompiler
    #[arg(long, default_value_t = 15)]
    show: usize,
    /// Known failures: only failures missing from this file fail the run
    #[arg(long)]
    baseline: Option<PathBuf>,
    /// Rewrite --baseline with this run's failures
    #[arg(long, requires = "baseline")]
    update_baseline: bool,
}

#[derive(Serialize, serde::Deserialize, Default)]
struct Baseline {
    generator: String,
    // decompiler -> "case profile"
    failing: BTreeMap<String, Vec<String>>,
    // the same for refusals: a decompiler that refuses instead of guessing shouldn't start
    // refusing something it used to do
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    refused: BTreeMap<String, Vec<String>>,
}

// Luau's experimental syntax that cases use: `if local`/`if const` and `while local`/`while const`.
// Flags are process-wide, so this is set once before anything compiles
const FLAGS: [&str; 2] = ["LuauExperimentalIfLocalSyntax", "LuauExperimentalWhileLocalSyntax"];

fn main() -> ExitCode {
    for flag in FLAGS {
        mlua::Lua::set_fflag(flag, true).unwrap_or_else(|()| panic!("this Luau has no fast flag {flag}"));
    }

    match Cli::parse().command {
        Command::Bench { decompilers, tier, versions, out, upload, sign, gallery, reduce } => bench::run(&decompilers, tier, &versions, &out, upload, sign, gallery, reduce),
        Command::Gallery { result, decompiler, reduce } => bench::backfill(&result, &decompiler, reduce),
        Command::Sign { files } => {
            let mut failed = false;
            for f in &files {
                match bench::sign(f) {
                    Ok(()) => println!("signed {}", f.display()),
                    Err(e) => {
                        eprintln!("{}: {e}", f.display());
                        failed = true;
                    }
                }
            }
            if failed { ExitCode::FAILURE } else { ExitCode::SUCCESS }
        }
        Command::Gen { seed, grammar } => {
            print!("{}", generator::generate(seed, grammar).render());
            ExitCode::SUCCESS
        }
        Command::Run { bytecode } => {
            let observed = luau::run(&fs::read(&bytecode).expect("read bytecode"));
            for line in &observed.log {
                println!("{line}");
            }
            println!("-- {:?}", observed.ending);
            ExitCode::SUCCESS
        }
        Command::LibraryCheck { dirs, verbose } => {
            let (mut files, mut library_dirs) = (Vec::new(), Vec::new());
            let dirs = if dirs.is_empty() { vec![Path::new(env!("CARGO_MANIFEST_DIR")).join("libraries")] } else { dirs };
            for dir in dirs {
                collect_programs(&dir, &mut files, &mut library_dirs);
            }
            library_dirs.sort();
            let mut ok = true;
            for dir in library_dirs {
                let case = match load_library(&dir) {
                    Ok(c) => c,
                    Err(e) => {
                        println!("{e}");
                        ok = false;
                        continue;
                    }
                };
                let name = &case.library.name;
                let mut first: Option<luau::Observation> = None;
                for profile in Profile::ALL {
                    let started = std::time::Instant::now();
                    let observed = match luau::compile(&case.source, profile) {
                        Ok(b) => luau::run_with(&b, Some(&case.library)),
                        Err(e) => {
                            println!("{name} {profile}: doesn't compile: {e}");
                            ok = false;
                            continue;
                        }
                    };
                    // the busted shim's lines, and the runners json.lua and lume ship with
                    let is_pass = |l: &str| ["ok\t", "[pass]", "[PASS]"].iter().any(|p| l.starts_with(p));
                    let is_fail = |l: &str| ["FAIL\t", "[fail]", "[FAIL]"].iter().any(|p| l.starts_with(p));
                    let passed = observed.log.iter().filter(|l| is_pass(l)).count();
                    let failed = observed.log.iter().filter(|l| is_fail(l)).collect::<Vec<_>>();
                    println!(
                        "{name} {profile}: {passed} passed, {} failed, {:?}, {:.0} ms",
                        failed.len(),
                        observed.ending,
                        started.elapsed().as_secs_f64() * 1000.0
                    );
                    for line in &failed {
                        println!("  {line}");
                    }
                    if verbose && first.is_none() {
                        for line in &observed.log {
                            println!("  {line}");
                        }
                    }
                    ok &= failed.is_empty() && passed > 0 && observed.ending == luau::Ending::Returned;
                    match &first {
                        None => first = Some(observed),
                        Some(f) if *f != observed => {
                            println!("  differs from {}", Profile::ALL[0]);
                            ok = false;
                        }
                        Some(_) => {}
                    }
                }
            }
            if ok { ExitCode::SUCCESS } else { ExitCode::FAILURE }
        }
        Command::TransformCheck { tier, show, only } => bench::transform_check(tier, show, only.as_deref()),
        Command::Compile { input, profile, grammar, output } => {
            let Some(profile) = Profile::parse(&profile) else {
                eprintln!("profiles look like O1g1");
                return ExitCode::from(2);
            };
            let source = match input.strip_prefix("seed").and_then(|n| n.parse().ok()) {
                Some(seed) => generator::generate(seed, grammar).render(),
                None => fs::read_to_string(&input).expect("read input"),
            };
            match luau::compile(&source, profile).and_then(|b| if profile.extra.transform > 0 { transform::apply(&b, profile.extra.transform) } else { Ok(b) }) {
                Ok(bytecode) => {
                    fs::write(&output, bytecode).expect("write output");
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("{e}");
                    ExitCode::FAILURE
                }
            }
        }
        Command::Syntax { dir } => {
            let files = fs::read_dir(&dir).expect("read dir").flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|e| e == "lua")).collect::<Vec<_>>();
            let bad = files
                .par_iter()
                .filter_map(|f| {
                    let text = fs::read_to_string(f).ok()?;
                    luau::compile(&text, Profile::new(1, 1)).err().map(|e| format!("{}: {}", f.display(), e.lines().next().unwrap_or_default()))
                })
                .collect::<Vec<_>>();
            println!("{} files, {} don't compile", files.len(), bad.len());
            for b in bad.iter().take(10) {
                println!("  {b}");
            }
            if bad.is_empty() { ExitCode::SUCCESS } else { ExitCode::FAILURE }
        }
        Command::Selfcheck { seeds, grammar } => {
            let Some(seeds) = parse_seeds(&seeds) else {
                eprintln!("--seeds takes a count or a range like 100..200");
                return ExitCode::from(2);
            };
            let problems = seeds
                .into_par_iter()
                .flat_map_iter(|seed| {
                    let source = generator::generate(seed, grammar).render();
                    Profile::ALL.into_iter().filter_map(move |profile| {
                        let problem = match luau::compile(&source, profile) {
                            Err(e) => format!("compile: {e}"),
                            Ok(bytecode) => match luau::run(&bytecode).ending {
                                luau::Ending::Returned => return None,
                                luau::Ending::TimedOut => "timeout".to_string(),
                                luau::Ending::Errored(e) => format!("error: {e}"),
                            },
                        };
                        Some((seed, profile, problem))
                    })
                })
                .collect::<Vec<_>>();
            let mut kinds = BTreeMap::<String, (usize, u64)>::new();
            for (seed, _, problem) in &problems {
                let key = problem.split(" with ").next().unwrap().to_string();
                kinds.entry(key).or_insert((0, *seed)).0 += 1;
            }
            for (problem, (count, seed)) in &kinds {
                println!("{count:5}  {problem}  (e.g. seed {seed})");
            }
            println!("{} problem runs", problems.len());
            if problems.is_empty() { ExitCode::SUCCESS } else { ExitCode::FAILURE }
        }
        Command::Check { common, paths } => {
            let root = Path::new(env!("CARGO_MANIFEST_DIR"));
            let paths = if paths.is_empty() { vec![root.join("cases"), root.join("libraries")] } else { paths };
            let (mut files, mut library_dirs) = (Vec::new(), Vec::new());
            for path in paths {
                collect_programs(&path, &mut files, &mut library_dirs);
            }
            files.sort();
            library_dirs.sort();
            let mut cases = files
                .into_iter()
                .map(|p| {
                    let extra = fs::read_to_string(&p).ok().and_then(|text| options_directive(&text)).unwrap_or_default();
                    Case { name: p.file_stem().unwrap().to_string_lossy().into_owned(), source: Source::File(p), extra, library: None }
                })
                .collect::<Vec<_>>();
            for dir in library_dirs {
                match load_library(&dir) {
                    Ok(l) => cases.push(Case { name: l.library.name.clone(), source: Source::Text(l.source), extra: Extra::default(), library: Some(l.library) }),
                    Err(e) => {
                        eprintln!("{e}");
                        return ExitCode::from(2);
                    }
                }
            }
            run(&common, cases, false, generator::GENERATOR_VERSION)
        }
        Command::Fuzz { common, seeds, reduce, grammar } => {
            let Some(seeds) = parse_seeds(&seeds) else {
                eprintln!("--seeds takes a count or a range like 100..200");
                return ExitCode::from(2);
            };
            let cases = seeds.map(|seed| Case { name: format!("seed{seed}"), source: Source::Seed(seed, grammar), extra: Extra::default(), library: None }).collect();
            let generator = match grammar {
                Grammar::Full => generator::GENERATOR_VERSION.to_string(),
                Grammar::Straight => format!("{}-straight", generator::GENERATOR_VERSION),
                Grammar::Branches => format!("{}-branches", generator::GENERATOR_VERSION),
            };
            run(&common, cases, reduce, &generator)
        }
        Command::Mutate { common, pool, pool_seeds, grammar, count, seed, compile_options, dump, reduce } => {
            let mut files = Vec::new();
            let paths = if pool.is_empty() { vec![Path::new(env!("CARGO_MANIFEST_DIR")).join("cases")] } else { pool };
            for path in paths {
                collect_luau(&path, &mut files);
            }
            files.sort();
            let mut programs = files
                .iter()
                .filter_map(|p| Some((p.file_stem()?.to_string_lossy().into_owned(), fs::read_to_string(p).ok()?)))
                .collect::<Vec<_>>();
            if let Some(seeds) = pool_seeds {
                let Some(seeds) = parse_seeds(&seeds) else {
                    eprintln!("--pool-seeds takes a count or a range like 100..200");
                    return ExitCode::from(2);
                };
                programs.extend(seeds.map(|s| (format!("seed{s}"), generator::generate(s, grammar).render())));
            }
            let hash = mutate::pool_hash(&programs);
            let pool_size = programs.len();
            let started = std::time::Instant::now();
            let (mutants, stats) = mutate::evolve(programs, seed, count, compile_options);
            println!(
                "kept {} mutants of {pool_size} programs (pool {hash}): {} attempts, {} usable, {} bytecode features, {:.1}s",
                mutants.len(),
                stats.attempts,
                stats.usable,
                stats.features,
                started.elapsed().as_secs_f64()
            );
            if let Some(dir) = &dump {
                fs::create_dir_all(dir).expect("create dump dir");
                for m in &mutants {
                    fs::write(dir.join(format!("{}.luau", m.name)), m.file_text()).expect("write mutant");
                }
                println!("wrote them to {}", dir.display());
                if common.decompilers.is_empty() {
                    return ExitCode::SUCCESS;
                }
            }
            let cases = mutants.into_iter().map(|m| Case { name: m.name.clone(), extra: m.extra, source: Source::Text(m.file_text()), library: None }).collect();
            let options = if compile_options { "-options" } else { "" };
            let generator = format!("mutate-{}-{}-seed{seed}-pool{hash}{options}", mutate::MUTATOR_VERSION, generator::GENERATOR_VERSION);
            run(&common, cases, reduce, &generator)
        }
    }
}

// `--!rosetta-options +c2+m3` near the top of a file: compiler options a dumped mutant needs
fn options_directive(text: &str) -> Option<Extra> {
    text.lines().take(3).find_map(|l| l.strip_prefix("--!rosetta-options ")).and_then(|o| Extra::parse(o.trim()))
}

fn parse_seeds(text: &str) -> Option<std::ops::Range<u64>> {
    match text.split_once("..") {
        Some((a, b)) => Some(a.parse().ok()?..b.parse().ok()?),
        None => Some(0..text.parse().ok()?),
    }
}

fn collect_luau(path: &Path, out: &mut Vec<PathBuf>) {
    if path.is_dir() {
        for entry in fs::read_dir(path).into_iter().flatten().flatten() {
            collect_luau(&entry.path(), out);
        }
    } else if path.extension().is_some_and(|e| e == "luau") {
        out.push(path.to_path_buf());
    }
}

// A directory holding spec.luau is a library (see load_library); anything else is searched for
// plain .luau programs
fn collect_programs(path: &Path, files: &mut Vec<PathBuf>, libraries: &mut Vec<PathBuf>) {
    if path.join("spec.luau").is_file() {
        libraries.push(path.to_path_buf());
    } else if path.is_dir() {
        for entry in fs::read_dir(path).into_iter().flatten().flatten() {
            collect_programs(&entry.path(), files, libraries);
        }
    } else if path.extension().is_some_and(|e| e == "luau") {
        files.push(path.to_path_buf());
    }
}

pub struct LibraryCase {
    pub source: String,
    pub library: luau::Library,
}

// libraries/<name>/ holds the library as <name>.luau, the spec that grades it as spec.luau and
// the library's license
pub fn load_library(dir: &Path) -> Result<LibraryCase, String> {
    let name = dir.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let read = |file: &str| fs::read_to_string(dir.join(file)).map_err(|e| format!("{}: {e}", dir.join(file).display()));
    let source = read(&format!("{name}.luau"))?;
    let library = luau::Library::new(&name, &read("spec.luau")?)?;
    Ok(LibraryCase { source, library })
}

enum Source {
    File(PathBuf),
    Seed(u64, Grammar),
    Text(String),
}

struct Case {
    name: String,
    source: Source,
    // compiler options for every profile this case runs at
    extra: Extra,
    // set when the source is a library graded by its spec
    library: Option<luau::Library>,
}

impl Case {
    fn text(&self) -> String {
        match &self.source {
            Source::File(p) => fs::read_to_string(p).unwrap_or_default(),
            Source::Seed(s, g) => generator::generate(*s, *g).render(),
            Source::Text(t) => t.clone(),
        }
    }
}

#[derive(Serialize)]
struct Failure {
    case: String,
    profile: String,
    kind: Kind,
    detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    reduced_lines: Option<usize>,
}

#[derive(Serialize)]
struct Report {
    generator: String,
    cases: usize,
    profiles: Vec<String>,
    // decompiler -> kind -> count
    totals: BTreeMap<String, BTreeMap<Kind, usize>>,
    // decompiler -> profile -> failures
    by_profile: BTreeMap<String, BTreeMap<String, usize>>,
    failures: BTreeMap<String, Vec<Failure>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quality: Option<QualityReport>,
}

// Readability, measured only on the round trips every decompiler in the run passed, so no
// decompiler is averaged over an easier set than another
#[derive(Serialize)]
struct QualityReport {
    round_trips: usize,
    by_decompiler: BTreeMap<String, Summary>,
}

fn decompilers(specs: &[String]) -> Result<Vec<Box<dyn Decompiler>>, String> {
    specs
        .iter()
        .map(|spec| -> Result<Box<dyn Decompiler>, String> { Ok(Box::new(External::parse(spec)?)) })
        .collect()
}

fn run(common: &Common, cases: Vec<Case>, reduce: bool, generator_version: &str) -> ExitCode {
    let profiles = if common.profiles == "all" {
        Profile::ALL.to_vec()
    } else {
        match common.profiles.split(',').map(Profile::parse).collect::<Option<Vec<_>>>() {
            Some(p) => p,
            None => {
                eprintln!("profiles look like O1g1 (optimization 0-2, debug 0-2)");
                return ExitCode::from(2);
            }
        }
    };
    if common.decompilers.is_empty() {
        eprintln!("give at least one --decompiler name=command");
        return ExitCode::from(2);
    }
    let decompilers = match decompilers(&common.decompilers) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("--decompiler: {e}");
            return ExitCode::from(2);
        }
    };

    let decompiler_count = decompilers.len();
    let jobs = cases
        .iter()
        .flat_map(|case| profiles.iter().flat_map(move |p| (0..decompiler_count).map(move |d| (case, Profile { extra: Extra { transform: p.extra.transform, ..case.extra }, ..*p }, d))))
        .collect::<Vec<_>>();
    let started = std::time::Instant::now();
    let results = jobs
        .par_iter()
        .map(|&(case, profile, d)| {
            let source = case.text();
            let decompiler = decompilers[d].as_ref();
            let verdict = roundtrip::check(&source, case.library.as_ref(), profile, decompiler);
            let mut reduced = None;
            if verdict.kind.is_failure() || verdict.kind == Kind::Refused {
                let kind = verdict.kind;
                if reduce && let Source::Seed(seed, grammar) = case.source {
                    let (program, _) = reduce::reduce(
                        &generator::generate(seed, grammar),
                        |p| roundtrip::check(&p.render(), None, profile, decompiler).kind == kind,
                        400,
                    );
                    reduced = Some(program.render());
                } else if reduce && case.library.is_none() && let Source::Text(text) = &case.source {
                    reduced = Some(mutate::reduce(text, |s| roundtrip::check(s, None, profile, decompiler).kind == kind, 400));
                }
            }
            // refusals too: a refusal is something to fix, just not a wrong one
            if (verdict.kind.is_failure() || verdict.kind == Kind::Refused)
                && let Some(work) = &common.work
            {
                save(work, decompiler.name(), &case.name, profile, &source, &verdict, reduced.as_deref());
            }
            (case, profile, d, verdict.kind, verdict.detail, reduced.map(|r| r.lines().count()), verdict.quality)
        })
        .collect::<Vec<_>>();
    let quality = quality_report(&results, &decompilers);

    let mut report = Report {
        generator: generator_version.to_string(),
        cases: cases.len(),
        profiles: profiles.iter().map(Profile::to_string).collect(),
        totals: BTreeMap::new(),
        by_profile: BTreeMap::new(),
        failures: BTreeMap::new(),
        quality,
    };
    let refusals = results
        .iter()
        .filter(|r| r.3 == Kind::Refused)
        .map(|(case, profile, d, kind, detail, _, _)| (case.name.clone(), *profile, *d, *kind, detail.clone()))
        .collect::<Vec<_>>();
    for (case, profile, d, kind, detail, reduced_lines, _) in results {
        let name = decompilers[d].name().to_string();
        *report.totals.entry(name.clone()).or_default().entry(kind).or_default() += 1;
        if kind.is_failure() {
            *report.by_profile.entry(name.clone()).or_default().entry(profile.to_string()).or_default() += 1;
            report.failures.entry(name).or_default().push(Failure {
                case: case.name.clone(),
                profile: profile.to_string(),
                kind,
                detail,
                reduced_lines,
            });
        }
    }

    let mut failed = false;
    for decompiler in &decompilers {
        let name = decompiler.name();
        let totals = report.totals.get(name).cloned().unwrap_or_default();
        let count = |k| totals.get(&k).copied().unwrap_or(0);
        let usable = totals.values().sum::<usize>() - count(Kind::Unusable);
        println!(
            "{name}: {}/{usable} pass | {} mismatch, {} don't recompile, {} decompile failures, {} refused | {} unusable inputs",
            count(Kind::Pass),
            count(Kind::Mismatch),
            count(Kind::RecompileFailed),
            count(Kind::DecompileFailed),
            count(Kind::Refused),
            count(Kind::Unusable),
        );
        if let Some(by_profile) = report.by_profile.get(name) {
            let parts = by_profile.iter().map(|(p, n)| format!("{p} {n}")).collect::<Vec<_>>();
            println!("  failures by profile: {}", parts.join(", "));
        }
        for failure in report.failures.get(name).into_iter().flatten().take(common.show) {
            let detail = failure.detail.lines().next().unwrap_or_default();
            let detail = if detail.len() > 160 { format!("{}...", &detail[..detail.floor_char_boundary(160)]) } else { detail.to_string() };
            let reduced = failure.reduced_lines.map(|n| format!(" (reduced to {n} lines)")).unwrap_or_default();
            println!("  {} {} {:?}{reduced}: {detail}", failure.case, failure.profile, failure.kind);
        }
        failed |= report.failures.contains_key(name);
        let mut reasons = BTreeMap::<String, (usize, String)>::new();
        for (case, profile, d, kind, detail) in &refusals {
            if decompilers[*d].name() == name {
                let key = detail.chars().map(|c| if c.is_ascii_digit() { 'N' } else { c }).collect::<String>().replace("NN", "N").replace("NN", "N");
                reasons.entry(key).or_insert((0, format!("{case} {profile}"))).0 += 1;
                let _ = kind;
            }
        }
        let mut reasons = reasons.into_iter().collect::<Vec<_>>();
        reasons.sort_by(|a, b| b.1.0.cmp(&a.1.0).then(a.0.cmp(&b.0)));
        for (reason, (count, example)) in reasons.iter().take(8) {
            println!("  refused {count:5}x  {reason}  (e.g. {example})");
        }
    }
    if let Some(quality) = &report.quality {
        print_quality(quality);
    }
    println!("{} round trips in {:.1}s", jobs.len(), started.elapsed().as_secs_f64());

    if let Some(path) = &common.report {
        fs::write(path, serde_json::to_string_pretty(&report).unwrap()).expect("write report");
    }

    if let Some(path) = &common.baseline {
        let current = Baseline {
            generator: generator_version.to_string(),
            failing: report
                .failures
                .iter()
                .map(|(name, failures)| {
                    let mut keys = failures.iter().map(|f| format!("{} {}", f.case, f.profile)).collect::<Vec<_>>();
                    keys.sort();
                    (name.clone(), keys)
                })
                .collect(),
            refused: {
                let mut refused = BTreeMap::<String, Vec<String>>::new();
                for (case, profile, d, _, _) in &refusals {
                    refused.entry(decompilers[*d].name().to_string()).or_default().push(format!("{case} {profile}"));
                }
                refused.values_mut().for_each(|keys| keys.sort());
                refused
            },
        };
        if common.update_baseline {
            fs::write(path, serde_json::to_string_pretty(&current).unwrap() + "\n").expect("write baseline");
            println!("baseline written to {}", path.display());
            return ExitCode::SUCCESS;
        }
        let known: Baseline = match fs::read_to_string(path).map(|t| serde_json::from_str(&t)) {
            Ok(Ok(b)) => b,
            _ => {
                eprintln!("can't read baseline {}", path.display());
                return ExitCode::from(2);
            }
        };
        if known.generator != current.generator {
            eprintln!("baseline is for generator {}, this is {}; rerun with --update-baseline", known.generator, current.generator);
            return ExitCode::from(2);
        }
        let mut regressed = false;
        for decompiler in &decompilers {
            let name = decompiler.name();
            for (what, known, current) in [("failures", &known.failing, &current.failing), ("refusals", &known.refused, &current.refused)] {
                let empty = Vec::new();
                let (was, now) = (known.get(name).unwrap_or(&empty), current.get(name).unwrap_or(&empty));
                let new = now.iter().filter(|k| was.binary_search(k).is_err()).collect::<Vec<_>>();
                let fixed = was.iter().filter(|k| now.binary_search(k).is_err()).count();
                if !new.is_empty() {
                    regressed = true;
                    println!("{name}: {} new {what} against the baseline: {}", new.len(), new.iter().take(20).map(|k| k.as_str()).collect::<Vec<_>>().join(", "));
                }
                if fixed > 0 {
                    println!("{name}: {fixed} baseline {what} are gone now; rerun with --update-baseline to lock that in");
                }
            }
        }
        return if regressed { ExitCode::FAILURE } else { ExitCode::SUCCESS };
    }
    if failed { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}

type Outcome<'a> = (&'a Case, Profile, usize, Kind, String, Option<usize>, Option<Quality>);

fn quality_report(results: &[Outcome], decompilers: &[Box<dyn Decompiler>]) -> Option<QualityReport> {
    let mut passes = BTreeMap::<(&str, Profile), Vec<Option<&Quality>>>::new();
    for (case, profile, d, _, _, _, quality) in results {
        passes.entry((&case.name, *profile)).or_insert_with(|| vec![None; decompilers.len()])[*d] = quality.as_ref();
    }
    let common = passes.values().filter_map(|qs| qs.iter().copied().collect::<Option<Vec<_>>>()).collect::<Vec<_>>();
    if common.is_empty() {
        return None;
    }
    let by_decompiler = decompilers
        .iter()
        .enumerate()
        .map(|(d, decompiler)| (decompiler.name().to_string(), Summary::of(&common.iter().map(|qs| qs[d]).collect::<Vec<_>>())))
        .collect();
    Some(QualityReport { round_trips: common.len(), by_decompiler })
}

fn print_quality(quality: &QualityReport) {
    println!("readability on the {} round trips every decompiler passed:", quality.round_trips);
    let width = quality.by_decompiler.keys().map(String::len).max().unwrap_or(0).max(10);
    println!("  {:width$}  similarity   size  locals  nesting  endless  names g2  cpu ms", "decompiler");
    for (name, s) in &quality.by_decompiler {
        let names = s.names.map_or("-".to_string(), |n| format!("{n:.1}%"));
        println!(
            "  {name:width$}  {:>9.1}%  {:>4.2}x  {:>5.2}x  {:>+7.2}  {:>7}  {:>8}  {:>6.2}",
            s.similarity, s.size, s.locals, s.nesting, s.endless, names, s.cpu_ms_median
        );
    }
}

fn save(work: &Path, decompiler: &str, case: &str, profile: Profile, source: &str, verdict: &Verdict, reduced: Option<&str>) {
    let dir = work.join(decompiler).join(format!("{case}-{profile}"));
    if fs::create_dir_all(&dir).is_err() {
        return;
    }
    let _ = fs::write(dir.join("input.luau"), source);
    let _ = fs::write(dir.join("detail.txt"), format!("{:?}: {}\n", verdict.kind, verdict.detail));
    if let Some(output) = &verdict.output {
        let _ = fs::write(dir.join("output.lua"), output);
    }
    if let Some(want) = &verdict.want {
        let _ = fs::write(dir.join("want.txt"), want.render());
    }
    if let Some(got) = &verdict.got {
        let _ = fs::write(dir.join("got.txt"), got.render());
    }
    if let Some(reduced) = reduced {
        let _ = fs::write(dir.join("reduced.luau"), reduced);
    }
}
