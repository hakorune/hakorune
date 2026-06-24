---
Status: Landed
Date: 2026-05-29
Scope: inventory release-result capsule IR shape after recordSuccess helper fusion.
Blocker: RELEASE-RESULT-CAPSULE-IR-SHAPE-INVENTORY-AFTER-RECORD-SUCCESS-HELPER-FUSION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-291-POST-PAGE-MODEL-HOTPATH-OWNER-REFRESH-AFTER-RECORD-SUCCESS-HELPER-FUSION.md
  - docs/development/current/main/phases/phase-296x/296x-282-RECORD-SUCCESS-HELPER-FUSION-IMPLEMENTATION.md
---

# 296x-292 Release Result Capsule IR Shape Inventory After RecordSuccess Helper Fusion

## Purpose

Inventory release-result capsule IR shape after row291 selected
`release_result_capsule` as the next unblocked exact-slot family.

This row is observation-only. It does not flatten capsules, batch fields,
change `.hako` source shape, or reopen provider/replacement lanes.

## Evidence

```text
output_contract=release-result-capsule-ir-shape-inventory-after-record-success-helper-fusion-v0
input_contract=post-page-model-hotpath-owner-refresh-after-record-success-helper-fusion-v0
workload_id=representative-object-lifecycle-small-block-v0
release_result_method_count=11
release_result_field_get_count=6
release_result_field_set_count=19
release_result_field_op_count=25
release_result_copy_count=14
release_result_call_count=0
release_result_phi_count=0
release_result_branch_count=0
top_release_method=HakoAllocObjectLifecycleReleaseResult.birth/0
top_release_method_field_op_count=6
top_release_hot_method=HakoAllocObjectLifecycleReleaseResult.recordSuccess/2
top_release_hot_method_field_op_count=6
record_success_helper_fusion_landed=1
record_success_repeat_closed=1
selected_next=release_result_capsule_owner_selection_after_record_success_helper_fusion
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_next=release_result_capsule_owner_selection_after_record_success_helper_fusion
selected_reason=release_result_capsule_has_25_field_ops_but_recordSuccess_repeat_is_closed
next_row=release_result_capsule_owner_selection_after_record_success_helper_fusion
optimization_open=0
```

The next row should select one release-result capsule owner from the current
shape. It must not repeat recordSuccess helper fusion, because row282/283
already landed and measured that keeper.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_release_result_capsule_ir_shape_inventory_after_record_success_helper_fusion_guard.sh
```
