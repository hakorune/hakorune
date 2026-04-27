---
Status: Landed
Date: 2026-04-27
Scope: Inventory GenericMethodRoute fixture construction ownership before isolation
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-480-next-lane-selection-card.md
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-481: GenericMethodRoute Fixture Construction Inventory

## Goal

Keep GenericMethodRoute construction details inside the MIR owner module, while
letting runner JSON tests keep asserting the public JSON shape.

This is BoxShape-only. It must not change route detection, emitted JSON fields,
helper symbols, proof tags, route order, `.inc` behavior, or lowering tiers.

## Findings

Owner module:

- `src/mir/generic_method_route_plan.rs`
  - owns `GenericMethodRoute::new(...)`
  - owns `GenericMethodRouteSurface`, `GenericMethodRouteSite`,
    `GenericMethodRouteEvidence`, `GenericMethodRouteOperands`, and
    `GenericMethodRouteDecision`
  - owns the route-kind/proof enums and internal route construction details

External manual route construction consumer:

- `src/runner/mir_json_emit/tests/generic_method_routes.rs`
  - imports all construction records directly
  - has a local `decision(...)` helper that mirrors owner construction shape
  - manually builds nine routes for JSON metadata assertions

External JSON emitter:

- `src/runner/mir_json_emit/root.rs`
  - already consumes stable accessors such as `route_kind_tag()`,
    `helper_symbol()`, and `proof_tag()`
  - does not need route-kind/proof enums or construction records

## Cleaner Boundary

Add owner-local test fixture builders:

```text
src/mir/generic_method_route_plan.rs
  #[cfg(test)] pub(crate) mod test_support
    -> returns ready-made GenericMethodRoute fixtures

src/runner/mir_json_emit/tests/generic_method_routes.rs
  -> imports test_support only
  -> asserts JSON output only
```

The runner test should not know how `GenericMethodRouteDecision` or the
component records are assembled.

## Acceptance For Next Card

- Add `generic_method_route_plan::test_support` behind `#[cfg(test)]`.
- Move the nine JSON fixture route builders behind owner-provided functions.
- Update `src/runner/mir_json_emit/tests/generic_method_routes.rs` to import
  only the fixture support module from `generic_method_route_plan`.
- Preserve route order and all JSON assertions.
- Keep codegen, route matching, `.inc`, helper symbols, and proof tags
  unchanged.
