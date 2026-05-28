---
Status: Landed
Date: 2026-05-28
Scope: select the next MIR body owner after the Hako/C body timing gap taxonomy.
Blocker: OBJECT-LIFECYCLE-MIR-BODY-OWNER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-175-OBJECT-LIFECYCLE-BODY-TIMING-GAP-TAXONOMY.md
  - tools/allocator/hako_mimalloc_object_lifecycle_mir_body_owner_selection.py
  - tools/allocator/mir_callsite_copy_attribution.py
---

# 296x-176 Object Lifecycle MIR Body Owner Selection

## Purpose

Select one MIR body owner from the large Hako/C body timing gap and current
`objectLifecycleSmallAlloc/1` callsite-copy attribution. This row does not
optimize; it chooses the next diagnostic and explicitly avoids retrying the
recent `local_ssa_same_block_field_get_reuse` non-keeper.

## Required Output

```text
output_contract=hako-mimalloc-object-lifecycle-mir-body-owner-selection-v0
body_gap_owner=compiler_lowering
selected_mir_body_owner=local_ssa_copy_materialization
secondary_mir_body_owner=...
rejected_recent_nonkeeper=local_ssa_same_block_field_get_reuse
next_diagnostic=local_ssa_dynamic_weight_probe
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Interpretation

```text
The structural owner is still local SSA copy materialization, but the previous
same-block reuse implementation was a performance non-keeper. The next row must
measure dynamic weight or emitted body impact before changing lowering again.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_object_lifecycle_mir_body_owner_selection_guard.sh
```
