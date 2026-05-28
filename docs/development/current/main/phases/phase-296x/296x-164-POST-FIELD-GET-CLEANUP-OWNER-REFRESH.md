---
Status: Landed
Date: 2026-05-28
Scope: refresh the objectLifecycleSmallAlloc copy-owner surface after field_get cleanup.
Blocker: POST-FIELD-GET-CLEANUP-OWNER-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-163-POST-FIELD-GET-RESULT-CHAIN-CLEANUP-MEASUREMENT.md
  - tools/allocator/mir_post_field_get_cleanup_owner_refresh.py
---

# 296x-164 Post Field Get Cleanup Owner Refresh

## Purpose

Refresh the current `objectLifecycleSmallAlloc/1` MIR copy-owner surface after
row162/row163. This row does not open another optimization by itself; it selects
the next diagnostic owner from the current post-keeper MIR evidence.

## Required Output

```text
output_contract=hako-mimalloc-post-field-get-cleanup-owner-refresh-v0
input_contract=hako-mimalloc-post-field-get-result-chain-cleanup-measurement-v0
selected_owner
owner_confidence
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-post-field-get-cleanup-owner-refresh-v0
input_contract=hako-mimalloc-post-field-get-result-chain-cleanup-measurement-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
block_count=20
instruction_count=180
call_count=12
copy_count=88
phi_count=18
receiver_copy_count=27
arg_copy_count=7
result_copy_count=9
local_ssa_copy_count=38
call_adjacent_copy_count=40
phi_edge_copy_count=10
expression_materialization_copy_count=24
field_get_result_chain_copy_count=23
dominant_position=call_adjacent
dominant_expression_owner=field_get_result_chain
selected_owner=field_get_result_chain_follow_on_probe
owner_confidence=medium
owner_reason=field_get_result_chain_remains_dominant_expression_owner
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
position_call_adjacent_copy_count=40
position_expression_materialization_copy_count=24
position_branch_condition_copy_count=6
position_block_entry_copy_count=6
position_field_set_value_copy_count=2
position_local_ssa_copy_count=0
position_phi_edge_copy_count=10
expression_owner_field_get_result_chain_copy_count=23
expression_owner_phi_result_chain_copy_count=1
top_block_0_id=block_552
top_block_0_copy_count=13
top_block_1_id=block_570
top_block_1_copy_count=10
top_block_2_id=block_560
top_block_2_copy_count=8
top_block_3_id=block_575
top_block_3_copy_count=8
top_block_4_id=block_553
top_block_4_copy_count=7
top_block_5_id=block_555
top_block_5_copy_count=6
summary=ok
```

Interpretation:

```text
The first field_get result-chain cleanup was a structural keeper, but the
post-keeper surface still has 23 expression copies tied to field_get result
chains. The next row should probe whether these are consumer-side directness
copies around field_set/binop/compare, not reopen broad source rewrites.
```

## Next

```text
row165:
  field_get_result_chain_follow_on_probe

Goal:
  identify whether the remaining field_get result-chain copies are caused by
  consumer-side LocalSSA finalization, field_set value materialization, or
  compare/binop operand materialization before another compiler patch.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_post_field_get_cleanup_owner_refresh_guard.sh
```
