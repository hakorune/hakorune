---
Status: Landed
Date: 2026-04-27
Scope: Select next GenericMethodRoute cleanup lane
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-291x/291x-470-generic-method-route-component-field-closeout-card.md
  - src/mir/mod.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-471: Next Lane Selection

## Goal

Select the next BoxShape-only compiler-cleanliness lane after
`GenericMethodRoute` component field privacy.

## Selected Lane

`GenericMethodRoute` and its component records are now constructor/accessor
based. The remaining broad surface is root `crate::mir` re-export of internal
component construction types:

- `GenericMethodRouteDecision`
- `GenericMethodRouteEvidence`
- `GenericMethodRouteOperands`
- `GenericMethodRouteSite`
- `GenericMethodRouteSurface`

These are useful for focused fixtures, but they do not need to be part of the
root MIR prelude.

The next lane is:

```text
GenericMethodRoute root re-export prune
```

## Constraints

- BoxShape-only.
- Do not change route matching.
- Do not change JSON field names or values.
- Do not change helper symbols, lowering tiers, or `.inc` behavior.
- Keep `GenericMethodRoute` itself available from root `crate::mir`.
- Keep fixture construction possible via the owner module.

## Next

Inventory root re-export consumers before editing code.
