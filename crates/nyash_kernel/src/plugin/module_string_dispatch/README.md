# Stage1 Module Dispatch

Status: compiled-stage1 route table and route-local probes.

Scope: compiled-stage1 string-module dispatch helpers under
`crates/nyash_kernel/src/plugin/module_string_dispatch/`.

## Placement Rule

- this directory owns the compiled-stage1 route table and its route-local probes
- keep route ownership local; do not widen it beyond the local routes

## Responsibility Split

- `module_string_dispatch.rs`
  - thin route table and route-local probe for using_resolver and MIR-builder routes
  - local MIR-builder gate and freeze wrappers for the source and Program(JSON) routes
  - does not own general string-handle encode/decode routes
- `build_surrogate.rs`
  - compiled-stage1 `BuildBox.emit_program_json_v0` route helper only
  - owner of the route match/dispatch contract, typed source decode,
    and encoded result handoff
  - build-box / launcher handoff regression coverage lives in
    `src/stage1/program_json_v0.rs` tests
  - parent modules probe it via `try_dispatch(...)`
- `compat/llvm_backend_surrogate.rs`
  - compiled-stage1 `selfhost.shared.backend.llvm_backend::{compile_obj,link_exe}` route helper only
  - owner of the backend route match/dispatch contract and its regression coverage
  - compile side loads MIR(JSON) locally and materializes the returned text handle locally; the helper owns only the file-path wrapper contract
  - path decode / compile opts / link arg decode stay behind local helpers; parent modules probe it via `try_dispatch(...)`

## Retirement Note

- do not mix `build_surrogate.rs` removal with `stage1_bridge` or `.hako`
  live/bootstrap caller deletion
- if a helper still cannot be removed, record that retreat in the current
  module-dispatch inventory note
- treat the shared `emit_from_program_json_v0` / `emit_from_source_v0`
  gate-decode helpers as support code, not as a new authority owner
- keep `build_surrogate.rs` and `compat/llvm_backend_surrogate.rs` local;
  do not widen either without review
