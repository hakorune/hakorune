---
Status: Landed
Date: 2026-05-29
Scope: select next exact-slot owner after releaseKnownLive rollback owner refresh.
Blocker: WEIGHTED-EXACT-SLOT-OWNER-SELECTION-AFTER-RELEASE-KNOWN-LIVE-ROLLBACK-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-270-POST-RELEASE-KNOWN-LIVE-RMW-ROLLBACK-OWNER-REFRESH.md
---

# 296x-271 Weighted Exact-Slot Owner Selection After Release Known Live Rollback

## Purpose

Select the next exact-slot owner from row270 weighted evidence.

This row does not implement a keeper. It selects
`object_lifecycle_facade` as the next observation owner because it is the
current dominant family after page-model rollback, while immediate page-model
retry remains blocked by row268 no-effect evidence.

## Evidence

```text
output_contract=weighted-exact-slot-owner-selection-v0
input_contract=weighted-exact-slot-callsite-attribution-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
dominant_family=object_lifecycle_facade
dominant_family_pct=15.68
recent_nonkeeper_family=page_model_hotpath
recent_nonkeeper_row=296x-268
recent_nonkeeper_candidate_count=9
recent_nonkeeper_hot_per_candidate_pct=1.26
dominant_family_is_recent_nonkeeper=0
top_unblocked_family=object_lifecycle_facade
top_unblocked_family_pct=15.68
selected_family=object_lifecycle_facade
selected_owner=facade_exact_slot_ir_shape_diff_inventory
selected_reason=dominant_family_not_recent_nonkeeper
next_diagnostic=facade_exact_slot_ir_shape_diff_inventory
rejected_owner=page_model_immediate_retry
rejected_reason=recent_nonkeeper_requires_ir_shape_diff_before_retry
rejected_owner_1=static_candidate_count_only_selection
rejected_reason_1=row241_candidate_count_prediction_failed
rejected_owner_2=implementation_without_ir_shape_diff
rejected_reason_2=ir_shape_diff_required_before_next_keeper
weighted_hot_candidate_score_required=1
ir_shape_diff_required_before_next_keeper=1
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_owner_family=facade_exact_slot_ir_shape_diff_inventory
selected_reason=object_lifecycle_facade_is_current_dominant_unblocked_family_after_page_model_no_effect_rollback
next_row=facade_exact_slot_ir_shape_diff_inventory
optimization_open=0
```

The next row must inventory residual facade exact-slot traffic before any
facade implementation. It must not reopen page-model helper tweaks without
fresh ownership evidence.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_weighted_exact_slot_owner_selection_after_release_known_live_rollback_guard.sh
```
