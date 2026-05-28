---
Status: Landed
Date: 2026-05-29
Scope: select the next page-model shape owner after result capsule reset batching.
Blocker: PAGE-MODEL-HOTPATH-SHAPE-OWNER-SELECTION-AFTER-RESULT-CAPSULE-RESET-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-262-PAGE-MODEL-HOTPATH-IR-SHAPE-DIFF-REFRESH-AFTER-RESULT-CAPSULE-RESET.md
  - docs/development/current/main/phases/phase-296x/296x-252-SELECTED-METHOD-RECEIVER-BLOCK-ENTRY-COPY-FORWARDING-MEASUREMENT.md
---

# 296x-263 Page Model Hotpath Shape Owner Selection After Result Capsule Reset

## Purpose

Choose one next page-model diagnostic from the refreshed row262 IR shape.

This row does not implement a keeper. It deliberately avoids selecting the
same `HakoAllocPageModel.acquire_usize/1` copy-materialization retry that row252
already showed as structurally useful but not materially faster.

## Tooling Note

`tools/allocator/page_model_hotpath_shape_owner_selection.py` now treats
`selected_method_prior_no_material_effect_row` as a stop-line for repeating the
top method's copy-materialization owner. When an alternate page-model method is
available, it selects that method as the next diagnostic owner instead. The
older row247 guard remains green because reports without the stop-line keep the
original behavior.

## Evidence

```text
output_contract=page-model-hotpath-shape-owner-selection-v0
input_contract=page-model-hotpath-ir-shape-diff-inventory-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_method_pct=9.05
selected_method_shape_owner=copy_materialization
selected_method_copy_count=31
selected_method_field_op_count=21
selected_method_call_count=3
selected_owner=page_model_release_known_live_field_traffic_probe
selected_owner_method=HakoAllocPageModel.releaseLocalKnownLive/1
selected_reason=prior_acquire_copy_materialization_no_material_effect_select_next_page_model_method
next_diagnostic=page_model_release_known_live_field_traffic_probe
rejected_owner=page_model_same_block_rmw_retry
rejected_reason=recent_selected_method_rmw_keeper_already_applied
rejected_owner_1=page_model_direct_op_retry
rejected_reason_1=direct_op_previous_rejected
rejected_owner_2=page_queue_retry
rejected_reason_2=page_queue_recent_nonkeeper_retry_closed
selected_method_prior_no_material_effect_row=296x-252
selected_owner_method_pct=4.14
selected_owner_method_field_get_count=7
selected_owner_method_field_set_count=5
selected_owner_method_copy_count=13
selected_owner_method_call_count=2
rejected_owner_3=page_model_acquire_usize_copy_materialization_retry
rejected_reason_3=prior_receiver_forwarding_no_material_effect_requires_different_page_model_owner
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
selected_owner_family=page_model_release_known_live_field_traffic_probe
selected_reason=releaseLocalKnownLive_is_the_next_page_model_method_after_blocking_prior_no_material_acquire_usize_copy_retry
next_row=page_model_release_known_live_field_traffic_probe
optimization_open=0
```

The next row must inspect `HakoAllocPageModel.releaseLocalKnownLive/1` field and
copy traffic before any page-model implementation. It must not re-enter the
row252 receiver-forwarding path without new evidence.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_page_model_hotpath_shape_owner_selection_after_result_capsule_reset_guard.sh
```
