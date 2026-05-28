---
Status: Landed
Date: 2026-05-29
Scope: measure result capsule reset field-batching implementation.
Blocker: RESULT-CAPSULE-RESET-FIELD-BATCHING-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-258-RESULT-CAPSULE-RESET-FIELD-BATCHING-IMPLEMENTATION.md
---

# 296x-259 Result Capsule Reset Field-Batching Measurement

## Purpose

Measure the result capsule reset field-batching implementation before accepting
it as a performance keeper.

This row uses the existing exact-slot helper measurement pair:

```text
floor:
  HAKO_TYPED_OBJECT_STORE=single_thread_exact
  HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER unset

candidate:
  HAKO_TYPED_OBJECT_STORE=single_thread_exact
  HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1
```

Both sides use `HAKO_ARRAY_SLOT_STORE=single_thread_exact`.

## Evidence

```text
output_contract=result-capsule-reset-field-batching-measurement-v0
input_contract=result-capsule-reset-field-batching-implementation-v0
base_measurement_contract=typed-object-exact-slot-direct-helper-measurement-v0
workload_id=representative-object-lifecycle-small-block-v0
measurement_scope=object_lifecycle_exact_exe_after_result_capsule_reset_batching
sample_count=3
typed_object_backend=single_thread_exact
array_slot_backend=single_thread_exact
single_thread_exact_floor_body_elapsed_ns=113000000
result_capsule_reset_batching_body_elapsed_ns=110000000
body_elapsed_delta_ns=3000000
result_capsule_reset_batching_body_ratio_pct=97
keeper_acceptance_min_improvement_pct=3
single_thread_exact_floor_external_elapsed_ms=110
result_capsule_reset_batching_external_elapsed_ms=110
keeper_effect=accepted
result_capsule_reset_batching_keeper=1
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_next=post_result_capsule_reset_batching_owner_refresh
next_row=post_result_capsule_reset_batching_owner_refresh
optimization_open=0
```

The keeper is accepted as a small win. The next row must refresh the current
hot owner before selecting another implementation.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_result_capsule_reset_field_batching_measurement_guard.sh
```
