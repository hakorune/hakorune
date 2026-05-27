---
Status: Current
Date: 2026-05-27
Scope: inspect the lowered MIR shape for objectLifecycleSmallAlloc before selecting another keeper.
Blocker: HAKO-MIMALLOC-SMALL-ALLOC-MIR-SHAPE-DEEP-DIVE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-113-HAKO-MIMALLOC-POST-ROLLBACK-INLINE-SUCCESS-SOURCE-MIR-REFRESH.md
---

# 296x-114 Hako Mimalloc Small Alloc MIR Shape Deep Dive

## Purpose

`objectLifecycleSmallAlloc` remains the top active owner after two measured
non-keepers. Before another `.hako` keeper is selected, inspect the lowered
shape and classify the actual owner of the remaining cost.

This row is diagnostic-only.

## Required Output

```text
output_contract=hako-mimalloc-small-alloc-mir-shape-deep-dive-v0
input_contract=hako-mimalloc-post-rollback-inline-success-source-mir-refresh-v0
selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
mir_instruction_count
mir_call_count
mir_field_access_count
mir_phi_count
mir_copy_count
dominant_shape_owner=method_call|field_access|phi_copy|branching|unknown
next_action=keeper_selection|mir_lowering_probe|measurement_refresh|stop_line
summary=ok
```

## Stop Line

Do not implement a keeper in this row. Do not open provider activation,
replacement, hooks, globals, or winner claims.
