---
Status: Landed
Date: 2026-05-29
Scope: measure the selected facade same-block get/set fusion keeper.
Blocker: SELECTED-FACADE-SAME-BLOCK-GET-SET-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-230-SELECTED-FACADE-SAME-BLOCK-GET-SET-KEEPER.md
---

# 296x-231 Selected Facade Same-Block Get/Set Measurement

## Purpose

Measure the row230 selected-facade same-block `field_get -> add -> field_set`
fusion on the object-lifecycle exact-EXE workload.

This row only closes the keeper measurement. It does not refresh perf ownership,
open generic typed-field residence, bind provider APIs, activate providers,
replace the host allocator, install hooks, or make winner claims.

## Evidence

```text
output_contract=selected-facade-same-block-get-set-measurement-v0
input_contract=selected-facade-same-block-get-set-keeper-v0
base_measurement_contract=typed-object-exact-slot-direct-helper-measurement-v0
workload_id=representative-object-lifecycle-small-block-v0
measurement_scope=object_lifecycle_exact_exe_after_selected_facade_get_set_fusion
sample_count=3
typed_object_backend=single_thread_exact
array_slot_backend=single_thread_exact
single_thread_exact_floor_body_elapsed_ns=124000000
selected_facade_get_set_body_elapsed_ns=119000000
body_elapsed_delta_ns=5000000
selected_facade_get_set_body_ratio_pct=96
keeper_acceptance_min_improvement_pct=3
single_thread_exact_floor_external_elapsed_ms=130
selected_facade_get_set_external_elapsed_ms=120
keeper_effect=accepted
selected_facade_get_set_keeper=1
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
keeper_effect=accepted
selected_facade_get_set_keeper=1
next_diagnostic=post_selected_facade_get_set_owner_refresh
optimization_open=0
```

The 5ms body-time median improvement meets the current 3% keeper threshold for
this diagnostic lane. The next row must refresh the hot owner before another
optimization.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_selected_facade_same_block_get_set_measurement_guard.sh
```
