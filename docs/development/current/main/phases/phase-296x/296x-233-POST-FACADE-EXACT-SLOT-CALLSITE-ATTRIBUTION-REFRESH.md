---
Status: Landed
Date: 2026-05-29
Scope: refresh exact-slot callsite attribution after selected facade get/set fusion.
Blocker: POST-FACADE-EXACT-SLOT-CALLSITE-ATTRIBUTION-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-232-POST-SELECTED-FACADE-GET-SET-OWNER-REFRESH.md
---

# 296x-233 Post Facade Exact-Slot Callsite Attribution Refresh

## Purpose

Refresh exact-slot get/set helper callsite attribution after row231 accepted the
selected-facade same-block get/set fusion keeper.

This row uses `perf record --call-graph dwarf,4096` because frame-pointer call
graphs did not preserve useful callers for the exact-EXE helper frames.

## Evidence

```text
output_contract=typed-object-exact-slot-callsite-attribution-v0
input_contract=post-selected-facade-get-set-owner-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
attribution_source=perf_callgraph
callgraph_attribution_available=1
exact_slot_get_set_pct=56.37
attributed_callsite_count=29
top_callsite_pct=4.15
top_callsite_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
top_callsite_helper=nyash.object.exact_slot_get_i64_hii
dominant_family=object_lifecycle_facade
dominant_family_pct=17.36
selected_boundary=exact_slot_callsite_owner_selection
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
family_0_name=object_lifecycle_facade
family_0_pct=17.36
family_1_name=page_queue_helpers
family_1_pct=13.19
family_2_name=page_model_hotpath
family_2_pct=10.57
family_3_name=alloc_result_capsule
family_3_pct=6.97
family_4_name=release_result_capsule
family_4_pct=5.50
helper_0_symbol=nyash.object.exact_slot_set_i64_hii
helper_0_pct=19.44
helper_1_symbol=nyash.object.exact_slot_get_i64_hii
helper_1_pct=10.38
helper_2_symbol=nyash.object.exact_slot_get_handle_hii
helper_2_pct=9.92
helper_3_symbol=nyash.object.exact_slot_get_u64_hii
helper_3_pct=8.96
helper_4_symbol=nyash.object.exact_slot_set_u64_hiu
helper_4_pct=4.89
helper_5_symbol=nyash.object.exact_slot_set_handle_hii
helper_5_pct=2.78
callsite_0_pct=4.15
callsite_0_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
callsite_0_helper=nyash.object.exact_slot_get_i64_hii
callsite_1_pct=4.14
callsite_1_symbol=HakoAllocObjectLifecycleReleaseResult.reset/0
callsite_1_helper=nyash.object.exact_slot_set_i64_hii
callsite_2_pct=3.54
callsite_2_symbol=HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
callsite_2_helper=nyash.object.exact_slot_set_i64_hii
summary=ok
```

## Decision

```text
selected_owner_family=exact_slot_callsite_owner_selection
selected_reason=remaining_exact_slot_get_set_cost_is_still_spread_but_facade_family_remains_largest
next_row=exact_slot_callsite_owner_selection_refresh
optimization_open=0
```

The selected facade fusion reduced some field traffic, but exact-slot get/set
helper cost is still the primary family. The largest family remains
`object_lifecycle_facade`, while the top individual callsite is now
`objectLifecycleSmallAlloc/1`.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_post_facade_exact_slot_callsite_attribution_refresh_guard.sh
```
