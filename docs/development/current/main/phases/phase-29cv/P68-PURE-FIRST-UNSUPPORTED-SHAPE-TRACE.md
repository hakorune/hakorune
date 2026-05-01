---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: implement trace-only pure-first unsupported-shape inventory.
Related:
  - docs/development/current/main/phases/phase-29cv/P67-PURE-FIRST-UNSUPPORTED-SHAPE-DIAGNOSTIC-LOCK.md
  - docs/development/current/main/design/ai-handoff-and-debug-contract.md
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering.inc
---

# P68 Pure-First Unsupported-Shape Trace

## Goal

Make the existing pure-first fail-fast actionable by emitting the first
candidate unsupported shape when route tracing is enabled.

## Decision

- Keep `unsupported pure shape for current backend recipe` as the returned
  error.
- Keep `HAKO_BACKEND_COMPAT_REPLAY=none` as the mainline proof setting.
- Record the first generic pure lowering abort candidate in local state.
- Emit `[llvm-pure/unsupported-shape]` only at the final unsupported stop line.
- If generic lowering did not identify a candidate, emit `first_op=unknown`
  rather than scanning route policy in C.

## Non-goals

- no pure-first acceptance widening
- no compat replay promotion
- no C-side route-policy ownership

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 bash tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_string_piecewise_direct_set_min.sh
git diff --check
```
