---
Status: Landed
Date: 2026-05-29
Scope: select page-model shape owner after recordSuccess helper fusion page-model inventory.
Blocker: PAGE-MODEL-HOTPATH-SHAPE-OWNER-SELECTION-AFTER-RECORD-SUCCESS-HELPER-FUSION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-289-PAGE-MODEL-HOTPATH-IR-SHAPE-INVENTORY-AFTER-RECORD-SUCCESS-HELPER-FUSION.md
  - docs/development/current/main/phases/phase-296x/296x-252-SELECTED-METHOD-RECEIVER-BLOCK-ENTRY-COPY-FORWARDING-MEASUREMENT.md
  - docs/development/current/main/phases/phase-296x/296x-268-PAGE-MODEL-RELEASE-KNOWN-LIVE-SINGLE-USE-RMW-MEASUREMENT.md
---

# 296x-290 Page Model Hotpath Shape Owner Selection After RecordSuccess Helper Fusion

## Purpose

Choose one next page-model diagnostic from the refreshed row289 IR shape.

This row does not implement a keeper. It deliberately avoids repeating both
known page-model non-keepers: `acquire_usize/1` receiver-copy forwarding from
row252 and `releaseLocalKnownLive/1` RMW from row268. Since both available
page-model subowners are already exercised, it selects owner refresh.

## Evidence

```text
output_contract=page-model-hotpath-shape-owner-selection-v0
input_contract=page-model-hotpath-ir-shape-diff-inventory-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_method_pct=6.31
selected_method_shape_owner=copy_materialization
selected_method_copy_count=31
selected_method_field_op_count=21
selected_method_call_count=3
selected_owner=post_page_model_hotpath_owner_refresh_after_record_success_helper_fusion
selected_owner_method=none
selected_reason=prior_acquire_copy_and_release_known_live_no_effect_select_owner_refresh
next_diagnostic=post_page_model_hotpath_owner_refresh_after_record_success_helper_fusion
rejected_owner=page_model_same_block_rmw_retry
rejected_reason=recent_selected_method_rmw_keeper_already_applied
rejected_owner_1=page_model_direct_op_retry
rejected_reason_1=direct_op_previous_rejected
rejected_owner_2=page_queue_retry
rejected_reason_2=page_queue_recent_nonkeeper_retry_closed
selected_method_prior_no_material_effect_row=296x-252
fallback_method=HakoAllocPageModel.releaseLocalKnownLive/1
fallback_method_prior_no_effect_row=296x-268
rejected_owner_3=page_model_acquire_usize_copy_materialization_retry
rejected_reason_3=prior_receiver_forwarding_no_material_effect_requires_different_page_model_owner
rejected_owner_4=page_model_release_known_live_field_traffic_probe
rejected_reason_4=prior_release_known_live_rmw_no_effect_requires_owner_refresh
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_owner_family=post_page_model_hotpath_owner_refresh_after_record_success_helper_fusion
selected_reason=both_known_page_model_subowners_have_recent_no_effect_evidence
next_row=post_page_model_hotpath_owner_refresh_after_record_success_helper_fusion
optimization_open=0
```

Do not re-enter the row252 receiver-forwarding path or row268 releaseKnownLive
RMW path without new evidence. The next row should refresh exact-slot ownership
after excluding page-model hotpath as already exercised.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_page_model_hotpath_shape_owner_selection_after_record_success_helper_fusion_guard.sh
```
