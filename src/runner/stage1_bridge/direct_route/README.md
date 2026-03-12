# Stage1 Bridge Direct Route

Scope: binary-only direct route helpers under `src/runner/stage1_bridge/direct_route/`.

## Sections

- `mod.rs`: direct-route facade (`emit_mir_binary_only_direct(...)` / `run_binary_only_direct(...)`)
- `compile.rs`: source read, parse, macro expand, MIR compile, optional dump
- `emit.rs`: MIR JSON write only
- `../emit_paths.rs`: shared MIR / Program(JSON) output-path resolution

## Forbidden

- stage1 stub route orchestration here
- route-to-executor dispatch here
- child env policy here
- `Program(JSON v0)` compatibility policy here
