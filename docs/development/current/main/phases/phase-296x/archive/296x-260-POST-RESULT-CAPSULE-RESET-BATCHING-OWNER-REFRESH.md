---
Status: Landed
Date: 2026-05-29
Scope: refresh exact-slot ownership after result capsule reset batching keeper.
Blocker: POST-RESULT-CAPSULE-RESET-BATCHING-OWNER-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-259-RESULT-CAPSULE-RESET-FIELD-BATCHING-MEASUREMENT.md
---

# 296x-260 Post Result Capsule Reset Batching Owner Refresh

## Purpose

Refresh exact-slot ownership after row259 accepted the result capsule reset
field-batching keeper.

This row does not implement another keeper. It reopens observation from a fresh
perf callgraph and selects weighted owner selection as the next diagnostic
boundary.

## Evidence

```text
output_contract=post-result-capsule-reset-batching-owner-refresh-v0
input_contract=result-capsule-reset-field-batching-measurement-v0
workload_id=representative-object-lifecycle-small-block-v0
attribution_source=perf_callgraph
callgraph_attribution_available=1
perf_sample_count=125
exact_slot_get_set_pct=52.67
attributed_callsite_count=29
top_callsite_pct=5.06
top_callsite_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
top_callsite_helper=nyash.object.exact_slot_get_i64_hii
dominant_family=page_model_hotpath
dominant_family_pct=16.81
recent_nonkeeper_family=page_queue_helpers
recent_nonkeeper_row=296x-241
recent_nonkeeper_family_pct=12.45
recent_nonkeeper_candidate_count=21
recent_nonkeeper_hot_per_candidate_pct=0.59
dominant_family_is_recent_nonkeeper=0
recent_nonkeeper_family_blocked_for_immediate_keeper=1
top_unblocked_family=page_model_hotpath
top_unblocked_family_pct=16.81
family_0_name=page_model_hotpath
family_0_pct=16.81
family_0_known_candidate_count=9
family_0_hot_per_candidate_pct=1.87
family_1_name=page_queue_helpers
family_1_pct=12.45
family_1_known_candidate_count=21
family_1_hot_per_candidate_pct=0.59
family_2_name=object_lifecycle_facade
family_2_pct=11.33
family_2_known_candidate_count=4
family_2_hot_per_candidate_pct=2.83
family_3_name=release_result_capsule
family_3_pct=5.40
family_4_name=alloc_result_capsule
family_4_pct=3.60
helper_0_symbol=nyash.object.exact_slot_get_i64_hii
helper_0_pct=11.77
helper_1_symbol=nyash.object.exact_slot_set_i64_hii
helper_1_pct=11.55
helper_2_symbol=nyash.object.exact_slot_get_handle_hii
helper_2_pct=9.88
helper_3_symbol=nyash.object.exact_slot_get_u64_hii
helper_3_pct=9.02
helper_4_symbol=nyash.object.exact_slot_set_u64_hiu
helper_4_pct=7.75
helper_5_symbol=nyash.object.exact_slot_set4_i64_hiiiii
helper_5_pct=1.80
helper_6_symbol=nyash.object.exact_slot_set_handle_hii
helper_6_pct=0.90
result_capsule_combined_pct=9.00
result_capsule_reset_batch_helper_pct=1.80
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
selected_reason=page_model_hotpath_is_current_dominant_family_after_result_capsule_reset_batching
next_row=weighted_exact_slot_owner_selection
optimization_open=0
```

The next row must choose one owner from weighted evidence. It must not retry
page queue immediately, and it must not implement a keeper without a fresh
IR-shape or field-traffic plan.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_post_result_capsule_reset_batching_owner_refresh_guard.sh
```
