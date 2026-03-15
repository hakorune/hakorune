# Stage1 Module Dispatch

Scope: compiled-stage1 string-module dispatch helpers under `crates/nyash_kernel/src/plugin/module_string_dispatch/`.

## Responsibility Split

- `module_string_dispatch.rs`
  - thin route table
  - shared string-handle encode/decode helpers
  - shared MirBuilder dispatch helpers
  - shared MirBuilder gate/decode/freeze wrappers for the source and Program(JSON) routes
  - probes `build_surrogate.rs` as an owner-local route before the shared table; it does not own `BuildBox.emit_program_json_v0` module/method strings or route registration
- `build_surrogate.rs`
  - compiled-stage1 `BuildBox.emit_program_json_v0` surrogate only
  - owner of the `build surrogate keep` bucket for `phase-29ci`
  - owner of the surrogate route match/dispatch contract and its regression coverage
  - owner of the launcher/stage1-cli-env Program(JSON) -> MIR handoff regression coverage too
  - the surrogate handler and route match stay owner-local; parent modules only probe via `try_dispatch(...)`
- `llvm_backend_surrogate.rs`
  - compiled-stage1 `selfhost.shared.backend.llvm_backend::{compile_obj,link_exe}` surrogate only
  - temporary B1 bridge owner for launcher/stage1-cli daily callers that still lower imported `LlvmBackendBox` methods to module-string receiver dispatch
  - owner of the backend boundary route match/dispatch contract and its regression coverage
  - compile side now shares the same path-based contract as the daily boundary through `mir_json_file_to_object(...)`
  - does not become the final backend owner; it remains a compiled-stage1 stop-gap until daily callers stop at the thin backend C boundary directly

## Retirement Note

- do not mix `build_surrogate.rs` retirement with `stage1_bridge` or `.hako` live/bootstrap caller deletion
- if the surrogate still cannot be removed, record that retreat in `phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md`
- treat the shared `emit_from_program_json_v0` / `emit_from_source_v0` gate-decode helpers as thin-floor support code, not as a new authority owner
