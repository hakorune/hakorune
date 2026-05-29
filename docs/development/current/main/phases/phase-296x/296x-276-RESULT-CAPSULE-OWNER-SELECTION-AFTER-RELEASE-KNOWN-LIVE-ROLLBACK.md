---
Status: Landed
Date: 2026-05-29
Scope: select one result capsule owner from current capsule IR shape.
Blocker: RESULT-CAPSULE-OWNER-SELECTION-AFTER-RELEASE-KNOWN-LIVE-ROLLBACK-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-275-ALLOC-RESULT-CAPSULE-IR-SHAPE-INVENTORY-AFTER-RELEASE-KNOWN-LIVE-ROLLBACK.md
---

# 296x-276 Result Capsule Owner Selection After Release Known Live Rollback

## Purpose

Select one result-capsule owner from row275 current IR-shape inventory.

This row keeps optimization closed. It does not repeat reset field batching
because that keeper already landed in row259. It selects the recordSuccess
shape as the next guard-surface row.

## Evidence

```text
output_contract=result-capsule-owner-selection-after-release-known-live-rollback-v0
input_contract=alloc-result-capsule-ir-shape-inventory-after-release-known-live-rollback-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_owner=result_capsule_record_success_shape_guard_surface
selected_owner_kind=branch_aware_exact_slot_rmw_and_status_set_plan
selected_reason=reset_batching_already_landed_and_record_success_is_top_hot_capsule_shape
selected_methods=HakoAllocObjectLifecycleAllocResult.recordSuccess/1,HakoAllocObjectLifecycleReleaseResult.recordSuccess/2
alloc_record_success_field_op_count=8
release_record_success_field_op_count=6
record_success_combined_field_op_count=14
alloc_record_success_has_branch_shape=1
release_record_success_has_straightline_shape=1
requires_guard_surface_before_implementation=1
requires_hako_source_change=0
selected_next=result_capsule_record_success_shape_guard_surface
rejected_owner=result_capsule_reset_field_batching
rejected_reason=result_capsule_reset_field_batching_already_landed_in_row259
rejected_owner_1=birth_batching
rejected_reason_1=birth_is_setup_shaped_not_current_hot_capsule_callsite
rejected_owner_2=record_request_batching
rejected_reason_2=smaller_release_only_owner_than_record_success_pair
rejected_owner_3=capsule_flattening
rejected_reason_3=too_broad_without_escape_specific_guard_surface
rejected_owner_4=source_inline_success_result_fast_path
rejected_reason_4=prior_source_inline_success_result_regressed_and_was_rolled_back
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_owner=result_capsule_record_success_shape_guard_surface
next_row=result_capsule_record_success_shape_guard_surface
optimization_open=0
```

The next row must freeze the exact shape and safety surface before
implementation. The alloc result shape is branch-aware and cannot be treated as
a simple reset-style constant set batch.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_result_capsule_owner_selection_after_release_known_live_rollback_guard.sh
```
