---
Status: Landed
Date: 2026-04-24
Scope: Promote `ArrayBox.contains(value)` into the stable Array surface catalog and the Unified value path as a read-only Bool-return row.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - src/boxes/array/surface_catalog.rs
  - src/runtime/type_registry.rs
  - src/mir/builder/router/policy.rs
  - src/mir/builder/types/annotation.rs
  - src/tests/mir_corebox_router_unified.rs
  - tools/smokes/v2/profiles/integration/apps/phase290x_arraybox_surface_catalog_vm.sh
---

# ArrayBox Contains Router Card

## Decision

Promote only the next deferred ArrayBox read row:

```text
ArrayBox.contains(value)
  -> receiver-plus-value Unified shape
  -> read effect
  -> Bool result
```

This card follows the already landed `StringBox.contains` Bool-return pattern
and the ArrayBox stable catalog shape. It does not widen into the other
deferred ArrayBox rows.

## Current Facts

- `ArrayBox.contains(value)` runtime behavior already exists in
  `src/boxes/array/ops/sequence/membership.rs`.
- `src/runtime/type_registry.rs` still carries `contains` as an Array extra
  instead of a catalog row.
- router allowlists do not yet treat `contains/1` as Unified.
- focused MIR/router coverage exists for the current stable Array rows, but not
  for `contains`.

## Implementation Slice

- add `contains` to `ArrayMethodId` / `ARRAY_SURFACE_METHODS`
- route `ArrayBox.contains/1` through `ArrayBox::invoke_surface(...)`
- publish `MirType::Bool` for `ArrayBox.contains/1`
- promote `contains/1` to `Route::Unified` and keep invalid arities on BoxCall
- pin focused MIR and dispatch tests for receiver-plus-value shape and Bool
  result
- extend the stable Array surface smoke minimally to exercise `contains()`

## Non-Goals

- do not widen into `indexOf` / `join` / `sort` / `reverse`
- do not change ArrayBox element-result publication (`get` / `pop` / `remove`)
- do not change source-level `slice()` union-receiver follow-up behavior
- do not reopen MapBox or StringBox cleanup cards

## Acceptance

```bash
cargo test --lib array_surface_catalog --quiet
cargo test --lib invoke_surface_routes_insert_remove_clear_contains_and_length_alias --quiet
cargo test --lib test_array_slots_resolve_from_surface_catalog --quiet
cargo test --release array_value_contains_uses_unified_receiver_arg_shape_and_bool_return -- --nocapture
bash tools/smokes/v2/profiles/integration/apps/phase290x_arraybox_surface_catalog_vm.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Exit Condition

`ArrayBox.contains(value)` is catalog-backed, resolves through the Unified
receiver-plus-value route, publishes `Bool`, and is smoke-pinned without
widening any other deferred ArrayBox row.
