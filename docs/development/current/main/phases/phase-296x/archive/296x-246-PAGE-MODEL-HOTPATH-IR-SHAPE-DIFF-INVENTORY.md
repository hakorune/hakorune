---
Status: Landed
Date: 2026-05-29
Scope: inventory page-model hotpath exact-slot callsites against current MIR shape.
Blocker: PAGE-MODEL-HOTPATH-IR-SHAPE-DIFF-INVENTORY-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-245-WEIGHTED-EXACT-SLOT-OWNER-SELECTION.md
---

# 296x-246 Page Model Hotpath IR Shape Diff Inventory

## Purpose

Inspect page-model hotpath methods before another keeper implementation.

This row keeps optimization closed. It combines the current exact-slot perf
callgraph with fresh MIR JSON so the next row can select a shape owner from
actual hot methods, not static candidate count alone.

## Evidence

```text
output_contract=page-model-hotpath-ir-shape-diff-inventory-v0
input_contract=weighted-exact-slot-owner-selection-v0
workload_id=representative-object-lifecycle-small-block-v0
target_family=page_model_hotpath
target_family_pct=15.29
page_model_method_count=5
missing_page_model_method_count=0
page_model_exact_slot_perf_pct=15.29
page_model_mir_field_get_count=26
page_model_mir_field_set_count=32
page_model_mir_field_op_count=58
page_model_mir_copy_count=62
page_model_mir_call_count=8
page_model_mir_phi_count=2
selected_method=HakoAllocPageModel.acquire_usize/1
selected_method_pct=8.52
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
method_0_pct=8.52
method_0_field_get_count=13
method_0_field_set_count=8
method_0_copy_count=31
method_0_call_count=3
method_1_symbol=HakoAllocPageModel.isRetired/0
method_1_pct=3.39
method_1_field_get_count=1
method_1_field_set_count=0
method_1_copy_count=1
method_1_call_count=0
method_2_symbol=HakoAllocPageModel.releaseLocalKnownLive/1
method_2_pct=1.70
method_2_field_get_count=7
method_2_field_set_count=5
method_2_copy_count=13
method_2_call_count=2
method_3_symbol=HakoAllocPageModel.resetToFresh/0
method_3_pct=0.85
method_3_field_get_count=4
method_3_field_set_count=19
method_3_copy_count=16
method_3_call_count=3
method_4_symbol=HakoAllocPageModel.isDecommitted/0
method_4_pct=0.83
method_4_field_get_count=1
method_4_field_set_count=0
method_4_copy_count=1
method_4_call_count=0
selected_next=page_model_hotpath_shape_owner_selection
summary=ok
```

## Decision

```text
selected_owner_family=page_model_hotpath_shape_owner_selection
selected_reason=top_page_model_method_is_acquire_usize_but_shape_owner_is_copy_materialization_after_rmw_and_direct_op_attempts
next_row=page_model_hotpath_shape_owner_selection
optimization_open=0
```

The selected method remains `HakoAllocPageModel.acquire_usize/1`, but the
current shape owner is not a fresh same-block RMW surface. The method already
has the RMW keeper, direct-op was previously rejected, and page queue retry is
closed. The next row must choose between copy-materialization cleanup, a more
narrow field operation shape, or owner refresh.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_page_model_hotpath_ir_shape_diff_inventory_guard.sh
```
