---
Status: Landed
Date: 2026-05-29
Scope: refresh the hot owner after rolling back the page queue non-keeper.
Blocker: POST-PAGE-QUEUE-ROLLBACK-OWNER-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-242-ROLLBACK-SELECTED-PAGE-QUEUE-SAME-BLOCK-GET-SET.md
---

# 296x-243 Post Page Queue Rollback Owner Refresh

## Purpose

Refresh the perf owner after rolling back the selected page queue same-block
get/set non-keeper.

This row also records the lesson from row241: static candidate count and net
helper-call delta are insufficient keeper predictors. The next diagnostic must
weight candidates by hot callsite evidence and inspect emitted IR shape before
another implementation row.

## Evidence

```text
output_contract=post-page-queue-rollback-owner-refresh-v0
input_contract=rollback-selected-page-queue-same-block-get-set-v0
workload_id=representative-object-lifecycle-small-block-v0
perf_exact_slot_helper_pct=58.57
perf_exact_slot_get_set_pct=54.29
perf_exact_slot_rmw_helper_pct=4.28
perf_legacy_field_helper_pct=0.00
perf_array_slot_backend_pct=16.38
perf_array_backend_hash_pct=18.93
perf_array_total_pct=35.31
perf_hako_method_pct=5.25
selected_boundary=weighted_exact_slot_callsite_attribution_refresh
next_diagnostic=weighted_exact_slot_callsite_attribution_refresh
static_candidate_count_only_rejected=1
weighted_hot_candidate_score_required=1
ir_shape_diff_required_before_next_keeper=1
sample_count_3_required_for_keeper_decision=1
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
perf_top_0_pct=17.22
perf_top_0_symbol=core::hash::BuildHasher::hash_one
perf_top_1_pct=17.17
perf_top_1_symbol=nyash.object.exact_slot_set_i64_hii
perf_top_2_pct=15.51
perf_top_2_symbol=nyash.object.exact_slot_get_i64_hii
perf_top_3_pct=14.70
perf_top_3_symbol=nyash_kernel::plugin::array_slot_backend::single_thread_store_i64
perf_top_4_pct=11.38
perf_top_4_symbol=nyash.object.exact_slot_get_u64_hii
perf_top_5_pct=4.28
perf_top_5_symbol=nyash.object.exact_slot_rmw_add_u64_hiii
perf_top_6_pct=4.23
perf_top_6_symbol=nyash.object.exact_slot_set_u64_hiu
perf_top_7_pct=3.44
perf_top_7_symbol=nyash.object.exact_slot_get_handle_hii
perf_top_8_pct=2.56
perf_top_8_symbol=nyash.object.exact_slot_set_handle_hii
perf_top_9_pct=1.77
perf_top_9_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
summary=ok
```

## Decision

```text
selected_owner_family=weighted_exact_slot_callsite_attribution_refresh
selected_reason=exact_slot_get_set_helpers_remain_primary_but_candidate_count_prediction_failed
next_row=weighted_exact_slot_callsite_attribution_refresh
optimization_open=0
```

Exact-slot get/set helpers remain the largest owner family after rollback.
However, the page queue non-keeper showed that helper-count reduction alone is
not enough. The next row must classify exact-slot callsites with a weighted
hot-candidate score and must require IR-shape diff evidence before another
keeper implementation.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_post_page_queue_rollback_owner_refresh_guard.sh
```
