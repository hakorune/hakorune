---
Status: Current
Date: 2026-05-27
Scope: apply source/MIR observation to multiple object-lifecycle methods and select the next keeper candidate.
Blocker: HAKO-MIMALLOC-MULTI-METHOD-SOURCE-MIR-OBSERVATION-296X-001
Related:
  - docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-87-HAKO-MIR-METHOD-SHAPE-HAKO-MIGRATION-SELECTION.md
---

# 296x-88 Hako Mimalloc Multi-Method Source/MIR Observation

## Purpose

Use the Python source/MIR observation stack across multiple object-lifecycle
methods before selecting the next keeper. Keep `.hako` MIR migration parked.

Candidate methods:

```text
HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
HakoAllocObjectLifecyclePageQueue.selectPage/0
```

## Required Output

```text
output_contract=hako-mimalloc-multi-method-source-mir-observation-v0
input_contract=hako-mir-method-shape-hako-migration-selection-v0
method_count
confirmed_source_mir_risk_count
selected_method
selected_risk_kind
next_keeper
summary=ok
```

## Stop Line

Do not implement the keeper in this row. Do not migrate MIR observation to
`.hako`.
