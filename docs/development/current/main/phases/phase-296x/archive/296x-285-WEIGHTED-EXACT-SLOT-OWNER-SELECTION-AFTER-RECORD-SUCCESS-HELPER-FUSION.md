---
Status: Landed
Date: 2026-05-29
Scope: select next exact-slot owner after recordSuccess helper fusion.
Blocker: WEIGHTED-EXACT-SLOT-OWNER-SELECTION-AFTER-RECORD-SUCCESS-HELPER-FUSION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-284-POST-RECORD-SUCCESS-HELPER-FUSION-OWNER-REFRESH.md
---

# 296x-285 Weighted Exact-Slot Owner Selection After RecordSuccess Helper Fusion

## Purpose

Select the next exact-slot owner from row284 weighted evidence.

This row does not implement a keeper. It keeps immediate page-queue retry
blocked because page queue was a recent no-effect family, then selects the top
unblocked family for IR shape inventory.

## Evidence

```text
output_contract=weighted-exact-slot-owner-selection-v0
input_contract=weighted-exact-slot-callsite-attribution-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
dominant_family=page_queue_helpers
dominant_family_pct=14.32
recent_nonkeeper_family=page_queue_helpers
recent_nonkeeper_row=296x-241
recent_nonkeeper_candidate_count=21
recent_nonkeeper_hot_per_candidate_pct=0.68
dominant_family_is_recent_nonkeeper=1
top_unblocked_family=object_lifecycle_facade
top_unblocked_family_pct=13.47
selected_family=object_lifecycle_facade
selected_owner=facade_exact_slot_ir_shape_diff_inventory
selected_reason=dominant_family_is_recent_nonkeeper_select_top_unblocked_family_with_ir_shape_diff
next_diagnostic=facade_exact_slot_ir_shape_diff_inventory
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
selected_owner_family=facade_exact_slot_ir_shape_diff_inventory
selected_reason=page_queue_blocked_select_object_lifecycle_facade_as_top_unblocked_family
next_row=facade_exact_slot_ir_shape_diff_inventory
optimization_open=0
```

The next row must inventory the facade exact-slot IR shape before any new
facade implementation. Page-queue retry stays blocked until a new positive-net
IR shape exists.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_weighted_exact_slot_owner_selection_after_record_success_helper_fusion_guard.sh
```
