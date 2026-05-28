---
Status: Landed
Date: 2026-05-29
Scope: attribute copy materialization origins inside `HakoAllocPageModel.acquire_usize/1`.
Blocker: PAGE-MODEL-ACQUIRE-USIZE-COPY-MATERIALIZATION-PROBE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-247-PAGE-MODEL-HOTPATH-SHAPE-OWNER-SELECTION.md
---

# 296x-248 Page Model Acquire Usize Copy Materialization Probe

## Purpose

Classify the `copy` instructions inside `HakoAllocPageModel.acquire_usize/1`
before designing another MIR-builder keeper.

This row keeps implementation closed. The goal is to avoid broad LocalSSA
cleanup and pick a narrow copy policy from the actual current MIR shape.

## Evidence

```text
output_contract=page-model-acquire-usize-copy-materialization-probe-v0
input_contract=page-model-hotpath-shape-owner-selection-v0
target_method=HakoAllocPageModel.acquire_usize/1
block_count=12
copy_count=31
dominant_copy_position=block_entry
block_entry_copy_count=13
block_entry_receiver_param_copy_count=9
block_entry_requested_size_param_copy_count=1
block_entry_derived_value_copy_count=3
call_adjacent_copy_count=7
expression_materialization_copy_count=5
expression_param_copy_count=1
branch_condition_copy_count=4
field_set_value_copy_count=2
local_ssa_copy_count=0
phi_edge_copy_count=0
recent_broad_local_ssa_nonkeeper_guard=1
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
top_block_0_id=block_45
top_block_0_copy_count=13
top_block_1_id=block_39
top_block_1_copy_count=5
top_block_2_id=block_42
top_block_2_copy_count=3
top_block_3_id=block_47
top_block_3_copy_count=3
selected_next=page_model_acquire_usize_block_entry_receiver_copy_policy_selection
summary=ok
```

## Decision

```text
selected_owner_family=page_model_acquire_usize_block_entry_receiver_copy_policy_selection
selected_reason=block_entry_receiver_param_copies_dominate_acquire_usize_copy_materialization
next_row=page_model_acquire_usize_block_entry_receiver_copy_policy_selection
optimization_open=0
```

The dominant surface is not a generic local-SSA pool. It is block-entry receiver
copy materialization, with 9 receiver-param copies at block starts. The next row
should decide whether there is a safe, narrow block-entry receiver copy policy.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_page_model_acquire_usize_copy_materialization_probe_guard.sh
```
