# 293x-388 MIR-SEMANTIC-PLANS-001 Classification

Status: ready
Date: 2026-05-15

## Decision

Classify top-level MIR plan / route / seed surfaces without moving files. The
goal is to make `SemanticPlans` readable while preserving the distinction
between plans, backend-consumable routes, facts, contracts, and temporary seed
bridges.

## Scope

- Update `src/mir/README.md` or a narrow design doc with a classification table.
- Use the categories `LayoutPlans`, `PlacementPlans`, `LoweringRoutes`,
  `ExperimentalSeedRoutes`, and `SemanticFacts/Contracts`.
- Mark backend-active vs metadata-only when obvious.
- Do not physically move files.

## Stop Lines

- No code behavior changes.
- No file moves.
- No route activation.
- No metadata promotion beyond classification.

## Required Evidence

```text
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
