# Selfhost Compat Payloads

This directory holds archive-later compat/proof payloads.

## Current Surface

- `hako_llvm_selfhost_driver.hako`
- proof/example payload behind `tools/selfhost/run_compat_pure_selfhost.sh`
- exercises the legacy `CodegenBridgeBox.emit_object_args(...)` and
  `CodegenBridgeBox.link_object_args(...)` route
- not a daily backend owner
- keep until the compat wrapper gains a root-first replacement or is retired as
  a whole

## Layering

- payload:
  - `hako_llvm_selfhost_driver.hako`
- transport wrapper:
  - `tools/selfhost/run_compat_pure_selfhost.sh`
- pack orchestrator:
  - `tools/selfhost/run_compat_pure_pack.sh`

Read this directory as payload-only. The shell scripts above are the transport
and orchestration layers that still sit on top of it.
