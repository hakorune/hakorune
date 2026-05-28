---
Status: Landed
Date: 2026-05-29
Scope: select the next exact-slot owner after result capsule reset batching owner refresh.
Blocker: WEIGHTED-EXACT-SLOT-OWNER-SELECTION-AFTER-RESULT-CAPSULE-RESET-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-260-POST-RESULT-CAPSULE-RESET-BATCHING-OWNER-REFRESH.md
---

# 296x-261 Weighted Exact-Slot Owner Selection After Result Capsule Reset

## Purpose

Select the next exact-slot owner from row260 weighted evidence.

This row does not implement another keeper. It chooses the next observation
owner and keeps two stop-lines active:

- `page_queue_helpers` is still blocked for immediate retry because row241 was
  a recent page-queue non-keeper.
- `page_model_hotpath` is selected only as a fresh IR-shape refresh owner,
  because row252 already showed no material body-time effect for the previous
  page-model receiver-forwarding keeper.

## Evidence

```text
output_contract=weighted-exact-slot-owner-selection-after-result-capsule-reset-v0
input_contract=post-result-capsule-reset-batching-owner-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
exact_slot_get_set_pct=52.67
dominant_family=page_model_hotpath
dominant_family_pct=16.81
dominant_family_is_recent_nonkeeper=0
recent_nonkeeper_family=page_queue_helpers
recent_nonkeeper_row=296x-241
recent_nonkeeper_family_pct=12.45
page_queue_immediate_retry_blocked=1
top_unblocked_family=page_model_hotpath
top_unblocked_family_pct=16.81
page_model_recent_no_material_effect_row=296x-252
page_model_immediate_implementation_blocked=1
page_model_ir_shape_refresh_required=1
object_lifecycle_facade_pct=11.33
result_capsule_combined_pct=9.00
result_capsule_reset_batch_helper_pct=1.80
selected_family=page_model_hotpath
selected_owner=page_model_hotpath_ir_shape_diff_refresh
selected_reason=dominant_page_model_family_requires_fresh_ir_shape_after_prior_no_material_receiver_forwarding
next_diagnostic=page_model_hotpath_ir_shape_diff_refresh
rejected_owner=page_queue_immediate_retry
rejected_reason=recent_nonkeeper_requires_ir_shape_diff_before_retry
rejected_owner_1=page_model_immediate_implementation
rejected_reason_1=receiver_forwarding_no_material_effect_requires_fresh_ir_shape_before_retry
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
selected_owner_family=page_model_hotpath_ir_shape_diff_refresh
selected_reason=page_model_hotpath_is_current_dominant_family_but_prior_receiver_forwarding_no_material_effect_blocks_immediate_implementation
next_row=page_model_hotpath_ir_shape_diff_refresh
optimization_open=0
```

The next row must refresh the page-model hotpath IR shape and field/copy
traffic after the result capsule reset batching keeper. It must not implement a
page-model keeper until the refreshed report identifies a positive-net owner
that is different from the row252 receiver-forwarding no-effect path.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_weighted_exact_slot_owner_selection_after_result_capsule_reset_guard.sh
```
