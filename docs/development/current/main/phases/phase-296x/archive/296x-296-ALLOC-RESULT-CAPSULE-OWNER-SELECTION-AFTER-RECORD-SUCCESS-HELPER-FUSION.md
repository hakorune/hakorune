---
Status: Landed
Date: 2026-05-29
Scope: select alloc-result capsule owner and close micro-helper hunting.
Blocker: ALLOC-RESULT-CAPSULE-OWNER-SELECTION-AFTER-RECORD-SUCCESS-HELPER-FUSION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-295-ALLOC-RESULT-CAPSULE-IR-SHAPE-INVENTORY-AFTER-RECORD-SUCCESS-HELPER-FUSION.md
  - docs/development/current/main/phases/phase-296x/296x-259-RESULT-CAPSULE-RESET-FIELD-BATCHING-MEASUREMENT.md
  - docs/development/current/main/phases/phase-296x/296x-282-RECORD-SUCCESS-HELPER-FUSION-IMPLEMENTATION.md
---

# 296x-296 Alloc Result Capsule Owner Selection After RecordSuccess Helper Fusion

## Purpose

Decide whether the last unblocked small exact-slot family supports another
keeper.

This row keeps optimization closed. It rejects alloc-result capsule repeats
because reset batching and recordSuccess helper fusion already landed, while
birth remains setup-shaped and broad capsule flattening requires a
representation contract instead of another helper row.

## Evidence

```text
output_contract=alloc-result-capsule-owner-selection-after-record-success-helper-fusion-v0
input_contract=alloc-result-capsule-ir-shape-inventory-after-record-success-helper-fusion-v0
workload_id=representative-object-lifecycle-small-block-v0
alloc_result_field_op_count=31
top_alloc_method=HakoAllocObjectLifecycleAllocResult.birth/0
top_alloc_method_field_op_count=9
top_alloc_hot_method=HakoAllocObjectLifecycleAllocResult.recordSuccess/1
top_alloc_hot_method_field_op_count=8
reset_batching_landed=1
record_success_helper_fusion_landed=1
record_success_repeat_closed=1
remaining_family_is_small=1
selected_owner=micro_helper_lane_closeout_and_representation_direct_lowering_selection
selected_reason=last_small_family_has_only_closed_reset_recordSuccess_or_setup_shaped_birth
next_diagnostic=micro_helper_lane_closeout_and_representation_direct_lowering_selection
rejected_owner=alloc_result_reset_batching_repeat
rejected_reason=result_capsule_reset_batching_already_landed_in_row259
rejected_owner_1=alloc_result_record_success_helper_fusion_repeat
rejected_reason_1=record_success_helper_fusion_already_landed_in_row282
rejected_owner_2=alloc_result_birth_batching
rejected_reason_2=birth_is_setup_shaped_not_current_hot_capsule_callsite
rejected_owner_3=generic_capsule_flattening
rejected_reason_3=too_broad_without_representation_contract_selection
micro_helper_lane_has_remaining_small_keeper=0
representation_direct_lowering_required=1
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_owner=micro_helper_lane_closeout_and_representation_direct_lowering_selection
next_row=micro_helper_lane_closeout_and_representation_direct_lowering_selection
optimization_open=0
```

The row284 exact-slot micro-helper owner table has no remaining promising
small keeper after known nonkeepers and landed helper fusions are excluded.
The next row should close the micro-helper lane and select the
representation/direct-lowering design path.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_alloc_result_capsule_owner_selection_after_record_success_helper_fusion_guard.sh
```
