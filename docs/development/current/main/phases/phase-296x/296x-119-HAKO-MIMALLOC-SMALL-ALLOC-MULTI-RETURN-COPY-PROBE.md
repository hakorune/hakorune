---
Status: Current
Date: 2026-05-27
Scope: classify the remaining multi-return/copy shape after single-pred PHI elision.
Blocker: HAKO-MIMALLOC-SMALL-ALLOC-MULTI-RETURN-COPY-PROBE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-118-HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-IMPLEMENTATION.md
---

# 296x-119 Hako Mimalloc Small Alloc Multi-Return Copy Probe

## Purpose

Single-pred PHI elision removed the single-incoming PHI bloat
(`61 -> 0`) and reduced instruction count (`247 -> 191`), but copy count is
still high (`94 -> 99`) and the remaining candidate source is
`multi_return_join`.

Classify whether the next owner is return lowering, local SSA copy materialize,
or `.hako` control-shape cleanup.

## Required Output

```text
output_contract=hako-mimalloc-small-alloc-multi-return-copy-probe-v0
input_contract=hako-mimalloc-single-pred-phi-elision-implementation-v0
selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
return_count=6
copy_count=99
copy_from_phi_count=19
candidate_source=multi_return_join
next_action=return_lowering_probe|local_ssa_copy_probe|hako_shape_probe|stop_line
summary=ok
```

## Stop Line

Do not implement a second MIR builder change in this row.
