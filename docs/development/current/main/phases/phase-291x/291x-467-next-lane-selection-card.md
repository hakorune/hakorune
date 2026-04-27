---
Status: Landed
Date: 2026-04-27
Scope: Select next GenericMethodRoute cleanup lane
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-291x/291x-466-generic-method-route-constructor-closeout-card.md
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-467: Next Lane Selection

## Goal

Select the next BoxShape-only compiler-cleanliness lane after
`GenericMethodRoute::new(...)`.

## Selected Lane

`GenericMethodRoute` itself now has a single assembly entry and private fields.
The component records are still publicly writable:

- `GenericMethodRouteSurface`
- `GenericMethodRouteSite`
- `GenericMethodRouteEvidence`
- `GenericMethodRouteOperands`
- `GenericMethodRouteDecision`

These records already have constructors. Keeping their fields public leaves
partial route state editable from outside the route-plan module.

The next lane is:

```text
GenericMethodRoute component field privacy
```

## Constraints

- BoxShape-only.
- Do not change route matching.
- Do not change JSON field names or values.
- Do not change helper symbols, lowering tiers, or `.inc` behavior.
- Do not add CoreMethod rows or hot lowering.
- Keep public construction through the existing component constructors.

## Next

Inventory external component-field access before editing code.
