# Stage1 Module Dispatch

Scope: compiled-stage1 string-module dispatch helpers under `crates/nyash_kernel/src/plugin/module_string_dispatch/`.

## Responsibility Split

- `module_string_dispatch.rs`
  - thin route table
  - shared string-handle encode/decode helpers
  - shared MirBuilder dispatch helpers
  - imports owner-local surrogate route registrations; it does not own `BuildBox.emit_program_json_v0` module/method strings
- `build_surrogate.rs`
  - compiled-stage1 `BuildBox.emit_program_json_v0` surrogate only
  - owner of the `build surrogate keep` bucket for `phase-29ci`
  - owner of the surrogate route registration (`BUILD_SURROGATE_ROUTE`) and its regression coverage
  - owner of the launcher/stage1-cli-env Program(JSON) -> MIR handoff regression coverage too
  - the surrogate handler stays owner-local; parent modules import only the route registration

## Retirement Note

- do not mix `build_surrogate.rs` retirement with `stage1_bridge` or `.hako` live/bootstrap caller deletion
- if the surrogate still cannot be removed, record that retreat in `phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md`
