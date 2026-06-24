---
Status: Landed
Date: 2026-05-29
Scope: inventory page-model hotpath IR shape after recordSuccess helper fusion and facade owner refresh.
Blocker: PAGE-MODEL-HOTPATH-IR-SHAPE-INVENTORY-AFTER-RECORD-SUCCESS-HELPER-FUSION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-288-POST-FACADE-INVENTORY-OWNER-REFRESH-AFTER-RECORD-SUCCESS-HELPER-FUSION.md
  - docs/development/current/main/phases/phase-296x/296x-262-PAGE-MODEL-HOTPATH-IR-SHAPE-DIFF-REFRESH-AFTER-RESULT-CAPSULE-RESET.md
---

# 296x-289 Page Model Hotpath IR Shape Inventory After RecordSuccess Helper Fusion

## Purpose

Inventory page-model hotpath IR shape after row288 selected `page_model_hotpath`
as the next unblocked exact-slot family.

This row does not implement a keeper. It refreshes the page-model method
ranking and MIR shape before choosing a page-model owner, because previous
page-model receiver forwarding had no material body-time effect.

## Evidence

```text
output_contract=page-model-hotpath-ir-shape-diff-inventory-v0
input_contract=post-facade-inventory-owner-refresh-after-record-success-helper-fusion-v0
workload_id=representative-object-lifecycle-small-block-v0
target_family=page_model_hotpath
target_family_pct=11.73
page_model_method_count=5
missing_page_model_method_count=0
page_model_exact_slot_perf_pct=11.73
page_model_mir_field_get_count=23
page_model_mir_field_set_count=13
page_model_mir_field_op_count=36
page_model_mir_copy_count=47
page_model_mir_call_count=5
page_model_mir_phi_count=1
selected_method=HakoAllocPageModel.acquire_usize/1
selected_method_pct=6.31
selected_method_field_get_count=13
selected_method_field_set_count=8
selected_method_field_op_count=21
selected_method_copy_count=31
selected_method_call_count=3
selected_method_phi_count=1
selected_method_shape_owner=copy_materialization
recent_selected_method_rmw_keeper_already_applied=1
direct_op_previous_rejected=1
page_queue_recent_nonkeeper_retry_closed=1
ir_shape_diff_inventory_only=1
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
method_0_symbol=HakoAllocPageModel.acquire_usize/1
method_0_pct=6.31
method_0_field_get_count=13
method_0_field_set_count=8
method_0_copy_count=31
method_0_call_count=3
method_1_symbol=HakoAllocPageModel.releaseLocalKnownLive/1
method_1_pct=2.89
method_1_field_get_count=7
method_1_field_set_count=5
method_1_copy_count=13
method_1_call_count=2
method_2_symbol=HakoAllocPageModel.isDecommitted/0
method_2_pct=1.26
method_2_field_get_count=1
method_2_field_set_count=0
method_2_copy_count=1
method_2_call_count=0
method_3_symbol=HakoAllocPageModel.freeCount/0
method_3_pct=0.64
method_3_field_get_count=1
method_3_field_set_count=0
method_3_copy_count=1
method_3_call_count=0
method_4_symbol=HakoAllocPageModel.isRetired/0
method_4_pct=0.63
method_4_field_get_count=1
method_4_field_set_count=0
method_4_copy_count=1
method_4_call_count=0
selected_next=page_model_hotpath_shape_owner_selection_after_record_success_helper_fusion
summary=ok
```

## Decision

```text
selected_owner_family=page_model_hotpath_shape_owner_selection_after_record_success_helper_fusion
selected_reason=acquire_usize_remains_top_page_model_method_but_copy_materialization_overlaps_prior_no_material_receiver_forwarding
next_row=page_model_hotpath_shape_owner_selection_after_record_success_helper_fusion
optimization_open=0
```

The next row must choose a page-model shape owner without immediately repeating
the row252 receiver-forwarding path. Valid choices include a narrower
`acquire_usize/1` copy owner with new evidence, `releaseLocalKnownLive/1`
field traffic, or owner refresh if no positive-net page-model shape remains.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_page_model_hotpath_ir_shape_inventory_after_record_success_helper_fusion_guard.sh
```
