---
Status: Landed
Date: 2026-05-29
Scope: inventory alloc/release result capsule IR shape before another keeper.
Blocker: RESULT-CAPSULE-IR-SHAPE-DIFF-INVENTORY-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-254-WEIGHTED-EXACT-SLOT-OWNER-SELECTION-AFTER-RECEIVER-FORWARDING.md
---

# 296x-255 Result Capsule IR Shape Diff Inventory

## Purpose

Inventory `HakoAllocObjectLifecycleAllocResult.*` and
`HakoAllocObjectLifecycleReleaseResult.*` field traffic before selecting a
capsule keeper.

This row is observation-only. It does not flatten capsules, batch fields, or
change `.hako` source shape.

## Evidence

```text
output_contract=result-capsule-ir-shape-diff-inventory-v0
input_contract=weighted-exact-slot-owner-selection-after-receiver-forwarding-v0
workload_id=representative-object-lifecycle-small-block-v0
alloc_result_method_count=13
alloc_result_field_get_count=9
alloc_result_field_set_count=23
alloc_result_field_op_count=32
alloc_result_copy_count=22
alloc_result_call_count=0
alloc_result_phi_count=2
alloc_result_branch_count=2
release_result_method_count=11
release_result_field_get_count=6
release_result_field_set_count=19
release_result_field_op_count=25
release_result_copy_count=14
release_result_call_count=0
release_result_phi_count=0
release_result_branch_count=0
combined_result_field_get_count=15
combined_result_field_set_count=42
combined_result_field_op_count=57
combined_result_copy_count=36
combined_result_call_count=0
combined_result_phi_count=2
combined_result_branch_count=2
top_alloc_method=HakoAllocObjectLifecycleAllocResult.birth/0
top_alloc_method_field_op_count=9
top_alloc_hot_method=HakoAllocObjectLifecycleAllocResult.recordSuccess/1
top_alloc_hot_method_field_op_count=8
top_release_method=HakoAllocObjectLifecycleReleaseResult.birth/0
top_release_method_field_op_count=6
top_release_hot_method=HakoAllocObjectLifecycleReleaseResult.recordSuccess/2
top_release_hot_method_field_op_count=6
release_request_method=HakoAllocObjectLifecycleReleaseResult.recordRequest/2
release_request_field_op_count=2
escape_shape=method_local_receiver_mutation_then_scalar_return
direct_call_count=0
capsule_flattening_requires_owner_selection=1
selected_next=result_capsule_owner_selection
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_owner_family=result_capsule_owner_selection
selected_reason=result_capsules_have_57_field_ops_and_no_internal_calls_so_shape_options_are_now_visible
next_row=result_capsule_owner_selection
optimization_open=0
```

The next row should choose one capsule owner, such as reset/birth batching,
recordSuccess fusion, recordRequest shape, or source-level capsule flattening.
No keeper should be implemented until that owner is selected.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_result_capsule_ir_shape_diff_inventory_guard.sh
```
