---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: share ArrayBox/MapBox birth call emission in module generic newbox handling
Related:
  - docs/development/current/main/phases/phase-29cv/P381BY-MODULE-GENERIC-I64-CALL-EMIT-HELPER-CLEANUP.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P381BZ: Module Generic Birth Call Emit Helper Cleanup

## Problem

The module generic `newbox` emitter repeated the same no-argument i64 birth call
shape for `ArrayBox` and `MapBox`:

```text
dst exists: emit %rN = call i64 @"birth"(), set T_I64, publish birth origin
no dst:     emit call i64 @"birth"()
```

The two branches differed only by symbol and origin.

## Decision

Introduce `module_generic_string_emit_i64_birth_call(dst, symbol, origin)` and
route `ArrayBox` / `MapBox` newbox emission through it.

This helper reuses the optional-`dst` i64 call seam from P381BY, then publishes
the birth origin only when `dst` exists. Newbox acceptance stays in
`module_generic_string_emit_newbox`.

This is behavior-preserving cleanup. It does not add a new accepted newbox type.

## Failure-First Probe

Before the cleanup, representative ArrayBox and MapBox constructor fixtures were
confirmed green:

```bash
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/runtime_data_array_push_min_v1.mir.json \
  --out /tmp/hakorune-array-birth-before.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_map_set_coldruntime_min_v1.mir.json \
  --out /tmp/hakorune-map-birth-before.o
```

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/runtime_data_array_push_min_v1.mir.json \
  --out /tmp/hakorune-array-birth.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_map_set_coldruntime_min_v1.mir.json \
  --out /tmp/hakorune-map-birth.o
cargo test --release generic_method_route_plan -- --nocapture
cargo test --release global_call_route_plan -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
