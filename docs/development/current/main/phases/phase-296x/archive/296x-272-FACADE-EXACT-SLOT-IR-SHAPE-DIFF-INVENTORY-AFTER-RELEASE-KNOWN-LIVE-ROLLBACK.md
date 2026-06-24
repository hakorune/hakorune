---
Status: Landed
Date: 2026-05-29
Scope: inventory facade exact-slot traffic after releaseKnownLive rollback owner selection.
Blocker: FACADE-EXACT-SLOT-IR-SHAPE-DIFF-INVENTORY-AFTER-RELEASE-KNOWN-LIVE-ROLLBACK-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-271-WEIGHTED-EXACT-SLOT-OWNER-SELECTION-AFTER-RELEASE-KNOWN-LIVE-ROLLBACK.md
---

# 296x-272 Facade Exact-Slot IR Shape Diff Inventory After Release Known Live Rollback

## Purpose

Inventory the selected `object_lifecycle_facade` exact-slot family after the
releaseKnownLive RMW rollback and weighted owner selection.

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
target_family_pct=15.68
facade_method_count=4
facade_exact_slot_get_count=20
facade_exact_slot_set_count=9
facade_exact_slot_field_op_count=29
top_facade_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
top_facade_method_pct=10.53
dominant_field_family=facade_receiver_state
dominant_field_family_count=15
field_family.facade_receiver_state_count=15
field_family.page_model_bridge_count=1
field_family.page_queue_bridge_count=9
field_family.alloc_result_capsule_count=4
field_family.release_result_capsule_count=0
field_family.temporary_status_result_count=0
field_family.unknown_count=0
pattern.same_block_get_set_count=3
pattern.same_receiver_repeated_get_count=1
pattern.write_only_field_count=6
pattern.read_only_field_count=16
pattern.positive_net_cache_candidate_count=4
selected_next=facade_field_owner_selection
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
facade_method_0_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
facade_method_0_pct=10.53
facade_method_0_field_op_count=17
facade_method_1_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
facade_method_1_pct=2.58
facade_method_1_field_op_count=7
facade_method_2_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
facade_method_2_pct=1.76
facade_method_2_field_op_count=4
facade_method_3_symbol=HakoAllocObjectLifecycleFacade.resetReleaseResult/0
facade_method_3_pct=0.81
facade_method_3_field_op_count=1
summary=ok
```

## Decision

```text
selected_next=facade_field_owner_selection
selected_reason=facade_receiver_state_is_largest_but_positive_net_candidates_remain_small
next_row=facade_field_owner_selection_after_release_known_live_rollback
optimization_open=0
```

The facade family is still the top unblocked exact-slot family, but the
positive-net surface is narrow: three same-block get/set pairs and one repeated
same-receiver get. The next row must choose whether this supports a small
facade owner, or whether the opportunity is too small and another ownership
refresh is cleaner.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_facade_exact_slot_ir_shape_diff_inventory_after_release_known_live_rollback_guard.sh
```
