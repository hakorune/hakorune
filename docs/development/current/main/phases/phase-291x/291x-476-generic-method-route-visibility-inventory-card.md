---
Status: Landed
Date: 2026-04-27
Scope: Inventory GenericMethodRoute component and enum visibility before pruning
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-475-next-lane-selection-card.md
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/root.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-476: GenericMethodRoute Visibility Inventory

## Goal

Separate the stable route-read contract from MIR-owned construction internals
before pruning visibility.

This is BoxShape-only. It must not change route refresh, JSON field names,
helper symbols, lowering tiers, or `.inc` behavior.

## Findings

External construction consumers outside `src/mir/generic_method_route_plan.rs`:

- `src/runner/mir_json_emit/tests/generic_method_routes.rs`

External route-kind/proof enum consumers outside the owner module:

- `src/runner/mir_json_emit/root.rs`
  - emits `route_kind`
  - emits `helper_symbol`
  - emits `proof`
- `src/runner/mir_json_emit/tests/generic_method_routes.rs`
  - fixture construction only

`src/mir/map_lookup_fusion_plan.rs` consumes the route record through stable
metadata accessors and does not need route-kind/proof enums.

## Keep Public

Keep `GenericMethodRoute` readable through stable accessors:

```text
box_name
method
route_id
emit_kind
arity
block
instruction_index
receiver_value
key_value
result_value
receiver_origin_box
key_route
effect_tags
core_method
return_shape
value_demand
publication_policy
```

Add primitive/string accessors for JSON emission so external code does not need
the route-kind/proof enums:

```text
route_kind_tag
helper_symbol
proof_tag
```

## Prune

Make construction internals crate-private:

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

Keep `route_kind()` and `proof()` as crate-private helpers for local tests and
internal route planners.

## Acceptance For Next Card

- JSON emitter uses `route_kind_tag()`, `helper_symbol()`, and `proof_tag()`.
- Component construction records become `pub(crate)`.
- Constructor APIs become `pub(crate)`.
- `cargo check -q` passes without private-interface warnings.
- Generic method route JSON fixture remains unchanged.
