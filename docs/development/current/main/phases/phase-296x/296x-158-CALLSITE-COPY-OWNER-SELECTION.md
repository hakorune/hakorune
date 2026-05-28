---
Status: Landed
Date: 2026-05-28
Scope: select one next owner from callsite-copy attribution evidence.
Blocker: CALLSITE-COPY-OWNER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-156-OBJECT-LIFECYCLE-SMALL-HOTPATH-CALLSITE-COPY-ATTRIBUTION.md
  - docs/development/current/main/phases/phase-296x/296x-157-CALLSITE-COPY-ATTRIBUTION-DIFF-HARNESS.md
  - tools/allocator/mir_callsite_copy_owner_selection.py
---

# 296x-158 Callsite Copy Owner Selection

## Purpose

Choose one next owner from row156 attribution and row157 diff evidence before
opening another optimization row.

This row does not optimize.

## Required Output

```text
output_contract=hako-mimalloc-callsite-copy-owner-selection-v0
input_contract=hako-mimalloc-callsite-copy-attribution-v0
selected_owner
owner_confidence
owner_reason
next_diagnostic
optimization_open=0
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-callsite-copy-owner-selection-v0
input_contract=hako-mimalloc-callsite-copy-attribution-v0
diff_contract=hako-mimalloc-callsite-copy-attribution-diff-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
dominant_callee_family=page_hotpath_helpers
dominant_copy_owner=local_ssa_copy_materialization
diff_structural_effect=no_effect
selected_owner=local_ssa_copy_materialization
owner_confidence=medium
owner_reason=dominant_baseline_copy_owner
next_diagnostic=local_ssa_block_position_probe
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
local_ssa_copy_materialization_copy_count=48
receiver_materialization_copy_count=27
phi_edge_copy_materialization_copy_count=10
result_materialization_copy_count=9
arg_materialization_copy_count=7
page_hotpath_helpers_attributed_copy_count=23
top_callsite_callee=acquire_usize
top_callsite_family=page_hotpath_helpers
top_callsite_attributed_copy_count=9
summary=ok
```

Interpretation:

```text
The next row should not start with source expansion or helper inlining.
The selected owner is local SSA copy materialization, but confidence is only
medium because page-hotpath helpers still contribute a large attributed copy
surface. The next diagnostic should locate local-SSA copies by block/position
before selecting an optimization.
```

## Next

```text
row159:
  local_ssa_block_position_probe

Goal:
  determine whether local SSA copies are concentrated in return blocks,
  branch merge edges, call-adjacent movement, or general block entry/exit
  materialization.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_callsite_copy_owner_selection_guard.sh
```
