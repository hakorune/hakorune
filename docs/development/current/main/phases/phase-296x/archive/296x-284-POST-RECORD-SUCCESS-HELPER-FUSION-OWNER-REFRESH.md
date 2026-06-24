---
Status: Landed
Date: 2026-05-29
Scope: refresh hot ownership after recordSuccess helper fusion.
Blocker: POST-RECORD-SUCCESS-HELPER-FUSION-OWNER-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-283-RECORD-SUCCESS-HELPER-FUSION-MEASUREMENT.md
---

# 296x-284 Post RecordSuccess Helper Fusion Owner Refresh

## Purpose

Refresh weighted exact-slot ownership after the row283 recordSuccess keeper.

This row does not implement another keeper. It records the new perf callgraph
shape and selects weighted owner selection as the next boundary.

## Evidence

```text
output_contract=post-record-success-helper-fusion-owner-refresh-v0
input_contract=record-success-helper-fusion-measurement-v0
workload_id=representative-object-lifecycle-small-block-v0
attribution_source=perf_callgraph
callgraph_attribution_available=1
exact_slot_get_set_pct=50.97
attributed_callsite_count=30
top_callsite_pct=3.13
top_callsite_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
top_callsite_helper=nyash.object.exact_slot_get_i64_hii
dominant_family=page_queue_helpers
dominant_family_pct=14.32
recent_nonkeeper_family=page_queue_helpers
recent_nonkeeper_row=296x-241
recent_nonkeeper_family_pct=14.32
recent_nonkeeper_candidate_count=21
recent_nonkeeper_hot_per_candidate_pct=0.68
dominant_family_is_recent_nonkeeper=1
recent_nonkeeper_family_blocked_for_immediate_keeper=1
top_unblocked_family=object_lifecycle_facade
top_unblocked_family_pct=13.47
family_0_name=page_queue_helpers
family_0_pct=14.32
family_1_name=object_lifecycle_facade
family_1_pct=13.47
family_2_name=page_model_hotpath
family_2_pct=11.73
family_3_name=release_result_capsule
family_3_pct=2.59
family_4_name=alloc_result_capsule
family_4_pct=2.19
helper_0_symbol=nyash.object.exact_slot_get_u64_hii
helper_0_pct=9.49
helper_1_symbol=nyash.object.exact_slot_get_i64_hii
helper_1_pct=9.31
helper_2_symbol=nyash.object.exact_slot_get_handle_hii
helper_2_pct=8.63
helper_3_symbol=nyash.object.exact_slot_set_i64_hii
helper_3_pct=7.50
helper_4_symbol=nyash.object.exact_slot_set_u64_hiu
helper_4_pct=5.69
helper_5_symbol=nyash.object.exact_slot_set4_i64_hiiiii
helper_5_pct=3.30
helper_6_symbol=nyash.object.exact_slot_set_handle_hii
helper_6_pct=3.03
helper_7_symbol=nyash.object.exact_slot_record_alloc_success_hii
helper_7_pct=2.55
helper_8_symbol=nyash.object.exact_slot_record_release_success_hiii
helper_8_pct=1.47
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
selected_reason=page_queue_is_dominant_but_recent_no_effect_family_requires_unblocked_weighted_selection
next_row=weighted_exact_slot_owner_selection_after_record_success_helper_fusion
optimization_open=0
```

The next row must choose one owner from weighted evidence. Immediate page-queue
retry remains blocked by row241 no-effect evidence unless a new IR shape diff
shows a positive-net plan.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_post_record_success_helper_fusion_owner_refresh_guard.sh
```
