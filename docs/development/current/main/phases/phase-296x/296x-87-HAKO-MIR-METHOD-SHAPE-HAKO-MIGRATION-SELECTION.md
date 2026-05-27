---
Status: Landed
Date: 2026-05-27
Scope: decide whether the Python MIR method shape adapter is stable enough for a minimal .hako migration.
Blocker: HAKO-MIR-METHOD-SHAPE-HAKO-MIGRATION-SELECTION-296X-001
Related:
  - docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-86-HAKO-SOURCE-MIR-SHAPE-JOIN-ADAPTER.md
---

# 296x-87 Hako MIR Method Shape .hako Migration Selection

## Purpose

Decide whether the Python MIR method shape adapter is stable enough to port a
minimal reader/checker to `.hako`.

## Required Output

```text
output_contract=hako-mir-method-shape-hako-migration-selection-v0
python_contract_stable=0|1
hako_migration_decision=accepted|parked
selected_scope
summary=ok
```

## Stop Line

Do not implement the `.hako` MIR reader in this selection row.

## Landed Evidence

```text
output_contract=hako-mir-method-shape-hako-migration-selection-v0
input_contract=hako-source-mir-shape-join-v0
python_contract_stable=0
hako_migration_decision=parked
park_reason=python_mir_shape_contract_needs_multi_method_use_before_hako_port
selected_scope=python_adapter_continues_multi_method_observation
next_row=HAKO-MIMALLOC-MULTI-METHOD-SOURCE-MIR-OBSERVATION-296X-001
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mir_method_shape_hako_migration_selection_guard.sh
```
