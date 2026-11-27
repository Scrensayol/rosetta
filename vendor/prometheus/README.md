# Prometheus (vendored)

Based on Prometheus by Elias Oelschner, https://github.com/prometheus-lua/Prometheus

These are the library files of Prometheus, an open-source Lua obfuscator, copied unchanged from
commit `a98c50165f2a53f6d765a82eee60dfea53803445` (2026-11-09): `src/prometheus/`, and
`presets.lua`, `logger.lua`, `colors.lua` and `config.lua` from `src/`. Its license is
[LICENSE](LICENSE) beside this file.

Rosetta's obfuscated tier runs them inside its own Luau VM (see `src/obfuscate.rs`), so nothing
has to be installed. Moving to another Prometheus commit changes what every obfuscated program is:
bump `obfuscate::VERSION` with it.
