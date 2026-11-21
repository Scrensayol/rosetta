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

## Programs

- **cases/**: hand-written programs, each aimed at something decompilers get wrong: inlined
  functions, loop exits, closures and upvalues, numbers (NaN sign, `x^3` folding), contextual
  keywords (`while continue do`), dead branches, `if`s nested 150 deep, Luau's experimental
  `if local` / `while local`, Roblox-style scripts. New edge cases are welcome.
- **generated**: `fuzz` builds random programs from a seed with a grammar (`full`, `straight`
  for straight-line code, `branches` for no loops), so a run is reproducible from its seed range.

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

Rosetta started inside sunberry, and its author also writes sunberry, so it's fair to be
suspicious of these numbers: everything needed to rerun them is here, and cases that break
sunberry are just as welcome as any other.

## License

MIT, see [LICENSE](LICENSE).
