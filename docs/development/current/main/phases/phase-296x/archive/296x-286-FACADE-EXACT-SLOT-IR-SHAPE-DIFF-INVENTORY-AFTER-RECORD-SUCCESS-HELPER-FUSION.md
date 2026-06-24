---
Status: Landed
Date: 2026-05-29
Scope: inventory facade exact-slot traffic after recordSuccess helper fusion owner selection.
Blocker: FACADE-EXACT-SLOT-IR-SHAPE-DIFF-INVENTORY-AFTER-RECORD-SUCCESS-HELPER-FUSION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-285-WEIGHTED-EXACT-SLOT-OWNER-SELECTION-AFTER-RECORD-SUCCESS-HELPER-FUSION.md
---

# 296x-286 Facade Exact-Slot IR Shape Diff Inventory After RecordSuccess Helper Fusion

## Purpose

Inventory the selected `object_lifecycle_facade` exact-slot family after the
recordSuccess helper fusion keeper and weighted owner selection.

This row does not implement a keeper. It decomposes facade exact-slot traffic
into receiver-state, page-model bridge, page-queue bridge, and result-capsule
field traffic so the next row can choose a single owner with positive-net
evidence.

## Evidence

```text
output_contract=object-lifecycle-facade-exact-slot-field-inventory-v0
input_contract=weighted-exact-slot-owner-selection-v0
workload_id=representative-object-lifecycle-small-block-v0
target_family=object_lifecycle_facade
target_family_pct=13.47
facade_method_count=5
facade_exact_slot_get_count=18
facade_exact_slot_set_count=9
facade_exact_slot_field_op_count=27
top_facade_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
top_facade_method_pct=7.14
dominant_field_family=facade_receiver_state
dominant_field_family_count=14
field_family.facade_receiver_state_count=14
field_family.page_model_bridge_count=1
field_family.page_queue_bridge_count=8
field_family.alloc_result_capsule_count=4
field_family.release_result_capsule_count=0
field_family.temporary_status_result_count=0
field_family.unknown_count=0
pattern.same_block_get_set_count=3
pattern.same_receiver_repeated_get_count=1
pattern.write_only_field_count=6
pattern.read_only_field_count=14
pattern.positive_net_cache_candidate_count=4
selected_next=facade_field_owner_selection
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
facade_method_0_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
facade_method_0_pct=7.14
facade_method_0_field_op_count=17
facade_method_1_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
facade_method_1_pct=3.13
facade_method_1_field_op_count=7
facade_method_2_symbol=HakoAllocObjectLifecycleFacade.resetReleaseResult/0
facade_method_2_pct=1.36
facade_method_2_field_op_count=1
facade_method_3_symbol=HakoAllocObjectLifecycleFacade.resetSmallAllocResult/0
facade_method_3_pct=1.20
facade_method_3_field_op_count=1
facade_method_4_symbol=HakoAllocObjectLifecycleFacade.recordReleaseSuccess/2
facade_method_4_pct=0.64
facade_method_4_field_op_count=1
summary=ok
```

## Decision

```text
selected_next=facade_field_owner_selection
selected_reason=facade_receiver_state_is_largest_but_positive_net_candidates_remain_small_after_record_success_fusion
next_row=facade_field_owner_selection_after_record_success_helper_fusion
optimization_open=0
```

The facade family remains the top unblocked exact-slot family, but the
positive-net surface is still narrow: three same-block get/set pairs and one
repeated same-receiver get. The next row must decide whether this supports a
small facade owner, or whether owner refresh is cleaner before another
implementation row.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_facade_exact_slot_ir_shape_diff_inventory_after_record_success_helper_fusion_guard.sh
```
