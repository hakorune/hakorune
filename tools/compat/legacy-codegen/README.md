# tools/compat/legacy-codegen

Canonical home for historical compat-codegen payloads and wrappers.

## Current Surface

- `hako_llvm_selfhost_driver.hako`
- `run_compat_pure_selfhost.sh`
- `run_compat_pure_pack.sh`

## Layering

- payload:
  - `hako_llvm_selfhost_driver.hako`
- transport wrapper:
  - `run_compat_pure_selfhost.sh`
- pack orchestrator:
  - `run_compat_pure_pack.sh`

Read this directory as the canonical compat-codegen bucket. The old
`tools/selfhost/run_compat_pure_*` entrypoints are thin backward-compat shims
that exec into this directory.
