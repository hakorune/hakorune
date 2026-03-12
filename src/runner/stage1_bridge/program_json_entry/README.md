# Stage1 Bridge Program JSON Entry

Scope: future-retire bridge-only `emit-program-json-v0` entry helpers under `src/runner/stage1_bridge/program_json_entry/`.

## Responsibility Split

- `mod.rs`
  - thin entry facade for the bridge-local `emit-program-json-v0` route
  - delegates request classification/building to `request.rs`
  - delegates success/error process-exit formatting to `exit.rs`
- `request.rs`
  - bridge-entry request building for `emit-program-json-v0`
  - owns the explicit request predicate used by outer callers
  - owns source-path precedence (`stage1::input_path()` aliases first, CLI input fallback second)
  - owns out-path extraction from the explicit CLI flag
- `exit.rs`
  - exact bridge-entry success/error process-exit formatting for `emit-program-json-v0`

## Guardrails

- keep this lane future-retire only
- outer callers should use the `program_json_entry` module helpers directly
- do not reintroduce bridge-local source-path precedence or emit-path extraction into `runner/mod.rs` or `runner/emit.rs`
