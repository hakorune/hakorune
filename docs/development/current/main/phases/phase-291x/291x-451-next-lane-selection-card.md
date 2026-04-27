---
Status: Landed
Date: 2026-04-27
Scope: next compiler-cleanliness lane selection after GenericMethodRoute surface split
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-291x/291x-450-generic-method-route-surface-closeout-card.md
  - src/mir/generic_method_route_plan.rs
  - src/mir/map_lookup_fusion_plan.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-451: Next Lane Selection

## Goal

Choose the next compiler-cleanliness lane after closing the
`GenericMethodRoute` surface/decision split.

This card is lane selection only. No behavior changed.

## Candidate Lanes

| Lane | Shape | Decision |
| --- | --- | --- |
| `GenericMethodRoute` decided metadata split | BoxShape | select next |
| `.inc` generated enum/table consumer migration | larger consumer lane | defer until MIR route metadata is better boxed |
| MapGet scalar lowering | proof/lowering lane | defer; no lowering change without owner evidence |
| Stage-B extra adapter thinning | BoxShape | defer; recently closed |
| JoinIR cleanup burst reopen | BoxShape | defer; needs a new seam inventory |

## Decision

Select **`GenericMethodRoute` decided metadata split** as the next lane.

Reason:

- `291x-449` made raw call-surface compatibility explicit with
  `GenericMethodRouteSurface`.
- The next visible flat field cluster is decided route/CoreMethod metadata:
  `route_kind`, `proof`, `core_method`, `return_shape`, `value_demand`, and
  `publication_policy`.
- These fields are what JSON emitters, `.inc` consumers, and
  `MapLookupFusion` should treat as the backend-facing route decision.
- Grouping them behind a named `GenericMethodRouteDecision` clarifies the
  boundary without changing JSON output, route matching, helper selection, or
  lowering tiers.

## Next Card

Create `291x-452-generic-method-route-decision-inventory` before code edits.

The inventory must classify:

- decided route metadata fields
- non-decision evidence fields that should stay outside the decision record
- direct readers that must move to accessors or the decision sub-record
- tests that pin unchanged JSON output

## Guards

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
