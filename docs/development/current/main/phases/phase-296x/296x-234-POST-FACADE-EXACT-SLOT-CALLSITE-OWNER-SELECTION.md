---
Status: Landed
Date: 2026-05-29
Scope: choose one next owner from refreshed post-facade exact-slot callsite attribution.
Blocker: POST-FACADE-EXACT-SLOT-CALLSITE-OWNER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-233-POST-FACADE-EXACT-SLOT-CALLSITE-ATTRIBUTION-REFRESH.md
---

# 296x-234 Post Facade Exact-Slot Callsite Owner Selection

## Purpose

Choose one narrow next owner from row233 attribution before another
optimization row.

This row keeps optimization closed. The selected facade same-block get/set
fusion already landed, so the next row must not blindly repeat that keeper or
reopen broad typed-field residence.

## Evidence

```text
output_contract=post-facade-exact-slot-callsite-owner-selection-v0
input_contract=typed-object-exact-slot-callsite-attribution-v0
workload_id=representative-object-lifecycle-small-block-v0
dominant_family=object_lifecycle_facade
dominant_family_pct=17.36
top_callsite_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
top_callsite_pct=4.15
selected_owner=object_lifecycle_facade_residual_exact_slot_field_inventory
selected_reason=dominant_facade_family_remains_after_selected_fusion
next_diagnostic=object_lifecycle_facade_residual_exact_slot_field_inventory
rejected_owner=repeat_selected_facade_same_block_get_set_fusion
rejected_reason=selected_facade_get_set_fusion_already_landed_and_residual_shape_needs_inventory
rejected_owner_1=generic_typed_field_residence_retry
rejected_reason_1=no_family_specific_positive_net_plan
rejected_owner_2=page_queue_followon_keeper
rejected_reason_2=page_queue_is_secondary_family_after_facade
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_owner=object_lifecycle_facade_residual_exact_slot_field_inventory
next_row=object_lifecycle_facade_residual_exact_slot_field_inventory
```

The refreshed attribution still has `object_lifecycle_facade` as the largest
family at 17.36%. The top individual callsite is now
`objectLifecycleSmallAlloc/1`, so the next step is a residual facade inventory
against the post-fusion shape, not another implementation row.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_post_facade_exact_slot_callsite_owner_selection_guard.sh
```
