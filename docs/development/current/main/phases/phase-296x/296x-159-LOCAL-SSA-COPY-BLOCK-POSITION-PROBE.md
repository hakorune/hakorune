---
Status: Landed
Date: 2026-05-28
Scope: classify local-like copy positions after selecting local SSA copy materialization.
Blocker: LOCAL-SSA-COPY-BLOCK-POSITION-PROBE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-158-CALLSITE-COPY-OWNER-SELECTION.md
  - tools/allocator/mir_local_ssa_copy_position_probe.py
---

# 296x-159 Local SSA Copy Block Position Probe

## Purpose

Make the selected `local_ssa_copy_materialization` owner visible enough for the
next decision. Row158 selected local SSA copy materialization with medium
confidence; this row classifies local-like copy positions so the next row can
choose a narrow optimization owner instead of guessing.

This row does not optimize.

## Required Output

```text
output_contract=hako-mimalloc-local-ssa-copy-position-probe-v0
input_contract=hako-mimalloc-callsite-copy-owner-selection-v0
dominant_local_like_position
local_like_copy_count
optimization_open=0
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-local-ssa-copy-position-probe-v0
input_contract=hako-mimalloc-callsite-copy-owner-selection-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
block_count=20
copy_count=98
local_like_copy_count=48
dominant_position=call_adjacent
dominant_local_like_position=expression_materialization
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
local_ssa_copy_count=3
expression_materialization_copy_count=29
field_set_value_copy_count=2
branch_condition_copy_count=6
return_block_copy_count=0
block_entry_copy_count=8
block_exit_copy_count=0
call_adjacent_copy_count=40
phi_edge_copy_count=10
top_block_0_id=block_552
top_block_0_copy_count=17
sample_0_category=expression_materialization
sample_0_block=block_552
sample_0_inst_index=4
summary=ok
```

Interpretation:

```text
The local-like copy pressure is not primarily return-block movement.
After removing call-adjacent and phi-edge copies, the dominant shape is
expression materialization, especially in block_552. The next decision should
inspect expression/value materialization around field_get/binop/field_set
chains before touching source-level helpers.
```

## Next

```text
row160:
  expression_materialization_owner_selection

Candidates:
  - field_get copy chain cleanup
  - binop operand materialization cleanup
  - field_set value materialization cleanup
  - block-entry receiver reuse
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_local_ssa_copy_block_position_probe_guard.sh
```
