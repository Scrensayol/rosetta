# Libraries

Each directory is one library case: `<name>.luau` is the library exactly as upstream ships it,
`spec.luau` is its upstream test suite and `LICENSE` is its license. Rosetta compiles and
decompiles only the library. The spec runs after it in the same VM, behind the busted shim in
`src/busted.luau`, and reaches the library through `require("<name>")`. Each test prints one
line, so the comparison names the test that broke.

`rosetta library-check` vets a library: its spec has to pass in full against the compiler's own
bytecode, the same way at every profile. A spec that depends on garbage collection timing or
heap addresses can't be compared across decompilers, so those tests are left out.

| library | upstream | commit | license | tests |
|---|---|---|---|---|
| middleclass 4.1.1 | [kikito/middleclass](https://github.com/kikito/middleclass) | 359f0e2 | MIT | 118 |
| inspect.lua 3.1.0 | [kikito/inspect.lua](https://github.com/kikito/inspect.lua) | a8ca312 | MIT | 48 |

## What was ported

The libraries themselves are untouched. In the specs:

- **middleclass**: the spec files are joined into one, each in a `do ... end` block.
  `debug.getmetatable(v)` (not in Luau) became `v.class.__instanceDict`, which is the same
  table. The Lua 5.2 and 5.3 metamethod specs are replaced by a Luau one: `__len` stays,
  `__idiv` and `__iter` stand in for `__pairs`, `__ipairs`, `__gc` and the bitwise operators,
  which Luau doesn't have.
- **inspect.lua**: `spec/unindent.lua` is inlined. "does not allow collecting weak tables while
  they are being inspected" is left out: it needs `collectgarbage('stop')`, which Luau doesn't
  have, and without it the result depends on when the collector runs.
