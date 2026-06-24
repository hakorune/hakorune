---
Status: Landed
Date: 2026-05-29
Scope: inventory alloc-result capsule IR shape as the last unblocked small exact-slot family.
Blocker: ALLOC-RESULT-CAPSULE-IR-SHAPE-INVENTORY-AFTER-RECORD-SUCCESS-HELPER-FUSION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-294-POST-RELEASE-RESULT-CAPSULE-OWNER-REFRESH-AFTER-RECORD-SUCCESS-HELPER-FUSION.md
  - docs/development/current/main/phases/phase-296x/296x-258-RESULT-CAPSULE-RESET-FIELD-BATCHING-IMPLEMENTATION.md
  - docs/development/current/main/phases/phase-296x/296x-282-RECORD-SUCCESS-HELPER-FUSION-IMPLEMENTATION.md
---

# 296x-295 Alloc Result Capsule IR Shape Inventory After RecordSuccess Helper Fusion

## Purpose

Inventory alloc-result capsule IR shape after row294 selected
`alloc_result_capsule` as the last unblocked family from the row284 exact-slot
table.

This row is observation-only. It does not flatten capsules, batch fields,
change `.hako` source shape, or reopen provider/replacement lanes.

## Evidence

```text
output_contract=alloc-result-capsule-ir-shape-inventory-after-record-success-helper-fusion-v0
input_contract=post-release-result-capsule-owner-refresh-after-record-success-helper-fusion-v0
workload_id=representative-object-lifecycle-small-block-v0
alloc_result_method_count=13
alloc_result_field_get_count=8
alloc_result_field_set_count=23
alloc_result_field_op_count=31
alloc_result_copy_count=22
alloc_result_call_count=0
alloc_result_phi_count=2
alloc_result_branch_count=2
top_alloc_method=HakoAllocObjectLifecycleAllocResult.birth/0
top_alloc_method_field_op_count=9
top_alloc_hot_method=HakoAllocObjectLifecycleAllocResult.recordSuccess/1
top_alloc_hot_method_field_op_count=8
reset_batching_landed=1
record_success_helper_fusion_landed=1
record_success_repeat_closed=1
remaining_family_is_small=1
selected_next=alloc_result_capsule_owner_selection_after_record_success_helper_fusion
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_next=alloc_result_capsule_owner_selection_after_record_success_helper_fusion
selected_reason=alloc_result_capsule_is_last_small_family_but_reset_and_recordSuccess_repeats_are_closed
next_row=alloc_result_capsule_owner_selection_after_record_success_helper_fusion
optimization_open=0
```

The next row should decide whether any alloc-result capsule owner remains. If
it only finds reset, recordSuccess, setup-shaped birth, or broad flattening,
the exact-slot micro-helper lane should close and return to
representation/direct-lowering design.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_alloc_result_capsule_ir_shape_inventory_after_record_success_helper_fusion_guard.sh
```
