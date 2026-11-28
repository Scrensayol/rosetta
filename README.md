# Rosetta

A standardized, open-source benchmark for Luau decompilers.

Rosetta compiles a program, runs it, hands the bytecode to a decompiler, recompiles what comes
back and runs that. If the two runs do the same thing, it passes. Leaderboard and every
official result: [rosetta.wtf](https://rosetta.wtf/). News: [Discord](https://discord.gg/7qE27wRwnd).

## Grading

Every program is compiled at six profiles (`-O0`, `-O1`, `-O2`, each at debug level 1 and 2),
and every decompiler gets the same bytes.

| verdict | meaning |
| --- | --- |
| pass | recompiles and does exactly what the original did: see below |
| wrong | recompiles but behaves differently: the worst outcome |
| doesn't recompile | the output isn't valid Luau |
| refused | a clean non-zero exit: no output, but no wrong output either |
| failed | crashed, or ran past 30 seconds |
| unusable | the input itself doesn't finish; it counts for nobody |

"Does the same thing" means four things match: everything it prints, in order; how it ends
(returns, or errors with which message, or runs out of time); every global it creates, changes
or removes, and every field it changes in a table that was already there (the standard library
included), compared all the way down; and what the main chunk returns. Line numbers in error
messages and heap addresses are normalized away, since a faithful decompile can't keep them.

Passes also get a stricter, separate check: whether the output made the same calls that change
things (`table.insert`, `table.remove`, `table.sort`, `setmetatable`, `rawset`, coroutine
switches, buffer writes) with the same arguments, in the same order. A program can get the same
result through different calls, so this is reported beside the score as "same calls" and doesn't
change it. Pure functions aren't tracked, since the compiler folds some of them away.

The score is passes out of usable round trips. Wrong output is reported on its own, because
refusing 1% is far better than being quietly wrong 1% of the time. Passes are also measured
against the source for readability (token similarity, size, locals, nesting, added endless loops,
recovered names). No model is in the loop.

## Programs

- **cases/**: hand-written programs aimed at what decompilers get wrong: inlining, loop exits,
  upvalues, NaN and number edges, precedence, deep nesting, metamethods, `getfenv`, type
  syntax, register limits and more. New edge cases are welcome.
- **libraries/**: real open-source libraries (middleclass, inspect, json.lua, lume, tween.lua,
  LibDeflate). Only the library is decompiled, then its own test suite runs against the output.
  See [libraries/README.md](libraries/README.md).
- **generated**: random programs built from a seed.
- **mutants**: the cases mutated over and over, keeping only the ones whose bytecode shows
  something new.

## Running it

A decompiler is a command: `name=command`, where `{}` is a raw bytecode file and the source comes
out on stdout. Web APIs get a small script in [adapters/](adapters).

```sh
cargo build --release
./target/release/rosetta bench --decompiler "mine=/path/to/decompiler {}" --version mine=1.0
./target/release/rosetta bench --tier full --decompiler "mine=/path/to/decompiler {}" --version mine=1.0
```

| tier | programs | round trips |
| --- | --- | --- |
| quick | the 42 cases, 6 libraries, and the 100 generated programs and mutants with the most bytecode features | 888 |
| full | the cases, libraries, 1000 generated programs and 1000 mutants | 12,282 |
| hard-quick / hard-full | the same programs with their bytecode rewritten first | 888 / 12,282 |
| obfuscated | quick's programs, obfuscated as source first | 888 |

`bench` writes `<name>-<tier>.json` with every program's verdict at every profile.
`--options name=...` records the settings a decompiler ran with, which the result page shows. `--gallery`
also writes `<name>-<tier>.gallery.json`, with the source, output and where they differ for every
failure, which the website shows in a failed test's "see why" panel.

Other commands: `check` (cases only), `fuzz`, `mutate`, `gen`, `compile`, `transform-check`,
`library-check`, `gallery`, plus `--reduce` to shrink a failing program and
`--baseline` for CI. `rosetta help` lists them all.

The suite is pinned as `rosetta-4` (Luau 0.747): which programs run and how is hashed, so two
results with the same hash ran the same programs.

### The hard tiers

Roblox bytecode always comes from Roblox's compiler, so a decompiler can lean on its patterns.
The hard tiers rewrite the bytecode so it does exactly the same thing but looks different
([src/transform.rs](src/transform.rs)), and every rewrite is checked to print exactly what
the original did. They measure real control-flow and data-flow analysis, and are scored apart
from quick and full.

| level | what changes |
| --- | --- |
| x1 | shifted registers, no-ops, jumps to the next instruction, unreachable junk |
| x2 | + shuffled registers, equivalent instruction swaps, shuffled basic blocks, no local debug info |
| x3 | + opaque always-taken branches guarding junk, shuffled functions, no upvalue names |

### The obfuscated tier

Scripts in the wild are often obfuscated as source before they're ever compiled. The obfuscated
tier does the same to quick's programs: each is lowered to Lua 5.1 with
[darklua](https://github.com/seaofvoices/darklua), then obfuscated by
[Prometheus](https://github.com/prometheus-lua/Prometheus), an open-source obfuscator, from a
fixed seed ([src/obfuscate.rs](src/obfuscate.rs)). Both run inside Rosetta, so nothing has to be
installed. The decompiler gets the obfuscated program's bytecode and has to give back something
that still prints what it printed: nobody has to deobfuscate anything, but the obfuscator's VM,
decoders and flattened control flow all have to survive. A program the obfuscator breaks counts
as unusable: in rosetta-4, 140, 135 and 135 of the 148 survive o1, o2 and o3, and o2 and o3 turn
a program of a few kilobytes into about 780 KB. `rosetta obfuscate-check` shows how many survive at each level, and
`rosetta obfuscate <file> --level N` prints what a program becomes.

| level | Prometheus preset |
| --- | --- |
| o1 | Weak: flattened control flow, a constant array |
| o2 | Medium: a VM, encrypted strings, flattening outside the VM, numbers as expressions |
| o3 | Strong: all of the above, flattening inside the VM too |

Prometheus's anti-tamper step is left out, since it trips on any change to the program's line
layout, which every decompiler makes. So is Strong's number representation mutation, which breaks
nearly every program under Luau. Based on Prometheus by Elias Oelschner,
https://github.com/prometheus-lua/Prometheus (vendored in [vendor/prometheus](vendor/prometheus)).

## Results

Suite rosetta-4, run 2026-11-27 and 28. Round trips passed (obfuscated is out of about 818
usable, the others out of 888 and 12,282):

| decompiler | quick | full | hard-quick | hard-full | obfuscated | wrong (full / hard-full / obfuscated) |
| --- | --- | --- | --- | --- | --- | --- |
| luarust 2.5 | 888 | 12,281 | 888 | 12,281 | 603 | 0 / 1 / 211 |
| sunberry 0.5.7 | 888 | 12,273 | 876 | 12,182 | 47 | 9 / 17 / 0 |
| Composite rust | 887 | 12,262 | 887 | 12,259 | 324 | 6 / 6 / 2 |
| luacid | 886 | 12,244 | 628 | 8,868 | 361 | 38 / 1,081 / 20 |
| Composite luau | 841 | 12,239 | 864 | 12,234 | 609 | 25 / 29 / 0 |
| lua.expert | 888 | 12,081 | 288 | 3,953 | 665 | 8 / 2 / 0 |
| Tovek 2.7 | 868 | 12,046 | 277 | 3,987 | 761 | 4 / 1 / 0 |
| Oracle v2 | 864 | 11,945 | 549 | 7,955 | 796 | 337 / 1,167 / 0 |
| sunberry (old core) | 652 | 9,177 | 120 | 1,579 | 0 | 3,105 / 10,703 / 817 |
| Fission 1.0.5-beta | 597 | 8,565 | 172 | 2,497 | 444 | 3,717 / 9,785 / 362 |

"wrong" counts wrong output plus output that doesn't recompile: luarust's 211 on the obfuscated
tier don't recompile, Fission's and the old core's are mostly wrong. rosetta-4 grades what a
program leaves behind as well as what it prints, which is most of why Fission and the old core
fell from rosetta-3. Oracle gets the most of the obfuscated tier and nothing wrong there; sunberry
fails nearly all of it (crashes and timeouts on the obfuscators' VMs) but isn't wrong either.
Composite runs as two rows, one per backend (`?backend=luau` or `rust`), each result naming the
Composite build it ran on (Composite redeployed during the luau runs, so luau's hard-full ran on
a newer build than its other three tiers). Per-test verdicts, failures, readability and "same calls" for every run
are on [the website](https://rosetta.wtf/), and rosetta-3's results are archived there.

Rosetta started inside sunberry and shares its author, so be suspicious: everything needed to
rerun these numbers is here, and cases that break sunberry are as welcome as any other.

## Website

[site/](site) is the static GitHub Pages site. The result viewer marks a file **verified** (an
official result, by SHA-256 or signature), **spoofed** (its numbers contradict themselves) or
**unverified**. To publish an official result, copy it into `site/results/` and run
`site/build-index.sh`. Older suites' results live in `site/results/<suite>/`.

## License

MIT, see [LICENSE](LICENSE).
