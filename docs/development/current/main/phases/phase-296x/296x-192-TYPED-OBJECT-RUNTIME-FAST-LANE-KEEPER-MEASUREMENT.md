---
Status: Landed
Date: 2026-05-28
Scope: measure SafeMutexStore versus SingleThreadExactStore on the object-lifecycle exact-EXE workload.
Blocker: TYPED-OBJECT-RUNTIME-FAST-LANE-KEEPER-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-191-TYPED-OBJECT-RUNTIME-SINGLE-THREAD-FAST-LANE.md
---

# 296x-192 Typed Object Runtime Fast Lane Keeper Measurement

## Purpose

Measure whether row191's `SingleThreadExactStore` is a keeper on the selected
object-lifecycle exact-EXE workload. This row is measurement-only; it does not
open MIR scalar residence, ArrayBox optimization, provider activation, allocator
replacement, hooks, globals, or winner claims.

## Measurement Contract

```text
output_contract=typed-object-runtime-fast-lane-keeper-measurement-v0
input_contract=typed-object-runtime-single-thread-fast-lane-v0
workload_id=representative-object-lifecycle-small-block-v0
measurement_scope=object_lifecycle_exact_exe_typed_object_store_backend_pair
safe_mutex_body_elapsed_ns=...
single_thread_exact_body_elapsed_ns=...
body_elapsed_delta_ns=...
single_thread_exact_body_ratio_pct=...
keeper_effect=accepted|no_effect
runtime_fast_lane_keeper=0|1
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
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```
