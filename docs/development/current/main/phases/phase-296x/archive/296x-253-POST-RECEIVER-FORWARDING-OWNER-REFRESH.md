---
Status: Landed
Date: 2026-05-29
Scope: refresh perf owner after receiver forwarding no-material-effect measurement.
Blocker: POST-RECEIVER-FORWARDING-OWNER-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-252-SELECTED-METHOD-RECEIVER-BLOCK-ENTRY-COPY-FORWARDING-MEASUREMENT.md
---

# 296x-253 Post Receiver Forwarding Owner Refresh

## Purpose

Refresh hot ownership after row252 found no material body-time improvement from
selected receiver forwarding.

This row does not implement another keeper. It reopens observation from perf
evidence and keeps the recent page-queue non-keeper blocked for immediate retry.

## Evidence

```text
output_contract=post-receiver-forwarding-owner-refresh-v0
input_contract=selected-method-receiver-block-entry-copy-forwarding-measurement-v0
workload_id=representative-object-lifecycle-small-block-v0
perf_exact_slot_helper_pct=56.72
perf_exact_slot_get_set_pct=52.64
perf_exact_slot_rmw_helper_pct=4.08
perf_legacy_field_helper_pct=0.00
perf_array_slot_backend_pct=15.91
perf_array_backend_hash_pct=15.97
perf_array_total_pct=31.88
perf_hako_method_pct=9.89
selected_boundary=weighted_exact_slot_callsite_attribution_refresh
next_diagnostic=weighted_exact_slot_owner_selection
static_candidate_count_only_rejected=1
weighted_hot_candidate_score_required=1
ir_shape_diff_required_before_next_keeper=1
sample_count_3_required_for_keeper_decision=1
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Weighted exact-slot attribution:

```text
output_contract=weighted-exact-slot-callsite-attribution-refresh-v0
input_contract=post-receiver-forwarding-owner-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
exact_slot_get_set_pct=52.64
attributed_callsite_count=30
dominant_family=page_queue_helpers
dominant_family_pct=12.11
recent_nonkeeper_family=page_queue_helpers
recent_nonkeeper_row=296x-241
recent_nonkeeper_family_pct=12.11
recent_nonkeeper_candidate_count=21
recent_nonkeeper_hot_per_candidate_pct=0.58
dominant_family_is_recent_nonkeeper=1
recent_nonkeeper_family_blocked_for_immediate_keeper=1
top_unblocked_family=page_model_hotpath
top_unblocked_family_pct=10.75
family_1_name=page_model_hotpath
family_1_pct=10.75
family_1_known_candidate_count=9
family_1_hot_per_candidate_pct=1.19
family_2_name=object_lifecycle_facade
family_2_pct=9.85
family_2_known_candidate_count=4
family_2_hot_per_candidate_pct=2.46
family_3_name=alloc_result_capsule
family_3_pct=8.30
family_4_name=release_result_capsule
family_4_pct=8.12
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
selected_reason=exact_slot_get_set_still_primary_and_dominant_page_queue_family_is_recent_nonkeeper
next_row=weighted_exact_slot_owner_selection
optimization_open=0
```

The next row must choose one owner from weighted evidence. It must not retry
page queue immediately, and it must not implement a keeper without a fresh
IR-shape or field-traffic plan.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_post_receiver_forwarding_owner_refresh_guard.sh
```
