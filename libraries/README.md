# Libraries

Each directory is one library case: `<name>.luau` is the library exactly as upstream ships it,
`spec.luau` is its upstream test suite (LibDeflate's is Rosetta's own, see below) and `LICENSE`
is its license. Rosetta compiles and
decompiles only the library. The spec runs after it in the same VM, behind the busted shim in
`src/busted.luau`, and reaches the library through `require("<name>")`. Each test prints one
line, so the comparison names the test that broke.

`rosetta library-check` vets a library: its spec has to pass in full against the compiler's own
bytecode, the same way at every profile. A spec that depends on garbage collection timing or
heap addresses can't be compared across decompilers, so those tests are left out.

| library | upstream | commit | license | tests |
|---|---|---|---|---|
| inspect.lua 3.1.0 | [kikito/inspect.lua](https://github.com/kikito/inspect.lua) | a8ca312 | MIT | 48 |
| json.lua 0.1.2 | [rxi/json.lua](https://github.com/rxi/json.lua) | dbf4b2d | MIT | 14 |
| LibDeflate 1.0.2 | [SafeteeWoW/LibDeflate](https://github.com/SafeteeWoW/LibDeflate) | afc3b78 | zlib | 159 (Rosetta's own) |
| lume 2.3.0 | [rxi/lume](https://github.com/rxi/lume) | 98847e7 | MIT | 257 assertions |
| middleclass 4.1.1 | [kikito/middleclass](https://github.com/kikito/middleclass) | 359f0e2 | MIT | 118 |
| tween.lua 2.1.1 | [kikito/tween.lua](https://github.com/kikito/tween.lua) | 9cdcd39 | BSD-3-Clause | 51 |

Library cases get a bigger step budget than generated programs (40 million interrupts, not 2
million), since LibDeflate compressing a few kilobytes at every level does real work.

## What was ported

The libraries themselves are untouched. In the specs:

- **middleclass**: the spec files are joined into one, each in a `do ... end` block.
  `debug.getmetatable(v)` (not in Luau) became `v.class.__instanceDict`, which is the same
  table. The Lua 5.2 and 5.3 metamethod specs are replaced by a Luau one: `__len` stays,
  `__idiv` and `__iter` stand in for `__pairs`, `__ipairs`, `__gc` and the bitwise operators,
  which Luau doesn't have.
- **json.lua**: upstream's `test/test.lua` with its own runner. Only `loadfile("../json.lua")()`
  became `require("json")`.
- **lume**: `test/test.lua` and its runner `test/util/tester.lua`. The runner prints each result
  by its line in the test chunk instead of reading the source line through `io`, and finds that
  line with `debug.info` instead of `debug.getinfo`; colors are gone. The `lume.trace` test
  (needs `debug.getinfo`) and the `lume.hotswap` test (needs `package`) are left out.
- **tween.lua**: the userdata subject comes from `newproxy()` instead of `io.input()`.
- **LibDeflate**: upstream's tests need `io`, files on disk and luaunit, so `spec.luau` is
  Rosetta's own. Eight fixed inputs (built in the spec, no `math.random`) go through every level,
  every strategy, zlib, a preset dictionary and every encoding, and each compressed result is
  printed with its length and Adler-32. A decompile that still round-trips but compresses
  differently shows up that way. Truncated and corrupted streams and bad arguments are checked
  too.
- **inspect.lua**: `spec/unindent.lua` is inlined. "does not allow collecting weak tables while
  they are being inspected" is left out: it needs `collectgarbage('stop')`, which Luau doesn't
  have, and without it the result depends on when the collector runs.
