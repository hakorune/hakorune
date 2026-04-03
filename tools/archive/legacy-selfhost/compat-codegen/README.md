# tools/archive/legacy-selfhost/compat-codegen

Archived home for historical compat-codegen payloads and wrappers.

## Current Surface

- `hako_llvm_selfhost_driver.hako`
- `run_compat_pure_selfhost.sh`
- `run_compat_pure_pack.sh`

The wrapper now preserves the historical shell contract while materializing the
payload onto `vm-hako`. The rendered payload proves the provider stop-line via
`LlvmBackendEvidenceAdapterBox.compile_obj_provider_stopline(...)` and then
links through `LlvmBackendBox.link_exe(...)`.

## Layering

- payload:
  - `hako_llvm_selfhost_driver.hako`
- transport wrapper:
  - `run_compat_pure_selfhost.sh`
- pack orchestrator:
  - `run_compat_pure_pack.sh`

Read this directory as the archived compat-codegen bucket. The old
`tools/selfhost/run_compat_pure_*` entrypoints are thin backward-compat shims
that exec into this directory.
