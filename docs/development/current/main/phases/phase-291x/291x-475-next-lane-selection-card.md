---
Status: Landed
Date: 2026-04-27
Scope: Select next compiler-cleanliness lane after GenericMethodRoute root export prune
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-474-generic-method-route-root-export-closeout-card.md
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-475: Next Lane Selection

## Selected Lane

GenericMethodRoute component visibility prune.

## Why This Lane

The root `crate::mir` surface no longer re-exports component construction
records, but the owner module still exposes them as public API. That leaves a
wider-than-needed construction surface:

```text
GenericMethodRouteKind
GenericMethodRouteProof
GenericMethodRouteSurface
GenericMethodRouteSite
GenericMethodRouteEvidence
GenericMethodRouteOperands
GenericMethodRouteDecision
GenericMethodRoute::new(...)
```

The route record itself still needs to be readable by consumers through stable
accessors, but construction and enum internals should remain MIR-owned.

## Boundaries

- BoxShape-only.
- Do not change route detection or route metadata refresh.
- Do not change JSON field names or helper symbols.
- Do not touch `.inc` lowering behavior.
- Do not add hot lowering or MapGet specialization.

## Next

Inventory which public route accessors are real consumer contract and which
construction/enum APIs can become crate-private.
