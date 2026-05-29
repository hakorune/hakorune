---
Status: Landed
Date: 2026-05-29
Scope: inventory alloc/release result capsule IR shape after rollback owner refresh.
Blocker: ALLOC-RESULT-CAPSULE-IR-SHAPE-INVENTORY-AFTER-RELEASE-KNOWN-LIVE-ROLLBACK-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-274-POST-FACADE-INVENTORY-OWNER-REFRESH-AFTER-RELEASE-KNOWN-LIVE-ROLLBACK.md
---

# 296x-275 Alloc Result Capsule IR Shape Inventory After Release Known Live Rollback

## Purpose

Inventory result capsule IR shape from current MIR after row274 selected
`alloc_result_capsule` as the next unblocked exact-slot family.

This row is observation-only. It does not flatten capsules, batch fields,
change `.hako` source shape, or reopen provider/replacement lanes.

## Evidence

```text
output_contract=alloc-result-capsule-ir-shape-inventory-after-release-known-live-rollback-v0
input_contract=post-facade-inventory-owner-refresh-after-release-known-live-rollback-v0
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
escape_shape=method_local_receiver_mutation_then_scalar_return
direct_call_count=0
capsule_flattening_requires_owner_selection=1
selected_next=result_capsule_owner_selection_after_release_known_live_rollback
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_next=result_capsule_owner_selection_after_release_known_live_rollback
selected_reason=result_capsules_still_have_57_field_ops_and_no_internal_calls
next_row=result_capsule_owner_selection_after_release_known_live_rollback
optimization_open=0
```

The next row should select one capsule owner from the current shape. It must
not implement a keeper before selecting whether the owner is reset/birth
batching, recordSuccess fusion, recordRequest shape, or capsule flattening.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_alloc_result_capsule_ir_shape_inventory_after_release_known_live_rollback_guard.sh
```
