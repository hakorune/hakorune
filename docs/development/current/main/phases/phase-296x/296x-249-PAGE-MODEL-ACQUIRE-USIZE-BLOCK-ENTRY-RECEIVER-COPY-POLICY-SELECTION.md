---
Status: Landed
Date: 2026-05-29
Scope: select a narrow block-entry receiver copy policy for `acquire_usize/1`.
Blocker: PAGE-MODEL-ACQUIRE-USIZE-BLOCK-ENTRY-RECEIVER-COPY-POLICY-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-248-PAGE-MODEL-ACQUIRE-USIZE-COPY-MATERIALIZATION-PROBE.md
---

# 296x-249 Page Model Acquire Usize Block-Entry Receiver Copy Policy Selection

## Purpose

Select a narrow policy for `HakoAllocPageModel.acquire_usize/1` receiver copies.

The probe shows a well-isolated surface: block-entry copies dominate and 9 of
them copy the receiver param. This row still keeps implementation closed. It
only selects a guard-surface row, with broad LocalSSA reuse and cross-block
rewrites explicitly rejected.

## Evidence

```text
output_contract=page-model-acquire-usize-block-entry-receiver-copy-policy-selection-v0
input_contract=page-model-acquire-usize-copy-materialization-probe-v0
target_method=HakoAllocPageModel.acquire_usize/1
copy_count=31
block_entry_copy_count=13
block_entry_receiver_param_copy_count=9
local_ssa_copy_count=0
phi_edge_copy_count=0
selected_policy=selected_method_receiver_block_entry_copy_forwarding_guard_surface
selected_reason=receiver_block_entry_copies_dominate_without_local_ssa_or_phi_edge_surface
next_row=selected_method_receiver_block_entry_copy_forwarding_guard_surface
policy_scope=selected_method_only
policy_shape=receiver_param_block_entry_copy_forwarding
broad_local_ssa_reuse=0
cross_block_value_rewrite=0
field_get_result_chain_rewrite=0
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
selected_owner_family=selected_method_receiver_block_entry_copy_forwarding_guard_surface
selected_reason=receiver_param_block_entry_copies_are_isolated_in_acquire_usize
next_row=selected_method_receiver_block_entry_copy_forwarding_guard_surface
optimization_open=0
```

The next row must freeze an implementation guard surface before touching the
MIR builder. The intended scope is selected-method receiver-param block-entry
copy forwarding only.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_page_model_acquire_usize_block_entry_receiver_copy_policy_selection_guard.sh
```
