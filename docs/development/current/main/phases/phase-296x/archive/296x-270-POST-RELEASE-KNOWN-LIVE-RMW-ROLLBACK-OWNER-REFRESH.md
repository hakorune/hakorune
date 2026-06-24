---
Status: Landed
Date: 2026-05-29
Scope: refresh hot ownership after releaseKnownLive RMW rollback.
Blocker: POST-RELEASE-KNOWN-LIVE-RMW-ROLLBACK-OWNER-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-269-PAGE-MODEL-RELEASE-KNOWN-LIVE-SINGLE-USE-RMW-ROLLBACK.md
---

# 296x-270 Post Release Known Live RMW Rollback Owner Refresh

## Purpose

Refresh hot ownership after rolling back the no-effect
`releaseLocalKnownLive/1` RMW implementation.

This row does not implement a keeper. It reopens observation from a fresh perf
callgraph and selects weighted exact-slot owner selection as the next boundary.

## Evidence

```text
output_contract=post-release-known-live-rmw-rollback-owner-refresh-v0
input_contract=page-model-release-known-live-single-use-rmw-rollback-v0
workload_id=representative-object-lifecycle-small-block-v0
attribution_source=perf_callgraph
callgraph_attribution_available=1
exact_slot_get_set_pct=49.64
attributed_callsite_count=26
top_callsite_pct=6.09
top_callsite_symbol=HakoAllocObjectLifecycleAllocResult.recordSuccess/1
top_callsite_helper=nyash.object.exact_slot_set_i64_hii
dominant_family=object_lifecycle_facade
dominant_family_pct=15.68
recent_nonkeeper_family=page_model_hotpath
recent_nonkeeper_row=296x-268
recent_nonkeeper_family_pct=11.34
recent_nonkeeper_candidate_count=9
recent_nonkeeper_hot_per_candidate_pct=1.26
dominant_family_is_recent_nonkeeper=0
recent_nonkeeper_family_blocked_for_immediate_keeper=1
top_unblocked_family=object_lifecycle_facade
top_unblocked_family_pct=15.68
family_0_name=object_lifecycle_facade
family_0_pct=15.68
family_0_known_candidate_count=4
family_0_hot_per_candidate_pct=3.92
family_1_name=page_model_hotpath
family_1_pct=11.34
family_1_known_candidate_count=9
family_1_hot_per_candidate_pct=1.26
family_2_name=alloc_result_capsule
family_2_pct=8.71
family_3_name=page_queue_helpers
family_3_pct=7.74
family_3_known_candidate_count=21
family_3_hot_per_candidate_pct=0.37
family_4_name=release_result_capsule
family_4_pct=4.40
helper_0_symbol=nyash.object.exact_slot_set_i64_hii
helper_0_pct=13.85
helper_1_symbol=nyash.object.exact_slot_set_u64_hiu
helper_1_pct=9.58
helper_2_symbol=nyash.object.exact_slot_get_u64_hii
helper_2_pct=8.74
helper_3_symbol=nyash.object.exact_slot_get_i64_hii
helper_3_pct=6.99
helper_4_symbol=nyash.object.exact_slot_get_handle_hii
helper_4_pct=6.96
helper_5_symbol=nyash.object.exact_slot_set_handle_hii
helper_5_pct=2.64
helper_6_symbol=nyash.object.exact_slot_set4_i64_hiiiii
helper_6_pct=0.88
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
summary=ok
```

## Decision

```text
selected_owner_family=weighted_exact_slot_owner_selection
selected_reason=object_lifecycle_facade_is_current_dominant_family_after_page_model_no_effect_rollback
next_row=weighted_exact_slot_owner_selection
optimization_open=0
```

The next row must choose one owner from weighted evidence. It must keep
immediate page-model retry blocked after the row268 no-effect measurement.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_post_release_known_live_rmw_rollback_owner_refresh_guard.sh
```
