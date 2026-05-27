---
Status: Landed
Date: 2026-05-27
Scope: join hako_check source perf-surface and MIR method shape evidence for one selected method.
Blocker: HAKO-SOURCE-MIR-SHAPE-JOIN-ADAPTER-296X-001
Related:
  - docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-85-HAKO-MIR-METHOD-SHAPE-PYTHON-ADAPTER.md
---

# 296x-86 Hako Source/MIR Shape Join Adapter

## Purpose

Join hako_check source perf-surface evidence with MIR method shape evidence for
one selected method.

## Required Output

```text
output_contract=hako-source-mir-shape-join-v0
source_contract=hako-check-perf-surface-v1
mir_contract=hako-mir-method-shape-v0
selected_method
source_risk_confirmed_in_mir=0|1
next_diagnostic
summary=ok
```

## Stop Line

Do not migrate MIR shape observation to `.hako` here.

## Landed Evidence

```text
output_contract=hako-source-mir-shape-join-v0
source_contract=hako-check-perf-surface-v1
mir_contract=hako-mir-method-shape-v0
selected_method=HakoAllocObjectLifecyclePageQueue.selectPage/0
source_risk_confirmed_in_mir=1
confirmed_risk_kind=array_access
next_diagnostic=keeper_candidate_from_confirmed_source_mir_array_access
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_source_mir_shape_join_adapter_guard.sh
```
