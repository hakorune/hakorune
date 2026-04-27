---
Status: Landed
Date: 2026-04-27
Scope: Select next GenericMethodRoute cleanup lane
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-291x/291x-462-generic-method-route-site-operands-closeout-card.md
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-463: Next Lane Selection

## Goal

Select the next BoxShape-only compiler-cleanliness lane after
`GenericMethodRouteSite` / `GenericMethodRouteOperands`.

## Selected Lane

`GenericMethodRoute` now separates surface, site, evidence, operands, and
decision. However, route records are still assembled with public struct
literals in multiple places:

- route matcher functions in `src/mir/generic_method_route_plan.rs`
- route fixture construction in
  `src/runner/mir_json_emit/tests/generic_method_routes.rs`

That leaves the route record shape as an implicit multi-entry contract.

The next lane is:

```text
GenericMethodRoute constructor SSOT
```

## Constraints

- BoxShape-only.
- Do not change route matching.
- Do not change JSON field names or values.
- Do not change helper symbols, lowering tiers, or `.inc` behavior.
- Do not add CoreMethod rows or hot lowering.
- Keep route fields readable through accessors, not external struct literals.

## Next

Inventory all direct `GenericMethodRoute` struct-literal construction sites
before editing code.
