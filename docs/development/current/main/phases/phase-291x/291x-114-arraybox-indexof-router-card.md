---
Status: Landed
Date: 2026-04-24
Scope: Promote `ArrayBox.indexOf(value)` into the stable Array surface catalog and the Unified value path as a read-only Integer-return row.
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

# ArrayBox IndexOf Router Card

## Decision

Promote only the next deferred ArrayBox read row:

```text
ArrayBox.indexOf(value)
  -> receiver-plus-value Unified shape
  -> read effect
  -> Integer result
```

This card follows the already landed `ArrayBox.contains(value)` receiver shape
and the StringBox search-row fixed Integer publication precedent. It does not
widen into the other deferred ArrayBox rows.

## Current Facts

- `ArrayBox.indexOf(value)` runtime behavior already exists in
  `src/boxes/array/ops/sequence/membership.rs`.
- `src/runtime/type_registry.rs` still carries `indexOf` as an Array extra
  instead of a catalog row.
- router allowlists do not yet treat `indexOf/1` as Unified.
- focused MIR/router coverage exists for `contains`, but not for `indexOf`.

## Implementation Slice

- add `indexOf` to `ArrayMethodId` / `ARRAY_SURFACE_METHODS`
- route `ArrayBox.indexOf/1` through `ArrayBox::invoke_surface(...)`
- publish `MirType::Integer` for `ArrayBox.indexOf/1`
- promote `indexOf/1` to `Route::Unified` and keep invalid arities on BoxCall
- pin focused MIR and dispatch tests for receiver-plus-value shape and Integer
  result
- extend the stable Array surface smoke minimally to exercise `indexOf()`

## Non-Goals

- do not widen into `join` / `sort` / `reverse`
- do not add an `indexOf` alias or overload
- do not change ArrayBox membership equality semantics
- do not change source-level `slice()` union-receiver follow-up behavior

## Acceptance

```bash
cargo test --lib array_surface_catalog --quiet
cargo test --lib invoke_surface_routes_insert_remove_clear_contains_indexof_and_length_alias --quiet
cargo test --lib test_array_slots_resolve_from_surface_catalog --quiet
cargo test --release array_value_index_of_uses_unified_receiver_arg_shape_and_integer_return -- --nocapture
bash tools/smokes/v2/profiles/integration/apps/phase290x_arraybox_surface_catalog_vm.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Exit Condition

`ArrayBox.indexOf(value)` is catalog-backed, resolves through the Unified
receiver-plus-value route, publishes `Integer`, and is smoke-pinned without
widening any other deferred ArrayBox row.
