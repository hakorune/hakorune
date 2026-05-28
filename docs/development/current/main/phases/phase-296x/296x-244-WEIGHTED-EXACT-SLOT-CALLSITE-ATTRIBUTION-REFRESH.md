---
Status: Landed
Date: 2026-05-29
Scope: refresh exact-slot callsite attribution with recent non-keeper weighting.
Blocker: WEIGHTED-EXACT-SLOT-CALLSITE-ATTRIBUTION-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-243-POST-PAGE-QUEUE-ROLLBACK-OWNER-REFRESH.md
---

# 296x-244 Weighted Exact-Slot Callsite Attribution Refresh

## Purpose

Attribute post-rollback exact-slot get/set helper cost with the page queue
non-keeper evidence in view.

This row does not open another optimization. It fixes the previous blind spot:
static candidate count and projected helper-call reduction can over-predict a
keeper when the hot callsites are diffuse or the emitted IR shape gets worse.

## Evidence

```text
output_contract=weighted-exact-slot-callsite-attribution-refresh-v0
input_contract=post-page-queue-rollback-owner-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
attribution_source=perf_callgraph
callgraph_attribution_available=1
exact_slot_get_set_pct=54.29
attributed_callsite_count=28
top_callsite_pct=4.60
top_callsite_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
top_callsite_helper=nyash.object.exact_slot_get_i64_hii
dominant_family=page_queue_helpers
dominant_family_pct=16.45
recent_nonkeeper_family=page_queue_helpers
recent_nonkeeper_row=296x-241
recent_nonkeeper_family_pct=16.45
recent_nonkeeper_candidate_count=21
recent_nonkeeper_hot_per_candidate_pct=0.78
dominant_family_is_recent_nonkeeper=1
recent_nonkeeper_family_blocked_for_immediate_keeper=1
top_unblocked_family=page_model_hotpath
top_unblocked_family_pct=15.29
static_candidate_count_only_rejected=1
weighted_hot_candidate_score_required=1
ir_shape_diff_required_before_next_keeper=1
sample_count_3_required_for_keeper_decision=1
selected_boundary=weighted_exact_slot_owner_selection
next_diagnostic=weighted_exact_slot_owner_selection
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
family_0_name=page_queue_helpers
family_0_pct=16.45
family_0_known_candidate_count=21
family_0_hot_per_candidate_pct=0.78
family_0_recent_nonkeeper=1
family_1_name=page_model_hotpath
family_1_pct=15.29
family_1_known_candidate_count=0
family_1_hot_per_candidate_pct=0.00
family_1_recent_nonkeeper=0
family_2_name=object_lifecycle_facade
family_2_pct=10.53
family_2_known_candidate_count=0
family_2_hot_per_candidate_pct=0.00
family_2_recent_nonkeeper=0
family_3_name=release_result_capsule
family_3_pct=6.05
family_3_known_candidate_count=0
family_3_hot_per_candidate_pct=0.00
family_3_recent_nonkeeper=0
family_4_name=alloc_result_capsule
family_4_pct=5.98
family_4_known_candidate_count=0
family_4_hot_per_candidate_pct=0.00
family_4_recent_nonkeeper=0
helper_0_symbol=nyash.object.exact_slot_set_i64_hii
helper_0_pct=17.17
helper_1_symbol=nyash.object.exact_slot_get_i64_hii
helper_1_pct=15.51
helper_2_symbol=nyash.object.exact_slot_get_u64_hii
helper_2_pct=11.38
helper_3_symbol=nyash.object.exact_slot_set_u64_hiu
helper_3_pct=4.23
helper_4_symbol=nyash.object.exact_slot_get_handle_hii
helper_4_pct=3.44
helper_5_symbol=nyash.object.exact_slot_set_handle_hii
helper_5_pct=2.56
callsite_0_pct=4.60
callsite_0_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
callsite_0_helper=nyash.object.exact_slot_get_i64_hii
callsite_1_pct=4.18
callsite_1_symbol=HakoAllocPageModel.acquire_usize/1
callsite_1_helper=nyash.object.exact_slot_get_i64_hii
callsite_2_pct=3.66
callsite_2_symbol=HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
callsite_2_helper=nyash.object.exact_slot_get_u64_hii
callsite_3_pct=3.52
callsite_3_symbol=HakoAllocObjectLifecycleReleaseResult.recordSuccess/2
callsite_3_helper=nyash.object.exact_slot_set_i64_hii
callsite_4_pct=3.44
callsite_4_symbol=HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
callsite_4_helper=nyash.object.exact_slot_get_u64_hii
summary=ok
```

## Decision

```text
selected_owner_family=weighted_exact_slot_owner_selection
selected_reason=dominant_family_page_queue_helpers_is_recent_nonkeeper_and_requires_ir_shape_diff_before_retry
next_row=weighted_exact_slot_owner_selection
optimization_open=0
```

Page queue remains the largest exact-slot caller family after rollback, but it
is also the recent non-keeper. Immediate page-queue retry is therefore closed.
The next row must select an owner using weighted callsite evidence and must
require IR-shape diff evidence before any implementation row.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_weighted_exact_slot_callsite_attribution_refresh_guard.sh
```
