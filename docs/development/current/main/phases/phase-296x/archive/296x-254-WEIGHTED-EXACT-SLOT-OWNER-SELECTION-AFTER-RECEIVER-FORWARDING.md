---
Status: Landed
Date: 2026-05-29
Scope: select next exact-slot owner after receiver forwarding owner refresh.
Blocker: WEIGHTED-EXACT-SLOT-OWNER-SELECTION-AFTER-RECEIVER-FORWARDING-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-253-POST-RECEIVER-FORWARDING-OWNER-REFRESH.md
---

# 296x-254 Weighted Exact-Slot Owner Selection After Receiver Forwarding

## Purpose

Select the next diagnostic owner from row253 weighted exact-slot evidence.

This selection adds one guardrail beyond the older row245 selection rule:
`page_model_hotpath` is the top unblocked family by raw pct, but the immediately
preceding page-model receiver-forwarding implementation produced no material
body-time effect. Re-entering page model without a different owner hypothesis
would repeat the same local path.

## Evidence

```text
output_contract=weighted-exact-slot-owner-selection-after-receiver-forwarding-v0
input_contract=weighted-exact-slot-callsite-attribution-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
dominant_family=page_queue_helpers
dominant_family_pct=12.11
dominant_family_is_recent_nonkeeper=1
recent_nonkeeper_family=page_queue_helpers
recent_nonkeeper_row=296x-241
page_queue_immediate_retry_blocked=1
top_unblocked_family=page_model_hotpath
top_unblocked_family_pct=10.75
page_model_recent_no_material_effect_row=296x-252
page_model_immediate_retry_blocked=1
object_lifecycle_facade_pct=9.85
alloc_result_capsule_pct=8.30
release_result_capsule_pct=8.12
combined_result_capsule_pct=16.42
selected_family=result_capsule_family
selected_owner=result_capsule_ir_shape_diff_inventory
selected_reason=combined_result_capsule_pct_exceeds_remaining_unblocked_family_after_recent_no_effect_blocks
next_diagnostic=result_capsule_ir_shape_diff_inventory
rejected_owner=page_queue_immediate_retry
rejected_reason=recent_nonkeeper_requires_ir_shape_diff_before_retry
rejected_owner_1=page_model_immediate_retry
rejected_reason_1=receiver_forwarding_no_material_effect_requires_different_owner_hypothesis
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
selected_owner_family=result_capsule_ir_shape_diff_inventory
selected_reason=combined_result_capsule_exact_slot_weight_is_largest_after_blocking_recent_nonkeeper_and_recent_no_effect_page_model_retry
next_row=result_capsule_ir_shape_diff_inventory
optimization_open=0
```

The next row must inventory alloc/release result capsule exact-slot traffic and
escape shape before any capsule flattening, batching, or source shape change.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_weighted_exact_slot_owner_selection_after_receiver_forwarding_guard.sh
```
