---
Status: Landed
Date: 2026-05-29
Scope: select the next exact-slot owner after weighted attribution.
Blocker: WEIGHTED-EXACT-SLOT-OWNER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-244-WEIGHTED-EXACT-SLOT-CALLSITE-ATTRIBUTION-REFRESH.md
---

# 296x-245 Weighted Exact-Slot Owner Selection

## Purpose

Select the next owner without repeating the page queue non-keeper pattern.

The dominant family is still `page_queue_helpers`, but it is the recent
non-keeper. This row therefore rejects immediate page-queue retry and selects
the top unblocked family for an IR-shape-diff inventory before any keeper.

## Evidence

```text
output_contract=weighted-exact-slot-owner-selection-v0
input_contract=weighted-exact-slot-callsite-attribution-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
dominant_family=page_queue_helpers
dominant_family_pct=16.45
recent_nonkeeper_family=page_queue_helpers
recent_nonkeeper_row=296x-241
recent_nonkeeper_candidate_count=21
recent_nonkeeper_hot_per_candidate_pct=0.78
dominant_family_is_recent_nonkeeper=1
top_unblocked_family=page_model_hotpath
top_unblocked_family_pct=15.29
selected_family=page_model_hotpath
selected_owner=page_model_hotpath_ir_shape_diff_inventory
selected_reason=dominant_family_is_recent_nonkeeper_select_top_unblocked_family_with_ir_shape_diff
next_diagnostic=page_model_hotpath_ir_shape_diff_inventory
rejected_owner=page_queue_immediate_retry
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
selected_owner_family=page_model_hotpath_ir_shape_diff_inventory
selected_reason=page_model_is_top_unblocked_family_after_page_queue_nonkeeper
next_row=page_model_hotpath_ir_shape_diff_inventory
optimization_open=0
```

This is still an observation row. The next row must inspect page-model hotpath
IR shape before implementing another helper fusion, source rewrite, or
residence transform.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_weighted_exact_slot_owner_selection_guard.sh
```
