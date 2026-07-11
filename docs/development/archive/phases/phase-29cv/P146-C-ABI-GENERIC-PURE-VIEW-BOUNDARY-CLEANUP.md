---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P146, C-ABI generic pure function view boundary cleanup
Related:
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - tools/checks/inc_codegen_thin_shim_guard.sh
  - tools/checks/mir_root_facade_guard.sh
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_leaf_function_emit.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
  - src/mir/mod.rs
---

# P146: C-ABI Generic Pure View Boundary Cleanup

## Problem

`tools/checks/dev_gate.sh quick` failed before the next development slice at
two compiler-cleanliness gates:

- `.inc codegen thin-shim guard` detected fresh direct shape reads in module
  function emitters.
- `MIR root facade guard` detected new root exports for
  `refresh_function_global_call_routes` and `refresh_module_global_call_routes`.

Both failures were BoxShape problems. The compiler already had a
`generic_pure_view` boundary and the global-call route owner module already
exported the refresh functions for direct callers. Allowlisting the drift would
have widened the root/facade surface instead of reducing it.

## Decision

Keep the cleanup behavior-preserving and move ownership back to existing
facades:

- Add `read_generic_pure_function_block_view(...)` to the `generic_pure_view`
  boundary so non-entry function emitters can read block instructions without
  hand-parsing `blocks` / `instructions`.
- Route `numeric_i64_leaf` and `generic_pure_string_body` module function
  emitters through the generic pure function view instead of local raw shape
  reads.
- Remove the unnecessary MIR-root re-export of global-call route refresh
  helpers. Callers use `crate::mir::global_call_route_plan::*` directly.

This does not add a new acceptance shape and does not change Program(JSON v0)
compat policy. It only reduces duplicate entry points and keeps the thin-shim
guard at zero direct analysis-debt files.

## Acceptance

```bash
bash tools/checks/inc_codegen_thin_shim_guard.sh
bash tools/checks/mir_root_facade_guard.sh
cargo fmt --check
bash tools/checks/dev_gate.sh quick
git diff --check
cargo build --release --bin hakorune
```
