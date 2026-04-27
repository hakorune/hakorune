---
Status: Landed
Date: 2026-04-27
Scope: Inventory GenericMethodRoute decided metadata before sub-record split
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-451-next-lane-selection-card.md
  - src/mir/generic_method_route_plan.rs
  - src/mir/map_lookup_fusion_plan.rs
  - src/runner/mir_json_emit/root.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-452: GenericMethodRoute Decision Inventory

## Goal

Inventory the decided metadata fields on `GenericMethodRoute` before splitting
them into a named sub-record.

This card is inventory only. No behavior changed.

## Field Classification

| Field family | Fields | Decision |
| --- | --- | --- |
| site | `block`, `instruction_index` | keep flat |
| surface compatibility | `surface` | already split by `291x-449` |
| evidence / analysis | `receiver_origin_box`, `key_route` | keep flat for now |
| operand values | `receiver_value`, `key_value`, `result_value` | keep flat |
| decided route metadata | `route_kind`, `proof`, `core_method`, `return_shape`, `value_demand`, `publication_policy` | move behind `GenericMethodRouteDecision` |

## Reader Inventory

Readers that must be updated:

- `src/runner/mir_json_emit/root.rs`
  - emits `route_kind`, `helper_symbol`, `proof`, `core_method`,
    `return_shape`, `value_demand`, `publication_policy`, and `effects`
  - must keep the same JSON field names and values
- `src/mir/map_lookup_fusion_plan.rs`
  - uses `core_method`, `return_shape`, `value_demand`, and
    `publication_policy` to select scalar MapGet / MapHas pairs
  - should read through accessors, not flat fields
- `src/mir/generic_method_route_plan.rs` tests
  - assert route metadata directly
  - should move to accessors so the struct layout is not the test contract
- `src/runner/mir_json_emit/tests/generic_method_routes.rs`
  - constructs explicit routes for JSON output tests
  - should construct a `GenericMethodRouteDecision`

## Decision

Introduce:

```rust
pub struct GenericMethodRouteDecision {
    pub route_kind: GenericMethodRouteKind,
    pub proof: GenericMethodRouteProof,
    pub core_method: Option<CoreMethodOpCarrier>,
    pub return_shape: Option<GenericMethodReturnShape>,
    pub value_demand: GenericMethodValueDemand,
    pub publication_policy: Option<GenericMethodPublicationPolicy>,
}
```

and replace the flat decided fields on `GenericMethodRoute` with:

```rust
pub decision: GenericMethodRouteDecision
```

Keep thin accessors on `GenericMethodRoute`:

```rust
route_kind()
proof()
core_method()
return_shape()
value_demand()
publication_policy()
```

This preserves behavior while making backend-facing decision metadata explicit.

## Non-Goals

- Do not change route matching or proof logic.
- Do not change JSON output names or values.
- Do not change `.inc` behavior.
- Do not change helper selection or lowering tiers.
- Do not add MapGet lowering.

## Next Card

Create `291x-453-generic-method-route-decision-record` and implement only the
decision sub-record split.

## Guards

```bash
cargo test -q generic_method_route
cargo test -q build_mir_json_root_emits_generic_method_routes
cargo test -q map_lookup_fusion
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
