---
Status: Landed
Date: 2026-04-27
Scope: Inventory direct GenericMethodRoute construction before constructor SSOT split
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-463-next-lane-selection-card.md
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-464: GenericMethodRoute Constructor Inventory

## Goal

Inventory every direct `GenericMethodRoute` struct-literal construction site
before changing the route assembly boundary.

This is BoxShape-only. The implementation must not change route matching, JSON
field names, helper symbols, lowering tiers, or `.inc` behavior.

## Findings

Direct route record literals are limited to two files:

```text
src/mir/generic_method_route_plan.rs
src/runner/mir_json_emit/tests/generic_method_routes.rs
```

Inventory command:

```bash
rg -n "GenericMethodRoute \\{" \
  src/mir/generic_method_route_plan.rs \
  src/runner/mir_json_emit/tests/generic_method_routes.rs
```

Construction sites:

- `src/mir/generic_method_route_plan.rs`: matcher output sites for
  has/get/len/substring/indexOf/push/set plus one local unit-test fixture.
- `src/runner/mir_json_emit/tests/generic_method_routes.rs`: JSON fixture route
  rows.

External consumers already use accessors for route metadata:

- JSON emission reads through `route.block()`, `route.route_kind()`,
  `route.value_demand()`, and related accessors.
- MapLookup fusion reads route site/evidence/operand values through accessors.

## Planned Change

- Add `GenericMethodRoute::new(site, surface, evidence, operands, decision)`.
- Make `GenericMethodRoute` record fields private.
- Replace direct route literals with the constructor in matcher code and tests.
- Keep `GenericMethodRouteSurface`, `GenericMethodRouteSite`,
  `GenericMethodRouteEvidence`, `GenericMethodRouteOperands`, and
  `GenericMethodRouteDecision` unchanged.

## Acceptance

- `rg -n "GenericMethodRoute \\{" src/mir src/runner -g '*.rs'` shows no route
  struct-literal construction outside the struct definition / impl.
- Existing generic-method JSON output is unchanged.
- Existing map lookup fusion behavior is unchanged.
- `cargo check -q` and focused tests pass.
