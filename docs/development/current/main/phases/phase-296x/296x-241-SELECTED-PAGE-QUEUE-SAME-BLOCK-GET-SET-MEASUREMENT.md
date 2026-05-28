---
Status: Landed
Date: 2026-05-29
Scope: measure the selected page queue same-block get/set fusion keeper.
Blocker: SELECTED-PAGE-QUEUE-SAME-BLOCK-GET-SET-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-240-SELECTED-PAGE-QUEUE-SAME-BLOCK-GET-SET-KEEPER.md
---

# 296x-241 Selected Page Queue Same-Block Get/Set Measurement

## Purpose

Measure the row240 selected page-queue same-block `field_get -> add ->
field_set` fusion on the object-lifecycle exact-EXE workload.

This row only closes the keeper measurement. It does not refresh perf
ownership, open generic typed-field residence, bind provider APIs, activate
providers, replace the host allocator, install hooks, or make winner claims.

## Evidence

```text
output_contract=selected-page-queue-same-block-get-set-measurement-v0
input_contract=selected-page-queue-same-block-get-set-keeper-v0
base_measurement_contract=typed-object-exact-slot-direct-helper-measurement-v0
workload_id=representative-object-lifecycle-small-block-v0
measurement_scope=object_lifecycle_exact_exe_after_selected_page_queue_get_set_fusion
sample_count=3
typed_object_backend=single_thread_exact
array_slot_backend=single_thread_exact
single_thread_exact_floor_body_elapsed_ns=114000000
selected_page_queue_get_set_body_elapsed_ns=119000000
body_elapsed_delta_ns=-5000000
selected_page_queue_get_set_body_ratio_pct=104
keeper_acceptance_min_improvement_pct=3
single_thread_exact_floor_external_elapsed_ms=110
selected_page_queue_get_set_external_elapsed_ms=120
keeper_effect=no_effect
selected_page_queue_get_set_keeper=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
keeper_effect=no_effect
selected_page_queue_get_set_keeper=0
next_diagnostic=rollback_selected_page_queue_same_block_get_set
optimization_open=0
```

The page queue fusion worsened the body-time median in this measurement. The
next row must roll back the row240 page queue target extension before refreshing
the hot owner.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_selected_page_queue_same_block_get_set_measurement_guard.sh
```
