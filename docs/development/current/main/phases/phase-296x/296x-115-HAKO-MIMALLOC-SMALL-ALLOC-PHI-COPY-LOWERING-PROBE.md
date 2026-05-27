---
Status: Current
Date: 2026-05-27
Scope: classify why objectLifecycleSmallAlloc lowers to high phi/copy counts.
Blocker: HAKO-MIMALLOC-SMALL-ALLOC-PHI-COPY-LOWERING-PROBE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-114-HAKO-MIMALLOC-SMALL-ALLOC-MIR-SHAPE-DEEP-DIVE.md
---

# 296x-115 Hako Mimalloc Small Alloc Phi/Copy Lowering Probe

## Purpose

`objectLifecycleSmallAlloc` is still the top active owner, but row114 shows the
dominant lowered shape is `phi_copy`, not a simple `.hako` source-level keeper.

This row should classify the phi/copy source before any MIR builder or `.hako`
rewrite is selected.

## Required Output

```text
output_contract=hako-mimalloc-small-alloc-phi-copy-lowering-probe-v0
input_contract=hako-mimalloc-small-alloc-mir-shape-deep-dive-v0
selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
phi_count=76
copy_count=94
candidate_source=multi_return_join|branch_result_merge|local_copy_churn|unknown
next_action=mirbuilder_owner_probe|hako_shape_probe|measurement_refresh|stop_line
summary=ok
```

## Stop Line

Do not implement a MIR builder change or `.hako` keeper in this row. This is a
classification probe only.
