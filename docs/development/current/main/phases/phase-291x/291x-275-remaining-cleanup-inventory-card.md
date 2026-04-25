---
Status: Landed
Date: 2026-04-26
Scope: Inventory the remaining phase-291x compiler-clean cleanup work after the has fallback closeout and docs simplification.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-255-post-birth-cleanup-task-order-card.md
  - docs/development/current/main/phases/phase-291x/291x-272-mir-call-maphas-surface-fallback-closeout-card.md
  - docs/development/current/main/phases/phase-291x/291x-274-docs-smoke-operating-simplification-card.md
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-275 Remaining Cleanup Inventory Card

## Goal

Close the open "next compiler-clean cleanup card selection" state by
classifying the remaining phase-291x cleanup surface into immediate, blocked,
and deferred buckets.

This is a docs-only inventory card. It does not edit `.inc`, MIR metadata,
fixtures, or smokes.

## Immediate

No further phase-291x `.inc` mirror prune is ready right now.

The `291x-255` post-birth task order has been exhausted by the landed
`291x-256` through `291x-273` cleanup/review series. The remaining
implementation-ready cleanup is therefore outside this closed task order and
needs a new owner-path change before reopening.

## Blocked

The no-growth guard intentionally has only the paired MIR-call `MapBox + has`
fallback baseline left:

```text
classifiers=2
rows=2
```

The rows are:

```text
classify_mir_call_receiver_surface box MapBox
classify_mir_call_method_surface method has
```

These are blocked by the metadata-absent direct `MapBox.has` boundary pinned
in `291x-272`. They may be revisited only after one of these changes lands:

- metadata-absent direct `MapBox.has` is retired
- the boundary gains an explicit non-surface contract
- a metadata-only producer contract proves that the fallback cannot be reached

## Deferred

Do not treat these as phase-291x mirror cleanup unless a new one-family card
reopens them with fresh evidence:

- MapBox future surface work such as `getField`, `setField`, `forEach`,
  `toJSON`, or compatibility exports
- performance work such as MapGet hot lowering or i64-key native storage
- Stage-B adapter thinning beyond the already-landed slices
- phase-137x app/perf work while it remains observe-only

## Supersedes

Several closed cards still contain historical `Next Work` sections pointing to
work that later landed or closed. Do not edit those ledger cards in place; this
inventory supersedes their stale next-work pointers:

```text
291x-248
291x-249..254
291x-256..261
291x-265..267
291x-269..271
```

`291x-272` remains the current fallback closeout authority for the final two
no-growth rows.

## Result

The next actionable compiler-clean step is not another phase-291x mirror prune.
The lane should either:

- move to a new owner-path change that retires metadata-absent direct
  `MapBox.has`, or
- select the next compiler-clean lane outside phase-291x.

## Acceptance

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
