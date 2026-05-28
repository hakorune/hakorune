---
Status: Landed
Date: 2026-05-29
Scope: measure the exact-lane typed-object slot direct helper implementation.
Blocker: TYPED-OBJECT-EXACT-SLOT-DIRECT-HELPER-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-215-TYPED-OBJECT-EXACT-SLOT-DIRECT-HELPER-IMPLEMENTATION.md
  - docs/development/current/main/phases/phase-296x/296x-212-SELECTED-METHOD-ARRAY-SLOT-DIRECT-OP-MEASUREMENT.md
---

# 296x-216 Typed Object Exact Slot Direct Helper Measurement

## Purpose

Measure whether the row215 exact-slot helper symbols reduce object-lifecycle
body time compared with the existing SingleThreadExact typed-object floor.

This row is measurement-only. It does not open provider activation, allocator
replacement, hooks, globals, generic CSE, or MIR field residence.

## Measurement

```text
output_contract=typed-object-exact-slot-direct-helper-measurement-v0
input_contract=typed-object-exact-slot-direct-helper-implementation-v0
workload_id=representative-object-lifecycle-small-block-v0
measurement_scope=object_lifecycle_exact_exe_exact_slot_helper_pair
sample_count=3
typed_object_backend=single_thread_exact
array_slot_backend=single_thread_exact
single_thread_exact_floor_body_elapsed_ns=120000000
exact_slot_helper_body_elapsed_ns=120000000
body_elapsed_delta_ns=0
exact_slot_helper_body_ratio_pct=100
keeper_acceptance_min_improvement_pct=3
single_thread_exact_floor_external_elapsed_ms=120
exact_slot_helper_external_elapsed_ms=120
keeper_effect=no_effect
exact_slot_helper_keeper=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_boundary=post_exact_slot_helper_owner_refresh
selected_reason=exact_slot_helper_erased_generic_validation_but_body_time_did_not_move
next_diagnostic=post_exact_slot_helper_owner_refresh
optimization_open=0
```

The exact-slot helper implementation remains diagnostic-only and default-off.
The measurement says the current large gap is not primarily the generic
typed-object helper validation branch removed by row215. The next row must
refresh the hot owner using perf evidence before another implementation.

Stability refresh:

```text
sample_count=5
single_thread_exact_floor_body_elapsed_ns=120000000
exact_slot_helper_body_elapsed_ns=118000000
body_elapsed_delta_ns=2000000
exact_slot_helper_body_ratio_pct=98
keeper_effect=no_effect
summary=ok
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_typed_object_exact_slot_direct_helper_measurement_guard.sh
```
