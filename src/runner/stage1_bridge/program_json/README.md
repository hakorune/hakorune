# Stage1 Bridge Program JSON

Scope: future-retire bridge-only `emit-program-json-v0` helpers under `src/runner/stage1_bridge/program_json/`.

## Responsibility Split

- `mod.rs`
  - thin facade for the bridge-local Program(JSON v0) emit route
  - delegates source-path precedence, source-text read, bridge-local payload emission, and writeback policy out of the facade
- `source.rs`
  - source-path precedence for `emit-program-json-v0`
  - `stage1::input_path()` aliases first, CLI input fallback second
- `read_input.rs`
  - source file read policy for `emit-program-json-v0`
  - exact bridge-local read error formatting stays here
- `emit_payload.rs`
  - bridge-local payload emission via `emit_program_json_v0_for_stage1_bridge_emit_program_json(...)`
  - exact bridge error prefix contract stays here
- `writeback.rs`
  - bridge-local file writeback policy for `emit-program-json-v0`

## Guardrails

- keep this lane future-retire only
- do not add parse/lower policy here
- do not bypass `emit_program_json_v0_for_stage1_bridge_emit_program_json(...)`
