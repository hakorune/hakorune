---
Status: Landed
Date: 2026-05-29
Scope: freeze selected-method receiver block-entry copy forwarding guard surface.
Blocker: SELECTED-METHOD-RECEIVER-BLOCK-ENTRY-COPY-FORWARDING-GUARD-SURFACE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-249-PAGE-MODEL-ACQUIRE-USIZE-BLOCK-ENTRY-RECEIVER-COPY-POLICY-SELECTION.md
---

# 296x-250 Selected Method Receiver Block-Entry Copy Forwarding Guard Surface

## Purpose

Freeze the implementation surface before changing MIR builder behavior.

The selected surface is intentionally narrow:

- method: `HakoAllocPageModel.acquire_usize/1`
- copy shape: block-entry `copy src=0`
- sink: same-block receiver use by `field_get` or `field_set`
- excluded: call-adjacent receiver copies, non-receiver-param copies,
  cross-block rewrites, broad LocalSSA reuse

## Evidence

```text
output_contract=selected-method-receiver-block-entry-copy-forwarding-guard-surface-v0
input_contract=page-model-acquire-usize-block-entry-receiver-copy-policy-selection-v0
target_method=HakoAllocPageModel.acquire_usize/1
candidate_count=9
field_get_receiver_candidate_count=8
field_set_receiver_candidate_count=1
receiver_source_value=0
candidate_position=block_entry
candidate_scope=selected_method_only
exclude_call_adjacent_receiver_copy=1
exclude_non_receiver_param_copy=1
exclude_cross_block_rewrite=1
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
candidate_0_block=block_34
candidate_0_inst_index=0
candidate_0_dst=2
candidate_0_sink=field_get_receiver
candidate_1_block=block_35
candidate_1_inst_index=0
candidate_1_dst=12
candidate_1_sink=field_get_receiver
candidate_2_block=block_36
candidate_2_inst_index=0
candidate_2_dst=24
candidate_2_sink=field_get_receiver
candidate_3_block=block_38
candidate_3_inst_index=0
candidate_3_dst=34
candidate_3_sink=field_get_receiver
candidate_4_block=block_39
candidate_4_inst_index=0
candidate_4_dst=46
candidate_4_sink=field_get_receiver
candidate_5_block=block_41
candidate_5_inst_index=0
candidate_5_dst=59
candidate_5_sink=field_get_receiver
candidate_6_block=block_42
candidate_6_inst_index=0
candidate_6_dst=71
candidate_6_sink=field_get_receiver
candidate_7_block=block_45
candidate_7_inst_index=2
candidate_7_dst=96
candidate_7_sink=field_get_receiver
candidate_8_block=block_47
candidate_8_inst_index=0
candidate_8_dst=149
candidate_8_sink=field_set_receiver
selected_next=selected_method_receiver_block_entry_copy_forwarding_implementation
summary=ok
```

## Decision

```text
selected_owner_family=selected_method_receiver_block_entry_copy_forwarding_implementation
selected_reason=guard_surface_has_9_selected_method_receiver_copy_candidates
next_row=selected_method_receiver_block_entry_copy_forwarding_implementation
optimization_open=0
```

The next row may implement this selected-method-only forwarding rule. It must
preserve semantic proof and measure before keeper acceptance.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_selected_method_receiver_block_entry_copy_forwarding_guard_surface_guard.sh
```
