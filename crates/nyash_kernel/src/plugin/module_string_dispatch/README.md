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
  - compiled-stage1 `BuildBox.emit_program_json_v0` dispatch shim only
  - frozen exact owner for the build surrogate residue bucket; docs/inventory closeout only until caller-proof says removable
  - owner of the surrogate route match/dispatch contract, decode, and encode
  - build-box / launcher handoff regression coverage lives in `src/stage1/program_json_v0.rs` tests
  - the surrogate handler and route match stay owner-local; parent modules only probe via `try_dispatch(...)`
- `llvm_backend_surrogate.rs`
  - compiled-stage1 `selfhost.shared.backend.llvm_backend::{compile_obj,link_exe}` surrogate only
  - frozen exact owner for the backend boundary residue bucket; docs/inventory closeout only until caller-proof says removable
  - owner of the backend boundary route match/dispatch contract and its regression coverage
  - compile side now shares the same path-based contract as the daily boundary through `mir_json_file_to_object(...)`
  - latest shrink keeps path decode / compile opts / link arg decode behind owner-local helpers; parent modules still probe only via `try_dispatch(...)`
  - does not become the final backend owner; it remains a compiled-stage1 stop-gap until daily callers stop at the thin backend C boundary directly

## Retirement Note

- do not mix `build_surrogate.rs` retirement with `stage1_bridge` or `.hako` live/bootstrap caller deletion
- if the surrogate still cannot be removed, record that retreat in `phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md`
- treat the shared `emit_from_program_json_v0` / `emit_from_source_v0` gate-decode helpers as thin-floor support code, not as a new authority owner
- treat `build_surrogate.rs` and `llvm_backend_surrogate.rs` as frozen exact owners; do not reopen either without caller-proof
