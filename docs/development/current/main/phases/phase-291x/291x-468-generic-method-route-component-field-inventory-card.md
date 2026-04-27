---
Status: Landed
Date: 2026-04-27
Scope: Inventory GenericMethodRoute component field access before privacy split
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-467-next-lane-selection-card.md
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-468: GenericMethodRoute Component Field Inventory

## Goal

Inventory external access to `GenericMethodRoute` component record fields before
making those fields private.

This is BoxShape-only. The implementation must not change route matching, JSON
field names, helper symbols, lowering tiers, or `.inc` behavior.

## Findings

The component records are constructed through existing constructors:

```bash
rg -n "GenericMethodRoute(Surface|Site|Evidence|Operands|Decision)::new" \
  src/mir src/runner -g '*.rs'
```

Direct component field reads are local to `src/mir/generic_method_route_plan.rs`
accessors. External consumers already read through `GenericMethodRoute`
accessors or construct fixtures through the component constructors.

This means the component fields can be made private without adding a new public
read surface.

## Planned Change

Make these component fields private:

- `GenericMethodRouteSurface::{box_name, method, arity}`
- `GenericMethodRouteSite::{block, instruction_index}`
- `GenericMethodRouteEvidence::{receiver_origin_box, key_route}`
- `GenericMethodRouteOperands::{receiver_value, key_value, result_value}`
- `GenericMethodRouteDecision::{route_kind, proof, core_method, return_shape,
  value_demand, publication_policy}`

## Acceptance

- Existing constructors remain public.
- Existing `GenericMethodRoute` accessors remain the public read surface.
- `cargo check -q` and focused tests pass.
