# Stage1 Bridge Program JSON

Scope: future-retire bridge-only `emit-program-json-v0` helpers under `src/runner/stage1_bridge/program_json/`.

## Responsibility Split

- `mod.rs`
  - thin facade for the bridge-local Program(JSON v0) emit route
  - delegates to `orchestrator.rs`
- `orchestrator.rs`
  - owns the bridge-local `ProgramJsonOutput` handoff object
  - owns source-path/source-text->emit->write orchestration
- source-path precedence stays in `program_json_entry/request.rs`
- `read_input.rs`
  - source file read policy for `emit-program-json-v0`
  - exact bridge-local read error formatting stays here
- `payload.rs`
  - owner-1 payload emission via `emit_program_json_v0_for_stage1_bridge_emit_program_json(...)`
  - keeps bridge-local source-text -> Program(JSON v0) contract outside the facade
- `writeback.rs`
  - bridge-local file writeback policy for `emit-program-json-v0`

## Guardrails

- keep this lane future-retire only
- do not add parse/lower policy here
- do not bypass `emit_program_json_v0_for_stage1_bridge_emit_program_json(...)` from `mod.rs`
- next Rust-only retire slices may stay inside this cluster; do not widen them into `src/runner/mod.rs` or `src/runner/emit.rs`
