---
Status: Landed
Date: 2026-04-27
Scope: Inventory GenericMethodRoute raw surface vs decided route metadata before code edits
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-447-next-lane-selection-card.md
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/root.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-448: GenericMethodRoute Surface Inventory

## Goal

Make the next code slice explicit before changing `GenericMethodRoute`.

This card is inventory only. No behavior changed.

## Current Shape

`GenericMethodRoute` currently stores three kinds of data in one flat struct:

| Field family | Current fields | Owner / role | Decision |
| --- | --- | --- | --- |
| site | `block`, `instruction_index` | MIR instruction location | keep flat |
| raw surface compatibility | `box_name`, `method`, `arity` | source/MIR call surface kept for JSON compatibility and debugging | move behind `GenericMethodRouteSurface` |
| operand values | `receiver_value`, `key_value`, `result_value` | MIR values needed by backend emit | keep flat |
| decided route metadata | `route_kind`, derived `route_id`, derived `emit_kind`, derived helper/effects | MIR route decision consumed by JSON / `.inc` | keep decided side separate from raw surface |
| proof / CoreMethod metadata | `proof`, `core_method`, `return_shape`, `value_demand`, `publication_policy` | MIR-owned proof and lowering contract metadata | keep flat for now; no behavior change |
| analysis metadata | `receiver_origin_box`, `key_route` | proof/route evidence | keep flat for now |

## Readers

Direct surface-field readers are:

- `src/runner/mir_json_emit/root.rs`
  - emits `box_name`, `method`, and `arity` as compatibility JSON fields
  - should continue emitting the same JSON shape
- `src/runner/mir_json_emit/tests/generic_method_routes.rs`
  - constructs test routes and asserts JSON surface fields
- `src/mir/generic_method_route_plan.rs` tests
  - assert raw surface fields on generated routes

No `.inc` reader should need the Rust struct layout. `.inc` consumes only the
emitted JSON fields.

## Decision

Introduce:

```rust
pub struct GenericMethodRouteSurface {
    pub box_name: String,
    pub method: String,
    pub arity: usize,
}
```

and replace the flat raw fields on `GenericMethodRoute` with:

```rust
pub surface: GenericMethodRouteSurface
```

Keep thin accessors on `GenericMethodRoute`:

```rust
box_name()
method()
arity()
```

This preserves JSON spellings while making call-surface compatibility visibly
different from decided route/core metadata.

## Non-Goals

- Do not change JSON output names.
- Do not change route matching, helper selection, `.inc` behavior, or lowering
  tiers.
- Do not add or remove CoreMethodContract rows.
- Do not add hot/warm lowering.
- Do not move proof fields yet.

## Next Card

Create `291x-449-generic-method-route-surface-record` and implement only the
surface sub-record split.

## Guards

```bash
cargo test -q generic_method_route
cargo test -q build_mir_json_root_emits_generic_method_routes
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
