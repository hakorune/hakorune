---
Status: Landed
Date: 2026-05-29
Scope: select one result capsule owner before implementation.
Blocker: RESULT-CAPSULE-OWNER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-255-RESULT-CAPSULE-IR-SHAPE-DIFF-INVENTORY.md
---

# 296x-256 Result Capsule Owner Selection

## Purpose

Select one result capsule owner from row255 inventory before implementing a
keeper.

The row255 totals show 57 result-capsule field ops and no internal calls. The
safe next owner is the reset path, not birth or recordSuccess:

- `birth/0` has many field sets but is setup-shaped rather than hot-loop shaped.
- `recordSuccess` has branch/RMW shape and needs a separate plan.
- `recordRequest` is simple, but smaller and release-only.
- `reset/0` is hot, constant-shaped, and exists in both alloc/release result
  capsules.

## Evidence

```text
output_contract=result-capsule-owner-selection-v0
input_contract=result-capsule-ir-shape-diff-inventory-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_owner=result_capsule_reset_field_batching_guard_surface
selected_owner_kind=runtime_exact_slot_batch_helper
selected_reason=alloc_and_release_reset_are_hot_constant_field_set_shapes
selected_methods=HakoAllocObjectLifecycleAllocResult.reset/0,HakoAllocObjectLifecycleReleaseResult.reset/0
alloc_reset_field_set_count=4
release_reset_field_set_count=4
planned_erased_exact_slot_set_count=8
planned_added_batch_helper_count=2
planned_net_helper_call_delta=6
selected_storage_family=i64_exact_slot_constants
selected_shape=constant_field_set_batch
requires_new_runtime_symbols=1
requires_c_abi_same_module_emit=1
requires_hako_source_change=0
rejected_owner=birth_batching
rejected_reason=birth_is_setup_shaped_not_hot_loop_shaped
rejected_owner_1=record_success_fusion
rejected_reason_1=branch_and_rmw_shape_needs_separate_plan
rejected_owner_2=record_request_batching
rejected_reason_2=smaller_release_only_owner_than_reset_pair
rejected_owner_3=capsule_flattening
rejected_reason_3=too_broad_without_escape_specific_guard_surface
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_owner_family=result_capsule_reset_field_batching_guard_surface
selected_reason=positive_net_exact_slot_set_delta_with_two_constant_reset_methods
next_row=result_capsule_reset_field_batching_guard_surface
optimization_open=0
```

The next row must freeze the exact slots, helper symbols, and semantic proof
surface before implementation.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_result_capsule_owner_selection_guard.sh
```
