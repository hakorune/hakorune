---
Status: Landed
Date: 2026-05-29
Scope: inventory facade exact-slot field traffic before selecting a keeper.
Blocker: OBJECT-LIFECYCLE-FACADE-EXACT-SLOT-FIELD-INVENTORY-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-226-TYPED-OBJECT-EXACT-SLOT-CALLSITE-OWNER-SELECTION.md
---

# 296x-227 Object-Lifecycle Facade Exact-Slot Field Inventory

## Purpose

Inventory the `object_lifecycle_facade` exact-slot callsite family before any
implementation row.

This row decomposes the facade family into receiver state, page queue bridge,
page model bridge, and result capsule field traffic. It does not optimize or
open generic typed-field residence.

## Evidence

```text
output_contract=object-lifecycle-facade-exact-slot-field-inventory-v0
input_contract=typed-object-exact-slot-callsite-owner-selection-v0
workload_id=representative-object-lifecycle-small-block-v0
target_family=object_lifecycle_facade
target_family_pct=18.52
facade_method_count=3
facade_exact_slot_get_count=16
facade_exact_slot_set_count=9
facade_exact_slot_field_op_count=25
top_facade_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
top_facade_method_pct=10.72
dominant_field_family=facade_receiver_state
dominant_field_family_count=12
field_family.facade_receiver_state_count=12
field_family.page_model_bridge_count=1
field_family.page_queue_bridge_count=8
field_family.alloc_result_capsule_count=4
field_family.release_result_capsule_count=0
field_family.temporary_status_result_count=0
field_family.unknown_count=0
pattern.same_block_get_set_count=3
pattern.same_receiver_repeated_get_count=1
pattern.write_only_field_count=6
pattern.read_only_field_count=12
pattern.positive_net_cache_candidate_count=4
selected_next=facade_field_owner_selection
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
facade_method_0_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
facade_method_0_pct=10.72
facade_method_0_field_op_count=17
facade_method_1_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
facade_method_1_pct=6.27
facade_method_1_field_op_count=7
facade_method_2_symbol=HakoAllocObjectLifecycleFacade.resetSmallAllocResult/0
facade_method_2_pct=1.53
facade_method_2_field_op_count=1
summary=ok
```

## Decision

```text
selected_next=facade_field_owner_selection
selected_reason=facade_receiver_state_is_largest_field_family_but_positive_net_candidates_are_small
next_row=facade_field_owner_selection
```

The facade family is not dominated by result capsules. It is mostly facade
receiver state and page queue bridge traffic, with only four positive-net cache
or same-block candidates. The next row should choose between a narrow facade
state cache/writeback owner, a selected same-block fusion, or stopping if the
net opportunity is too small.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_object_lifecycle_facade_exact_slot_field_inventory_guard.sh
```
