---
Status: Landed
Date: 2026-05-29
Scope: select the next page-model shape owner from IR-shape inventory.
Blocker: PAGE-MODEL-HOTPATH-SHAPE-OWNER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-246-PAGE-MODEL-HOTPATH-IR-SHAPE-DIFF-INVENTORY.md
---

# 296x-247 Page Model Hotpath Shape Owner Selection

## Purpose

Choose one next diagnostic from the page-model IR-shape inventory.

This row does not implement a keeper. It selects a narrow copy-materialization
probe for `HakoAllocPageModel.acquire_usize/1`, while rejecting immediate retry
of the recent RMW, direct-op, and page-queue paths.

## Evidence

```text
output_contract=page-model-hotpath-shape-owner-selection-v0
input_contract=page-model-hotpath-ir-shape-diff-inventory-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_method_pct=8.52
selected_method_shape_owner=copy_materialization
selected_method_copy_count=31
selected_method_field_op_count=21
selected_method_call_count=3
selected_owner=page_model_acquire_usize_copy_materialization_probe
selected_reason=selected_method_shape_owner_copy_materialization
next_diagnostic=page_model_acquire_usize_copy_materialization_probe
rejected_owner=page_model_same_block_rmw_retry
rejected_reason=recent_selected_method_rmw_keeper_already_applied
rejected_owner_1=page_model_direct_op_retry
rejected_reason_1=direct_op_previous_rejected
rejected_owner_2=page_queue_retry
rejected_reason_2=page_queue_recent_nonkeeper_retry_closed
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
selected_owner_family=page_model_acquire_usize_copy_materialization_probe
selected_reason=copy_materialization_is_current_shape_owner_after_rmw_and_direct_op_paths_are_closed
next_row=page_model_acquire_usize_copy_materialization_probe
optimization_open=0
```

The next row should attribute copies inside `acquire_usize/1` by origin before
any MIR-builder cleanup. This prevents another broad LocalSSA-style non-keeper.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_page_model_hotpath_shape_owner_selection_guard.sh
```
