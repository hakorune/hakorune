---
Status: Landed
Date: 2026-04-24
Scope: Promote `ArrayBox.join(delimiter)` into the stable Array surface catalog and the Unified value path as a read-only String-return row.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md
  - src/boxes/array/surface_catalog.rs
  - src/runtime/type_registry.rs
  - src/mir/builder/router/policy.rs
  - src/mir/builder/types/annotation.rs
  - src/tests/mir_corebox_router_unified.rs
  - tools/smokes/v2/profiles/integration/apps/phase290x_arraybox_surface_catalog_vm.sh
---

# ArrayBox Join Router Card

## Decision

Promote only the next deferred ArrayBox read row:

```text
ArrayBox.join(delimiter)
  -> receiver-plus-delimiter Unified shape
  -> read effect
  -> String result
```

This card follows the already landed receiver-plus-value Array rows and the
String-return publication precedent from StringBox read rows. It does not widen
into mutating Array order rows.

## Current Facts

- `ArrayBox.join(delimiter)` runtime behavior already exists in
  `src/boxes/array/ops/sequence/order.rs`.
- `src/runtime/type_registry.rs` still carries `join` as an Array extra instead
  of a catalog row.
- router allowlists do not yet treat `join/1` as Unified.
- focused MIR/router coverage exists for fixed Array return rows, but not for
  `join`.

## Implementation Slice

- add `join` to `ArrayMethodId` / `ARRAY_SURFACE_METHODS`
- route `ArrayBox.join/1` through `ArrayBox::invoke_surface(...)`
- publish `MirType::String` for `ArrayBox.join/1`
- promote `join/1` to `Route::Unified` and keep invalid arities on BoxCall
- pin focused MIR and dispatch tests for receiver-plus-delimiter shape and
  String result
- extend the stable Array surface smoke minimally to exercise `join()`

## Non-Goals

- do not widen into `sort` / `reverse`
- do not change `join()` delimiter type rules
- do not change ArrayBox stringification semantics
- do not change source-level `slice()` union-receiver follow-up behavior

## Acceptance

```bash
cargo test --lib array_surface_catalog --quiet
cargo test --lib invoke_surface_routes_insert_remove_clear_contains_indexof_join_and_length_alias --quiet
cargo test --lib test_array_slots_resolve_from_surface_catalog --quiet
cargo test --release array_value_join_uses_unified_receiver_arg_shape_and_string_return -- --nocapture
bash tools/smokes/v2/profiles/integration/apps/phase290x_arraybox_surface_catalog_vm.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Exit Condition

`ArrayBox.join(delimiter)` is catalog-backed, resolves through the Unified
receiver-plus-delimiter route, publishes `String`, and is smoke-pinned without
widening any mutating Array order row.
