# Selfhost Compat Payloads

This directory is now a compatibility alias. The canonical compat-codegen
payload bucket lives under `tools/compat/legacy-codegen/`.

## Current Surface

- canonical payload: `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako`
- old payload path: `tools/selfhost/compat/hako_llvm_selfhost_driver.hako` (retired alias)
- archive-later payload behind `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh`
- wrapper materializes the payload onto `vm-hako`
- payload proves the provider stop-line through `LlvmBackendEvidenceAdapterBox.compile_obj_provider_stopline(...)`
- payload links through `LlvmBackendBox.link_exe(...)`
- non-owner compat surface
- keep until the compat wrapper gains a root-first replacement or is retired as
  a whole

## Layering

- payload:
  - `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako`
- transport wrapper:
  - `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh`
- pack orchestrator:
  - `tools/compat/legacy-codegen/run_compat_pure_pack.sh`

Read this directory as payload-only. The shell scripts above are the transport
and orchestration layers that still sit on top of it, but the canonical home is
now `tools/compat/legacy-codegen/`.
