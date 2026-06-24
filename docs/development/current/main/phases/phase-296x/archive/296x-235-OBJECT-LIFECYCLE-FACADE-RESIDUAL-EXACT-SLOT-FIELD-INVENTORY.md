---
Status: Landed
Date: 2026-05-29
Scope: inventory residual facade exact-slot field traffic after selected facade fusion.
Blocker: OBJECT-LIFECYCLE-FACADE-RESIDUAL-EXACT-SLOT-FIELD-INVENTORY-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-234-POST-FACADE-EXACT-SLOT-CALLSITE-OWNER-SELECTION.md
---

# 296x-235 Object-Lifecycle Facade Residual Exact-Slot Field Inventory

## Purpose

Inventory the residual `object_lifecycle_facade` exact-slot traffic after the
selected facade same-block get/set fusion keeper landed.

This row keeps optimization closed. It checks whether the remaining facade
traffic has a new positive-net shape before another keeper row.

## Evidence

```text
output_contract=object-lifecycle-facade-residual-exact-slot-field-inventory-v0
input_contract=post-facade-exact-slot-callsite-owner-selection-v0
workload_id=representative-object-lifecycle-small-block-v0
target_family=object_lifecycle_facade
target_family_pct=17.36
facade_method_count=5
facade_exact_slot_get_count=21
facade_exact_slot_set_count=9
facade_exact_slot_field_op_count=30
top_facade_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
top_facade_method_pct=11.12
dominant_field_family=facade_receiver_state
dominant_field_family_count=16
field_family.facade_receiver_state_count=16
field_family.page_model_bridge_count=1
field_family.page_queue_bridge_count=9
field_family.alloc_result_capsule_count=4
field_family.release_result_capsule_count=0
field_family.temporary_status_result_count=0
field_family.unknown_count=0
pattern.same_block_get_set_count=3
pattern.same_receiver_repeated_get_count=1
pattern.write_only_field_count=6
pattern.read_only_field_count=17
pattern.positive_net_cache_candidate_count=4
selected_next=facade_field_owner_selection
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
facade_method_0_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
facade_method_0_pct=11.12
facade_method_0_field_op_count=17
facade_method_1_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
facade_method_1_pct=2.76
facade_method_1_field_op_count=7
facade_method_2_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
facade_method_2_pct=1.40
facade_method_2_field_op_count=4
facade_method_3_symbol=HakoAllocObjectLifecycleFacade.recordReleaseSuccess/2
facade_method_3_pct=1.39
facade_method_3_field_op_count=1
facade_method_4_symbol=HakoAllocObjectLifecycleFacade.resetSmallAllocResult/0
facade_method_4_pct=0.69
facade_method_4_field_op_count=1
summary=ok
```

## Decision

```text
selected_next=facade_field_owner_selection
selected_reason=residual_facade_receiver_state_still_largest_but_positive_net_surface_remains_small
next_row=facade_residual_field_owner_selection
```

The post-fusion residual inventory still shows facade receiver state as the
largest field family, but the direct positive-net surface is not larger than
before. The next row should choose whether there is a narrow residual keeper or
whether to move to the next family instead of forcing another facade
optimization.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_object_lifecycle_facade_residual_exact_slot_field_inventory_guard.sh
```
