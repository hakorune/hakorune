---
Status: Landed
Date: 2026-04-27
Scope: Inventory GenericMethodRoute root re-export consumers
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-471-next-lane-selection-card.md
  - src/mir/mod.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-472: GenericMethodRoute Root Export Inventory

## Goal

Inventory consumers of root `crate::mir` re-exported GenericMethodRoute
component types before pruning the root surface.

This is BoxShape-only. The implementation must not change route matching, JSON
field names, helper symbols, lowering tiers, or `.inc` behavior.

## Findings

Root `crate::mir` currently re-exports both the route record and its component
construction records:

```text
GenericMethodRoute
GenericMethodRouteDecision
GenericMethodRouteEvidence
GenericMethodRouteKind
GenericMethodRouteOperands
GenericMethodRouteProof
GenericMethodRouteSite
GenericMethodRouteSurface
```

The component construction records are only used outside
`src/mir/generic_method_route_plan.rs` by
`src/runner/mir_json_emit/tests/generic_method_routes.rs`.

`GenericMethodRoute` itself is still a legitimate root export because
`MirFunction` metadata stores `Vec<GenericMethodRoute>` and route consumers read
route records through accessors.

## Planned Change

- Keep `GenericMethodRoute` and route refresh functions in root `crate::mir`.
- Remove component construction records and route metadata enums from root
  `crate::mir` re-export.
- Update the JSON fixture test to import component construction records from
  `crate::mir::generic_method_route_plan`.

## Acceptance

- `cargo check -q` passes.
- Generic method route JSON fixture still passes.
- `src/mir/mod.rs` root re-export surface no longer lists component construction
  records.
