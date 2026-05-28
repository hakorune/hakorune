---
Status: Landed
Date: 2026-05-29
Scope: attribute remaining exact-slot typed-object helper cost by callsite.
Blocker: TYPED-OBJECT-EXACT-SLOT-CALLSITE-ATTRIBUTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-224-POST-RMW-FUSION-OWNER-REFRESH.md
---

# 296x-225 Typed-Object Exact-Slot Callsite Attribution

## Purpose

Attribute the remaining exact-slot typed-object helper cost by callsite and
method family before another keeper.

This row does not optimize. It exists to avoid another broad source-level or
MIR-level guess.

## Evidence

```text
output_contract=typed-object-exact-slot-callsite-attribution-v0
input_contract=typed-object-post-rmw-fusion-owner-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
exact_slot_get_set_pct=59.97
attributed_callsite_count=30
top_callsite_pct=4.54
top_callsite_symbol=HakoAllocPageModel.acquire_usize/1
top_callsite_helper=nyash.object.exact_slot_get_u64_hii
dominant_family=object_lifecycle_facade
dominant_family_pct=18.52
selected_boundary=exact_slot_callsite_owner_selection
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
family_0_name=object_lifecycle_facade
family_0_pct=18.52
family_1_name=page_model_hotpath
family_1_pct=14.72
family_2_name=page_queue_helpers
family_2_pct=11.46
family_3_name=alloc_result_capsule
family_3_pct=9.11
family_4_name=release_result_capsule
family_4_pct=6.17
helper_0_symbol=nyash.object.exact_slot_set_i64_hii
helper_0_pct=16.00
helper_1_symbol=nyash.object.exact_slot_get_u64_hii
helper_1_pct=11.42
helper_2_symbol=nyash.object.exact_slot_get_handle_hii
helper_2_pct=9.53
helper_3_symbol=nyash.object.exact_slot_set_u64_hiu
helper_3_pct=9.20
helper_4_symbol=nyash.object.exact_slot_get_i64_hii
helper_4_pct=6.94
helper_5_symbol=nyash.object.exact_slot_set_handle_hii
helper_5_pct=6.88
callsite_0_pct=4.54
callsite_0_symbol=HakoAllocPageModel.acquire_usize/1
callsite_0_helper=nyash.object.exact_slot_get_u64_hii
callsite_1_pct=3.84
callsite_1_symbol=HakoAllocPageModel.acquire_usize/1
callsite_1_helper=nyash.object.exact_slot_set_u64_hiu
callsite_2_pct=3.81
callsite_2_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
callsite_2_helper=nyash.object.exact_slot_set_handle_hii
summary=ok
```

## Decision

```text
selected_owner_family=exact_slot_callsite_owner_selection
selected_reason=remaining_exact_slot_cost_is_spread_across_facade_page_model_page_queue_and_result_capsule_families
next_row=exact_slot_callsite_owner_selection
```

The strongest single callsite is still `HakoAllocPageModel.acquire_usize/1`,
but the largest family bucket is `object_lifecycle_facade`. The next row should
choose one narrow owner from this attribution instead of reopening broad
typed-field residence or source expansion.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_typed_object_exact_slot_callsite_attribution_guard.sh
```
