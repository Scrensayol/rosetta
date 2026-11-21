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
  keywords (`while continue do`), dead branches, Roblox-style scripts. New edge cases are welcome.
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

The Luau compiler and VM come from `mlua` 0.12.2 / `luau0-src` 0.22.0, pinned, because the
Luau version decides what the programs compile to.

## Results

Run on 2026-11-21: sunberry 0.5.0 (its new core, `--strict`, and its original core with
`--core old`), Tovek 2.7.

**Generated programs** (`fuzz --seeds 1000..2000`, 5,994 usable round trips each):

| decompiler | pass | wrong | doesn't recompile | refused |
| --- | --- | --- | --- | --- |
| sunberry (new core) | 5,986 (99.9%) | 0 | 0 | 8 |
| Tovek 2.7 | 5,772 (96.3%) | 4 | 0 | 218 |
| sunberry (old core) | 3,685 (61.5%) | 2,043 | 266 | 0 |

**Hand-written cases** (`check`, 24 programs x 6 profiles):

| decompiler | pass | wrong | refused |
| --- | --- | --- | --- |
| sunberry (new core) | 144 | 0 | 0 |
| Tovek 2.7 | 142 | 0 | 2 |
| sunberry (old core) | 136 | 8 | 0 |

Rosetta started inside sunberry, and its author also writes sunberry, so it's fair to be
suspicious of these numbers: everything needed to rerun them is here, and cases that break
sunberry are just as welcome as any other.

## License

MIT, see [LICENSE](LICENSE).
