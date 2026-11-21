# Rosetta

A standardized, open-source benchmark for Luau decompilers.

Every decompiler claims to be accurate. Rosetta checks it the only way that can't be argued with:
it compiles a program, runs it, hands the bytecode to the decompiler, compiles what comes back,
runs that, and compares the two runs. Same output, it passes. Anything else is graded.

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
  `if local` / `while local`, Roblox-style scripts. New edge cases are welcome.
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
gets an advantage.

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

## Results

Run on 2026-11-21 (Luau 0.747) with five decompilers: sunberry 0.5.1 (its new core with `--strict`, and its
original core with `--core old`), Tovek 2.7, and the two web APIs lua.expert and Composite through
the scripts in [adapters/](adapters) (default options; Composite with telemetry off, so it keeps
none of the programs). All five got the same bytecode.

**Generated programs** (`fuzz --seeds 1000..1200`, 1,200 round trips each):

| decompiler | pass | wrong | doesn't recompile | refused |
| --- | --- | --- | --- | --- |
| sunberry (new core) | 1,198 (99.8%) | 0 | 0 | 2 |
| lua.expert | 1,172 (97.7%) | 4 | 0 | 24 |
| Composite | 1,170 (97.5%) | 7 | 0 | 23 |
| Tovek 2.7 | 1,142 (95.2%) | 4 | 0 | 54 |
| sunberry (old core) | 722 (60.2%) | 408 | 70 | 0 |

**Hand-written cases** (`check`, 26 programs x 6 profiles = 156 round trips each):

| decompiler | pass | wrong | doesn't recompile | refused |
| --- | --- | --- | --- | --- |
| sunberry (new core) | 156 | 0 | 0 | 0 |
| lua.expert | 156 | 0 | 0 | 0 |
| Composite | 156 | 0 | 0 | 0 |
| Tovek 2.7 | 154 | 0 | 0 | 2 |
| sunberry (old core) | 138 | 12 | 6 | 0 |

What the wrong ones were:
- **lua.expert and Tovek** (4 each): seed 1077, where `x^3` on a run-time value prints
  `-0.31840539825142244` instead of `-0.31840539825142256`. Luau computes `x^3` at run time
  as `x*x*x` but folds a constant `^` with `pow()`; writing the value back as a constant
  changes the last digits.
- **Composite** (7): the same seed 1077 twice, and seeds 1034, 1106 and 1188. On 1106 a
  variable that holds `nil` comes back holding a function (the program prints `fn` where it
  printed `nil`); on 1188 the decompiled program stops two lines early.

lua.expert's refusals are `Provided bytecode is not supported`; Composite's are internal
errors (`readable HIR preparation introduced a nonstructural node`, `structure recovery could
not represent every prototype`).

A larger run of the three local decompilers (`--seeds 1000..2000`, 5,994 usable round trips):
sunberry's new core 99.9% with 0 wrong, Tovek 96.3% with 4 wrong, sunberry's old core 61.5%.

**Mutants** (`mutate --count 1000 --seed 1`, 1,000 mutants of the cases, 6,000 round trips each).
These are local decompilers only, because a mutation run is thousands of requests, more than
it's fair to send someone else's server:

| decompiler | pass | wrong | doesn't recompile | refused |
| --- | --- | --- | --- | --- |
| Tovek 2.7 | 5,966 (99.4%) | 0 | 0 | 34 |
| sunberry (new core) | 5,964 (99.4%) | 0 | 0 | 36 |
| sunberry (old core) | 5,698 (95.0%) | 254 | 48 | 0 |

Tovek's refusals are `residual goto/label would be invalid Luau`; sunberry's are a register
read before it's written (`mut1-103` at `-O0`, for one).

### How the output reads

Run on 2026-11-22 (Luau 0.747). Every table only covers the round trips all of its decompilers
passed, so compare rows within a table, never across tables. CPU time isn't given for the web
APIs, because their adapters only measure the local `curl`.

**Hand-written cases** (`check`, 138 round trips all five passed):

| decompiler | similarity | size | locals | nesting | endless | names g2 | cpu ms |
| --- | --- | --- | --- | --- | --- | --- | --- |
| sunberry (new core) | 92.3% | 0.98x | 1.00x | -0.19 | 8 | 98.9% | 4.1 |
| Tovek 2.7 | 90.3% | 0.94x | 0.95x | -0.33 | 14 | 97.5% | 4.0 |
| Composite | 87.5% | 1.02x | 1.10x | -0.30 | 11 | 97.7% | - |
| lua.expert | 86.9% | 0.98x | 1.02x | -0.07 | 41 | 58.8% | - |
| sunberry (old core) | 83.1% | 0.99x | 0.98x | -0.13 | 38 | 83.4% | 4.7 |

**Generated programs** (`fuzz --seeds 1000..1200`, 674 round trips all five passed):

| decompiler | similarity | size | locals | nesting | endless | names g2 | cpu ms |
| --- | --- | --- | --- | --- | --- | --- | --- |
| sunberry (new core) | 72.5% | 0.65x | 0.78x | -1.38 | 20 | 78.4% | 6.7 |
| Composite | 68.0% | 0.66x | 0.90x | -1.07 | 187 | 73.1% | - |
| Tovek 2.7 | 66.2% | 0.57x | 0.57x | -1.65 | 16 | 66.2% | 8.5 |
| lua.expert | 64.8% | 0.56x | 0.51x | -1.38 | 343 | 36.2% | - |
| sunberry (old core) | 59.5% | 0.52x | 0.50x | -1.69 | 318 | 39.7% | 7.5 |

**Mutants** (`mutate --count 1000 --seed 1`, 5,664 round trips all three local decompilers
passed; the web APIs aren't sent mutation runs):

| decompiler | similarity | size | locals | nesting | endless | names g2 | cpu ms |
| --- | --- | --- | --- | --- | --- | --- | --- |
| sunberry (new core) | 91.6% | 0.99x | 1.04x | -0.55 | 546 | 91.7% | 6.2 |
| Tovek 2.7 | 90.5% | 0.94x | 0.96x | -0.78 | 580 | 95.6% | 9.4 |
| sunberry (old core) | 81.8% | 0.93x | 0.87x | -0.62 | 1,492 | 76.5% | 8.0 |

Generated programs come out much smaller than their source because the generator writes
`do ... end` blocks and constant expressions that Luau's compiler folds away. No decompiler can
bring those back, which is why every row's size is well under 1x there.

Rosetta started inside sunberry, and its author also writes sunberry, so it's fair to be
suspicious of these numbers: everything needed to rerun them is here, and cases that break
sunberry are just as welcome as any other.

## License

MIT, see [LICENSE](LICENSE).
