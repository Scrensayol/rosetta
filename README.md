# Rosetta

A standardized, open-source benchmark for Luau decompilers.

Every decompiler claims to be accurate. Rosetta checks it the only way that can't be argued with:
it compiles a program, runs it, hands the bytecode to the decompiler, compiles what comes back,
runs that, and compares the two runs. Same output, it passes. Anything else is graded.

New suites and official scores are posted on [Discord](https://discord.gg/7qE27wRwnd) as they land.

## How a program is graded

Each program is compiled at six Luau profiles (`-O0`, `-O1`, `-O2`, each at debug level 1 and 2)
and every decompiler gets the same bytecode.

| verdict | meaning |
| --- | --- |
| pass | the decompiled source compiles and prints exactly what the original printed |
| mismatch | it compiles but behaves differently: **a wrong decompile**, the worst outcome |
| doesn't recompile | the output isn't valid Luau |
| refused | the decompiler said it can't do this one (a clean non-zero exit): no output, but no wrong one either |
| failed | it crashed (a signal, or a Rust panic's exit status 101) or ran past 30 seconds |
| unusable | the input itself doesn't finish or errors at top level; it counts for nobody |

The score is passes out of usable programs. Wrong decompiles are reported on their own, because a
decompiler that refuses 1% is far more useful than one that's quietly wrong 1% of the time.

## How the output reads

Passing is the gate. Every pass is then measured against the source the bytecode came from, and
a run prints one more table:

| column | what it is |
| --- | --- |
| similarity | how much of the original's token sequence the output keeps in order, with every name, number and string replaced by its kind, so renaming costs nothing here |
| size | output tokens over original tokens (comments don't count) |
| locals | names the output declares over names the original declares: temporaries push this up |
| nesting | the output's deepest block nesting minus the original's |
| endless | `while true do` and `repeat ... until false` loops the output has beyond the original's: what a decompiler falls back to when it can't recover a loop's condition |
| names g2 | at debug level 2, where the bytecode carries local names: how many of the original's local, parameter and loop-variable names come back |
| cpu ms | the median CPU time per decompile, the decompiler's own plus any processes it starts. For a web API's adapter that's the adapter's local work, not the server's |

Ratios are averaged geometrically, so one big program doesn't drown the small ones. All of it is
measured **only on round trips every decompiler in the run passed**, so nobody is averaged over an
easier set than anyone else. That makes the numbers depend on who's in the run, so compare rows
within one table, never across tables. Wrong decompiles have no readability: a decompiler that
prints clean but wrong code scores nothing for it. None of it is folded into one score either.
Weights would be arbitrary, and the columns pull against each other: a decompiler can shrink
`size` by inlining everything.

None of this is a judgment of taste, and there's no model in the loop. Every number comes from
the original program, so it's reproducible and anyone can check it.

## Programs

- **cases/**: hand-written programs, each aimed at something decompilers get wrong: inlined
  functions, loop exits, closures and upvalues, numbers (NaN sign, `x^3` folding), contextual
  keywords (`while continue do`), dead branches, `if`s nested 150 deep, Luau's experimental
  `if local` / `while local`, comparisons with NaN, operator precedence, string escapes and
  number edges, table constructors, iterators, metamethods, builtins, Roblox-style scripts,
  evaluation order with side effects, `getfenv` / `setfenv`, type syntax compiled with type
  information, generalized iteration and `__iter`, a function at the local and register
  limits, and vector constants. New edge cases are welcome.
- **libraries/**: open-source Luau libraries graded by their own test suites. Only the library
  is decompiled; its upstream spec then runs against the output in the same VM, reaching it
  through `require`, under a small busted shim (src/busted.luau). Real code with real
  semantics, judged by tests nobody wrote for Rosetta. `rosetta library-check` vets a new one:
  its spec has to pass in full on the compiler's own bytecode, the same way at every profile.
  See [libraries/README.md](libraries/README.md) for versions, licenses and what was ported.
- **generated**: `fuzz` builds random programs from a seed with a grammar (`full`, `straight`
  for straight-line code, `branches` for no loops), so a run is reproducible from its seed range.
- **mutants**: `mutate` is mutation fuzzing in the style of Bin2Wrong. It starts from existing programs (the cases by default, plus `--pool` directories and
  `--pool-seeds`), changes their source again and again, and keeps every mutant whose bytecode
  shows something no program before it did. Mutations wrap statements in blocks, loops, closures
  and local functions (which `-O2` inlines), duplicate, delete and swap statements, negate
  conditions, flip operators, swap in edge-case numbers and strings, and print variables so more
  values get compared. "Something new" means opcode pairs and triples, and where jumps land, at
  each optimization level. That depends only on the bytecode, never on how a decompiler did, so a
  seed and a pool give every decompiler the same mutants in the same order, whatever the thread
  count. `--compile-options` also mutates compiler options real scripts aren't built with
  (coverage instructions, type info, mutable globals, disabled builtins), and those mutants
  carry them in their profile name, like `O1g1+c2+m5`. `--dump DIR` writes the mutants out
  as files that `check` reruns exactly, options included.

## Decompilers

Every decompiler runs the same way, as a command: `name=command`, where `{}` is the path of a raw
(unencoded) bytecode file and the source comes out on stdout. Nothing runs in-process, so nobody
gets an advantage. A decompiler that only runs as a web API or a server gets a small adapter
script: [adapters/](adapters) has lua.expert, Composite, luacid and Fission. luarust is a private
API that needs its owner's key, so its adapter isn't published.

```sh
cargo build --release
R=target/release/rosetta

# the hand-written cases
$R check --decompiler "mine=/path/to/decompiler {}"

# 1000 generated programs, three decompilers side by side
$R fuzz --seeds 1000..2000 \
  --decompiler "sunberry=sunberry decompile --strict -e 1 -i {} -o /dev/stdout" \
  --decompiler "tovek=luau-lifter {}" \
  --decompiler "mine=/path/to/decompiler {}"

# 1000 mutants of the cases, kept for new bytecode shapes (same seed, same mutants)
$R mutate --count 1000 --seed 1 --decompiler "mine=/path/to/decompiler {}"
```

Other commands: `gen <seed>` prints a generated program, `compile` compiles a file or seed with
Rosetta's Luau to reproduce a case, `--reduce` shrinks a failing program to a minimal one,
`--report` writes a JSON summary, and `--baseline` / `--update-baseline` let a CI run fail only
on failures that are new (see [sunberry's workflow](https://github.com/Scrensayol/sunberry/blob/main/.github/workflows/test.yaml)).

The Luau compiler and VM come from `luau0-src` 0.24.0 (Luau 0.747) through `mlua` pinned to a
commit on its main branch, because the Luau version decides what the programs compile to.
The experimental `if local` and `while local` syntax is switched on, so cases can use it.

## The benchmark

`bench` runs all three kinds of program as one suite and gives one score:

```sh
$R bench --decompiler "mine=/path/to/decompiler {}" --version mine=1.0            # quick
$R bench --tier full --decompiler "mine=/path/to/decompiler {}" --version mine=1.0
```

| tier | programs | round trips |
|---|---|---|
| quick | the 41 cases and 6 libraries, then the 50 generated programs and the 50 mutants of the full tier whose bytecode has the most features | 882 |
| full | the 41 cases and 6 libraries, seeds 1000..2000, 1000 mutants of the cases (seed 1) | 12,276 |
| hard-quick | quick's programs, with the bytecode rewritten before the decompiler gets it (below) | 882 |
| hard-full | full's programs, rewritten the same way | 12,276 |

"Features" are the ones mutation fuzzing counts: distinct opcode pairs and triples and where
jumps land, at each optimization level. Picking by them never looks at a decompiler, and it
makes quick harder than the full tier on average, not an easy sample of it. Every program runs at
all six profiles. The score is the round trips that passed, out of every
one whose input was usable. The suite is pinned as `rosetta-3`: which programs run, the
profiles and the Luau version are fingerprinted into a suite hash, so two results with the same
hash ran the same programs. Anything that changes the programs gets a new suite name. `rosetta-1` was the first 35 cases;
`rosetta-2` added six and, because mutants come from the cases, a new set of mutants.
`rosetta-3` added the libraries and fixed a hard-tier transform that left captured locals open
(transform-2), so its hard tiers aren't comparable with rosetta-2's.

### The hard tiers

On Roblox, bytecode always comes from Roblox's compiler, so a decompiler can lean on the shapes
that compiler writes. The hard tiers don't let it: Rosetta reads each program's bytecode
([src/chunk.rs](src/chunk.rs), every byte of Luau's format, written back byte for byte),
rewrites it so it does exactly the same thing ([src/transform.rs](src/transform.rs)), and hands
the decompiler that. What it measures is robustness and real control-flow and data-flow
analysis, which matters most for other Luau embedders, so they're scored on their own and never
mixed into quick or full. hard-quick runs quick's programs and hard-full full's, at the six
profiles, each paired with a level
(`O0g1+x1`, `O1g1+x2`, `O2g1+x3`, `O0g2+x2`, `O1g2+x3`, `O2g2+x1`):

| level | what changes |
| --- | --- |
| x1 | registers past the parameters shifted up, so locals don't start right after them; no-op instructions and jumps to the next instruction; unreachable junk after every instruction that never falls through |
| x2 | and registers renumbered in a shuffled order (keeping what calls, loops and CLOSEUPVALS need), instructions swapped for others that do the same (JUMPX for JUMP, an inverted test over a JUMP, AND or OR of a register with itself for MOVE, LOADK for LOADN / LOADNIL / LOADB, a constant loaded into a register for arithmetic with it, GETGLOBAL and GETTABLEKS for GETIMPORT), basic blocks laid out in a shuffled order and glued with jumps, and insertions also between a call whose results run to the top of the stack and the instruction that takes them. Local debug info goes, since its ranges no longer mean anything |
| x3 | and branches that always go one way guard junk (a register loaded with a boolean, or `b <= a * a` worked out at run time), functions are shuffled in the chunk, and upvalue names go |

Every rewrite is seeded by the program's own bytecode, so every decompiler gets the same bytes.
Before a decompiler sees it, the round trip runs the rewritten bytecode: if it doesn't print
exactly what the compiler's did, that's Rosetta's bug and the program counts as unusable, for
nobody. On the leaderboard, a hard tab says what x3 does to everyone until some decompiler passes every
one of its round trips. `rosetta transform-check` checks all of that without a decompiler, over a tier's
programs at every profile and level; `rosetta compile --profile O2g1+x3` writes one rewrite out
to look at.

`bench` writes one JSON file per decompiler, `<name>-<tier>.json`. Each file lists every
program with one letter per profile (`p` pass, `w` wrong output, `c` doesn't recompile,
`f` crashed or timed out, `r` refused, `u` unusable input) and the first line of why a
program didn't pass. The file also has the section and overall scores, the totals, the
readability of its passes, the wall time and the suite.

### The failure gallery

`bench --gallery` also writes `<name>-<tier>.gallery.json`: for every round trip that went
wrong (wrong output, output that doesn't recompile, a crash; refusals already say why), the
program, the decompiler's output and the printed lines around where they part. With `--reduce`
each failing program is first shrunk to a smaller one the decompiler still gets wrong (each try
is a decompile, so that's for local decompilers, not rate-limited web APIs). The first 150
failures of each section are kept. `rosetta gallery <result.json> --decompiler ...` writes the
gallery for a result that already exists by rerunning only its failed round trips, and says
when one ends differently this time. On the website, a failed profile in a test's drawer opens
its gallery entry. The gallery is a separate file, so the result itself (what's hashed and
signed) stays exactly what bench wrote.

### The website

[site/](site) is a static site for GitHub Pages, in the spirit of sUNC's result pages. It shows
the leaderboard and a result viewer. Drop a result file on the viewer, or open one with
`result?r=<name>`, `?url=<link to a JSON file>` or a share link. The file is read in
the browser and never uploaded. The viewer puts every result in one of three states:

- **verified**: byte for byte an official result in [site/results/](site/results), whose
  SHA-256 is listed in `results/index.json`.
- **spoofed**: the file contradicts itself. Its scores or totals don't match its own
  verdicts, or it claims a known suite but lists other programs.
- **unverified**: anything else that parses. A result someone edited by hand while keeping
  it consistent also lands here. Without a server there's nothing to sign with, so only
  official results are proof.

Official runs can also be signed: `bench --sign` (or `rosetta sign FILE`) adds an ECDSA P-256
signature over the file as bench wrote it, with a private key that never leaves the machine of
whoever runs the official results. The site has the public half, and the viewer checks the
signature in the browser, so a signed result shows as verified wherever it's opened from (a
Rubiš link, a download) and changing a single byte of it breaks that.

To share a result that isn't official, `bench --upload` (or the viewer's "upload & copy link"
button) puts it on [Rubiš](https://rubis.app), a paste service with an open API, as a public
scrap, and gives a link that opens it in the viewer (`result?rubis=<id>`). That's sharing,
not proof: the viewer still says verified, unverified or spoofed the same way. Nothing is ever
uploaded unless you ask for it.

When the suite changes, the old suite's results move into a directory named after it
(`site/results/rosetta-1/`). They stay verified, and the leaderboard can still switch to them.

To publish an official result, copy it into `site/results/` and run `site/build-index.sh`
(it needs `jq`). The Pages workflow refuses to deploy if the index is stale. To preview
locally, serve site/ with something that maps `/result` to `result.html` the way GitHub Pages does, like `npx serve site` (or open `result.html` itself).

Once the site is live, the workflow posts every official result the push added or changed to
the Discord's score updates channel, with its rank and how it moved. A push that renames the
suite or changes the cases posts to the benchmark updates channel instead
([announce.yml](.github/workflows/announce.yml)), which can also send a message written by hand
from the Actions tab, to any of the three channels (general updates by default). The webhooks are
the repository secrets `DISCORD_WEBHOOK_SCORES`, `DISCORD_WEBHOOK_BENCHMARK` and
`DISCORD_WEBHOOK_SITE`; without them (in a fork) [announce.py](.github/announce.py)
prints what it would have sent.

## Results

The leaderboard and every official `bench` result are on the website,
[rosetta.wtf](https://rosetta.wtf/). The `bench` tables come first (suite rosetta-2, 41 cases, run
2026-11-25 on Luau 0.747, luarust 2026-11-26). The fuzz and check tables after them are older per-command runs
(rosetta-1's 35 cases, sunberry 0.5.4), which also compare readability across
decompilers on the round trips they all passed.

**The full tier** (`bench --tier full`: the 41 cases, seeds 1000..2000 and 1000 mutants of the
cases, 12,240 usable round trips each). Every result file is on [the website](https://rosetta.wtf/),
with each program's verdict at each profile:

| decompiler | pass | wrong | doesn't recompile | refused | cases | generated | mutants |
| --- | --- | --- | --- | --- | --- | --- | --- |
| sunberry 0.5.6 (new core) | 12,238 (99.98%) | 0 | 0 | 2 | 246/246 | 5,994/5,994 | 5,998/6,000 |
| luarust 2.5 | 12,236 (99.97%) | 0 | 0 | 4 | 246/246 | 5,994/5,994 | 5,996/6,000 |
| luacid | 12,201 (99.7%) | 37 | 0 | 2 | 244/246 | 5,990/5,994 | 5,967/6,000 |
| lua.expert | 12,132 (99.1%) | 4 | 0 | 104 | 246/246 | 5,886/5,994 | 6,000/6,000 |
| Composite | 11,923 (97.4%) | 76 | 0 | 241 | 246/246 | 5,873/5,994 | 5,804/6,000 |
| Tovek 2.7 | 11,843 (96.8%) | 4 | 0 | 393 | 244/246 | 5,772/5,994 | 5,827/6,000 |
| Fission 1.0.5-beta | 10,009 (81.8%) | 1,281 | 950 | 0 | 210/246 | 4,426/5,994 | 5,373/6,000 |
| sunberry (old core) | 8,991 (73.5%) | 2,932 | 317 | 0 | 200/246 | 3,685/5,994 | 5,106/6,000 |

sunberry's two refusals are mutant mut1-606 at O2g1 and O2g2 (`a condition that isn't an
expression`); luarust's four are mutant mut1-472 at O1 and O2 (HTTP 422, `This could not be
decompiled`). The quick tier (846 round trips) has the same order at the top: sunberry, luarust and
lua.expert pass all 846, luacid 843, Composite 834, Tovek 756, Fission 684 and the old core 584.

**The hard tiers** (the same programs with their bytecode rewritten; see [The hard tiers](#the-hard-tiers)).
x1, x2 and x3 each cover a third of the round trips. hard-full, 12,240 usable round trips each:

| decompiler | pass | x1 | x2 | x3 | wrong | doesn't recompile | refused |
| --- | --- | --- | --- | --- | --- | --- | --- |
| luarust 2.5 | 12,236 (99.97%) | 4,079/4,080 | 4,079/4,080 | 4,078/4,080 | 0 | 0 | 4 |
| luacid | 5,613 (45.9%) | 3,726/4,080 | 1,391/4,080 | 496/4,080 | 577 | 5 | 6,045 |
| lua.expert | 3,852 (31.5%) | 3,722/4,080 | 128/4,080 | 2/4,080 | 1 | 0 | 8,387 |
| Tovek 2.7 | 3,784 (30.9%) | 3,598/4,080 | 179/4,080 | 7/4,080 | 1 | 0 | 8,455 |
| Fission 1.0.5-beta | 2,995 (24.5%) | 2,954/4,080 | 25/4,080 | 16/4,080 | 8,442 | 803 | 0 |
| Composite | 2,919 (23.8%) | 2,145/4,080 | 752/4,080 | 22/4,080 | 722 | 0 | 8,599 |
| sunberry (old core) | 1,317 (10.8%) | 933/4,080 | 372/4,080 | 12/4,080 | 10,887 | 36 | 0 |
| sunberry 0.5.6 (new core) | 379 (3.1%) | 372/4,080 | 5/4,080 | 2/4,080 | 2 | 0 | 11,859 |

hard-quick, 846 round trips each:

| decompiler | pass | x1 | x2 | x3 | wrong | doesn't recompile | refused |
| --- | --- | --- | --- | --- | --- | --- | --- |
| luarust 2.5 | 846 (100%) | 282/282 | 282/282 | 282/282 | 0 | 0 | 0 |
| luacid | 344 (40.7%) | 239/282 | 77/282 | 28/282 | 30 | 0 | 472 |
| lua.expert | 256 (30.3%) | 240/282 | 16/282 | 0/282 | 0 | 0 | 590 |
| Tovek 2.7 | 223 (26.4%) | 207/282 | 16/282 | 0/282 | 0 | 0 | 623 |
| Fission 1.0.5-beta | 216 (25.5%) | 203/282 | 7/282 | 6/282 | 579 | 51 | 0 |
| Composite | 213 (25.2%) | 142/282 | 69/282 | 2/282 | 41 | 0 | 592 |
| sunberry (old core) | 73 (8.6%) | 54/282 | 17/282 | 2/282 | 770 | 3 | 0 |
| sunberry 0.5.6 (new core) | 35 (4.1%) | 34/282 | 1/282 | 0/282 | 0 | 0 | 811 |

luarust is the first to clear the hard tiers: its hard-full score equals its full score, with the
same four refusals and nothing wrong, and it passes all of hard-quick. Behind it, luacid gets the
furthest by a long way, but it pays for it in wrong output (577 on hard-full), where lua.expert and
Tovek mostly refuse what they can't follow.
sunberry's new core refuses nearly everything past x1: it won't print code it can't prove matches.

The per-command runs were on 2026-11-22 (Luau 0.747), with seven decompilers: sunberry 0.5.4 (its new core with `--strict`, and
its original core with `--core old`), Tovek 2.7, Fission 1.0.5-beta, and the three web APIs lua.expert,
Composite and luacid. Fission runs as its own HTTP server on this machine; it and the web APIs go
through the scripts in [adapters/](adapters), all with default options (Composite with telemetry
off, so it keeps none of the programs). All seven got the same bytecode.

**Generated programs** (`fuzz --seeds 1000..1200`, 1,200 round trips each):

| decompiler | pass | wrong | doesn't recompile | refused |
| --- | --- | --- | --- | --- |
| sunberry (new core) | 1,200 (100%) | 0 | 0 | 0 |
| luacid | 1,196 (99.7%) | 4 | 0 | 0 |
| lua.expert | 1,172 (97.7%) | 4 | 0 | 24 |
| Composite | 1,168 (97.3%) | 9 | 0 | 23 |
| Tovek 2.7 | 1,142 (95.2%) | 4 | 0 | 54 |
| Fission | 915 (76.3%) | 181 | 104 | 0 |
| sunberry (old core) | 722 (60.2%) | 408 | 70 | 0 |

**Hand-written cases** (`check`, 35 programs x 6 profiles = 210 round trips each):

| decompiler | pass | wrong | doesn't recompile | refused |
| --- | --- | --- | --- | --- |
| sunberry (new core) | 210 | 0 | 0 | 0 |
| lua.expert | 210 | 0 | 0 | 0 |
| Composite | 210 | 0 | 0 | 0 |
| luacid | 210 | 0 | 0 | 0 |
| Tovek 2.7 | 208 | 0 | 0 | 2 |
| Fission | 183 | 21 | 6 | 0 |
| sunberry (old core) | 176 | 28 | 6 | 0 |

What the wrong ones were:
- **luacid, lua.expert and Tovek** (4 each): seed 1077, where `x^3` on a run-time value prints
  `-0.31840539825142244` instead of `-0.31840539825142256`. Luau computes `x^3` at run time
  as `x*x*x` but folds a constant `^` with `pow()`; writing the value back as a constant
  changes the last digits.
- **Composite** (9): the same seed 1077 twice, and seeds 1018, 1034, 1106 and 1188. On 1018 a
  comparison that printed `true` prints `false`; on 1106 a variable that holds `nil` comes back
  holding a function (the program prints `fn` where it printed `nil`); on 1188 the decompiled
  program stops two lines early.
- **Fission** (181, and 104 that don't recompile): 115 print different values (`2.5 3.5 6.5` comes
  back as `3.5 4.5 8.5`, `11 12 31` as `31 32 33`), 66 stop early, and the outputs that don't compile mostly call a method on a literal without the
  parentheses Luau needs (`` `{v}`:rep(2) ``). On the hand-written cases it also turns `Class.new(x)`
  into `function Class:new()` building `{ self = self }` at debug level 2.

lua.expert's refusals are `Provided bytecode is not supported`; Composite's are internal
errors (`readable HIR preparation introduced a nonstructural node`, `structure recovery could
not represent every prototype`). luacid and Fission never refuse: they always print something.

### How the output reads

Run on 2026-11-22 (Luau 0.747). Every table only covers the round trips all of its decompilers
passed, so compare rows within a table, never across tables. CPU time isn't given for the web
APIs or Fission, because their adapters only measure the local `curl` (Fission decompiles inside
its server process).

**Hand-written cases** (`check`, 154 round trips all seven passed):

| decompiler | similarity | size | locals | nesting | endless | names g2 | cpu ms |
| --- | --- | --- | --- | --- | --- | --- | --- |
| sunberry (new core) | 91.5% | 0.97x | 1.00x | -0.18 | 8 | 98.6% | 4.9 |
| Tovek 2.7 | 89.4% | 0.93x | 0.96x | -0.25 | 14 | 96.6% | 7.5 |
| Composite | 87.0% | 1.02x | 1.11x | -0.27 | 14 | 96.6% | - |
| lua.expert | 86.9% | 0.96x | 0.97x | -0.05 | 37 | 56.5% | - |
| luacid | 86.7% | 0.98x | 1.00x | -0.15 | 14 | 98.0% | - |
| Fission | 84.0% | 1.03x | 1.18x | +0.06 | 10 | 88.1% | - |
| sunberry (old core) | 83.8% | 0.95x | 0.92x | -0.12 | 35 | 78.4% | 6.2 |

**Generated programs** (`fuzz --seeds 1000..1200`, 512 round trips all seven passed):

| decompiler | similarity | size | locals | nesting | endless | names g2 | cpu ms |
| --- | --- | --- | --- | --- | --- | --- | --- |
| sunberry (new core) | 72.1% | 0.64x | 0.77x | -1.39 | 14 | 77.8% | 6.6 |
| Composite | 67.6% | 0.66x | 0.89x | -1.08 | 145 | 72.2% | - |
| luacid | 66.9% | 0.59x | 0.63x | -1.40 | 49 | 48.3% | - |
| Tovek 2.7 | 65.5% | 0.56x | 0.56x | -1.66 | 10 | 65.1% | 8.4 |
| Fission | 64.3% | 0.68x | 0.85x | -0.53 | 17 | 66.6% | - |
| lua.expert | 64.3% | 0.55x | 0.50x | -1.35 | 252 | 35.7% | - |
| sunberry (old core) | 58.8% | 0.52x | 0.49x | -1.68 | 220 | 38.6% | 7.5 |

Each result page on the website also shows how one decompiler's own passes read, over its full
tier run.

Generated programs come out much smaller than their source because the generator writes
`do ... end` blocks and constant expressions that Luau's compiler folds away. No decompiler can
bring those back, which is why every row's size is well under 1x there.

Rosetta started inside sunberry, and its author also writes sunberry, so it's fair to be
suspicious of these numbers: everything needed to rerun them is here, and cases that break
sunberry are just as welcome as any other.

## License

MIT, see [LICENSE](LICENSE).
