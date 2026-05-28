---
Status: Current
Date: 2026-05-28
Scope: measure SafeRwLock versus SingleThreadExact on the object-lifecycle exact-EXE workload.
Blocker: ARRAY-RUNTIME-SINGLE-THREAD-STORE-BACKEND-KEEPER-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-205-ARRAY-RUNTIME-SINGLE-THREAD-STORE-BACKEND-IMPLEMENTATION.md
---

# 296x-206 Array Runtime Single-Thread Store Backend Keeper Measurement

## Purpose

Measure whether row205's helper-side `SingleThreadExactArrayStore` is a keeper
on the selected object-lifecycle exact-EXE workload.

This row is measurement-only. It does not open MIR ArraySlotResidence,
provider activation, allocator replacement, hooks, globals, or winner claims.

## Measurement Contract

```text
output_contract=array-runtime-single-thread-store-backend-keeper-measurement-v0
input_contract=array-runtime-single-thread-store-backend-v0
workload_id=representative-object-lifecycle-small-block-v0
measurement_scope=object_lifecycle_exact_exe_array_slot_backend_pair
sample_count=3
typed_object_backend=single_thread_exact
safe_rwlock_body_elapsed_ns=219000000
single_thread_exact_body_elapsed_ns=129000000
body_elapsed_delta_ns=90000000
single_thread_exact_body_ratio_pct=59
safe_rwlock_external_elapsed_ms=220
single_thread_exact_external_elapsed_ms=130
keeper_effect=accepted
runtime_fast_lane_keeper=1
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Acceptance

```text
measurement_contract=accepted
runtime_fast_lane_keeper=1
keeper_effect=accepted
runtime_backend_is_floor_measurement=1
mir_array_slot_residence_still_required=1
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Next

```text
row207:
  mir_array_slot_residence_ssot

Goal:
  define ArraySlotResidencePlan / DirectSlotOp as the C-parity target now that
  the runtime helper backend floor is measured.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_array_runtime_single_thread_store_backend_keeper_measurement_guard.sh
```
