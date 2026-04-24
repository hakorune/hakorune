---
Status: Landed
Date: 2026-04-25
Scope: Clarify the RuntimeDataBox.has compatibility contract before any remaining has mirror pruning.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-209-runtime-data-map-has-metadata-fixture-card.md
  - docs/development/current/main/phases/phase-291x/291x-210-runtime-data-map-has-dispatch-fixture-card.md
  - apps/tests/mir_shape_guard/runtime_data_array_has_missing_min_v1.mir.json
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-211 RuntimeData Has Compat Contract Design Card

## Goal

Prevent another premature `has` mirror prune by making the remaining
compatibility boundary explicit.

Map-origin `RuntimeDataBox.has(i64_const)` is now covered by `MapHas`
CoreMethod metadata. Array-origin `RuntimeDataBox.has(index)` is not covered by
a CoreMethod contract.

## Decision

Keep Array-origin `RuntimeDataBox.has` on the `runtime_data_contains_any`
compat fallback for this cleanup pass.

Do not introduce `CoreMethodOp::ArrayHas` in this card. Adding `ArrayHas` is a
separate BoxCount task because it needs:

- manifest vocabulary
- MIR route planning contract
- `.inc` metadata consumer support
- boundary fixtures and smokes
- a clear semantic contract for index presence versus value containment

## Prune Rule

The `has` mirror rows may be pruned only after one of these happens:

1. `ArrayHas` lands as a CoreMethod contract and the Array-origin RuntimeData
   boundary fixtures carry that metadata.
2. Array-origin `RuntimeDataBox.has` is retired or replaced by an explicit
   non-generic contract that no longer needs method-name fallback.

Until then, keep:

- generic emit-kind `method has`
- MIR-call method-surface `method has`
- receiver-surface fallback rows needed by metadata-absent `has`

## Acceptance

```bash
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

The no-growth allowlist now names the remaining Array-origin RuntimeData.has
compat blocker directly. This keeps the next cleanup step from treating MapHas
coverage as sufficient for `has` mirror deletion.
