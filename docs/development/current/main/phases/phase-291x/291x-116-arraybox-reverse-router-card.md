---
Status: Landed
Date: 2026-04-24
Scope: Promote `ArrayBox.reverse()` into the stable Array surface catalog and the Unified value path as a mutating String-receipt row.
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

# ArrayBox Reverse Router Card

## Decision

Promote only the next deferred ArrayBox order row:

```text
ArrayBox.reverse()
  -> receiver-only Unified shape
  -> WriteHeap effect
  -> String receipt result
```

This card preserves the existing Rust runtime contract: `reverse()` mutates the
array in place and returns the `"ok"` receipt string. It does not widen into
`sort()`.

## Current Facts

- `ArrayBox.reverse()` runtime behavior already exists in
  `src/boxes/array/ops/sequence/order.rs`.
- `src/runtime/type_registry.rs` still carries `reverse` as an Array extra
  instead of a catalog row.
- router allowlists do not yet treat `reverse/0` as Unified.
- focused MIR/router coverage exists for receiver-only Array rows, but not for
  a mutating String-receipt Array row.

## Implementation Slice

- add `reverse` to `ArrayMethodId` / `ARRAY_SURFACE_METHODS`
- route `ArrayBox.reverse/0` through `ArrayBox::invoke_surface(...)`
- publish `MirType::String` for `ArrayBox.reverse/0`
- promote `reverse/0` to `Route::Unified` and keep invalid arities on BoxCall
- pin focused MIR and dispatch tests for receiver-only shape and String result
- extend the stable Array surface smoke minimally to exercise `reverse()`

## Non-Goals

- do not widen into `sort`
- do not change the `"ok"` receipt contract
- do not change ArrayBox ordering semantics beyond calling the existing runtime
  method
- do not change source-level `slice()` union-receiver follow-up behavior

## Acceptance

```bash
cargo test --lib array_surface_catalog --quiet
cargo test --lib invoke_surface_routes_insert_remove_clear_contains_indexof_join_reverse_and_length_alias --quiet
cargo test --lib test_array_slots_resolve_from_surface_catalog --quiet
cargo test --release array_value_reverse_uses_unified_receiver_shape_and_string_return -- --nocapture
bash tools/smokes/v2/profiles/integration/apps/phase290x_arraybox_surface_catalog_vm.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Exit Condition

`ArrayBox.reverse()` is catalog-backed, resolves through the Unified
receiver-only route, publishes `String`, and is smoke-pinned without widening
`sort()` or changing the receipt contract.
