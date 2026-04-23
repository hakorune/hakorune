---
Status: Landed
Date: 2026-04-24
Scope: Promote `ArrayBox.clear` into the stable Array surface catalog and the Unified value path as a receiver-only write-`Void` row.
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

# ArrayBox Clear Router Card

## Decision

Promote only the next deferred ArrayBox write row:

```text
ArrayBox.clear()
  -> receiver-only Unified shape
  -> write effect
  -> Void result
```

This card follows the already landed `ArrayBox.push` / `set` / `insert`
write-`Void` pattern. It does not widen into the other deferred ArrayBox rows.

## Current Facts

- `ArrayBox.clear()` runtime behavior already exists in `src/boxes/array/ops/clear.rs`
- `src/runtime/type_registry.rs` still carried `clear` as an Array extra instead
  of a catalog row
- router allowlists did not yet treat `clear/0` as Unified
- focused MIR/router coverage existed for `push`, `set`, and `insert`, but not
  for `clear`

## Implementation Slice

- add `clear` to `ArrayMethodId` / `ARRAY_SURFACE_METHODS`
- route `ArrayBox.clear/0` through `ArrayBox::invoke_surface(...)`
- publish `MirType::Void` for `ArrayBox.clear/0`
- promote `clear/0` to `Route::Unified` and keep invalid arities on BoxCall
- pin focused MIR and dispatch tests for receiver-only shape, duplicate-receiver
  stripping, and `Void` result
- extend the stable Array surface smoke minimally to exercise `clear()`

## Non-Goals

- do not widen into `contains` / `indexOf` / `join` / `sort` / `reverse`
- do not reopen MapBox or StringBox cleanup cards
- do not change the existing ArrayBox read-result publication rules
- do not change source-level `slice()` union-receiver follow-up behavior

## Acceptance

```bash
cargo test --lib array_surface_catalog --quiet
cargo test --lib invoke_surface_routes_insert_remove_clear_and_length_alias --quiet
cargo test --lib test_array_slots_resolve_from_surface_catalog --quiet
cargo test --release array_value_clear_uses_unified_receiver_arg_shape_and_void_return -- --nocapture
cargo test --release method_callee_arraybox_clear_strips_duplicate_receiver_arg -- --nocapture
bash tools/smokes/v2/profiles/integration/apps/phase290x_arraybox_surface_catalog_vm.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Exit Condition

`ArrayBox.clear()` is catalog-backed, resolves through the Unified receiver-only
route, publishes `Void`, and is smoke-pinned without widening any other deferred
ArrayBox row.
