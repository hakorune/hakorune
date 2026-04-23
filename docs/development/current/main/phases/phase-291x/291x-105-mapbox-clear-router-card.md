---
Status: Landed
Date: 2026-04-24
Scope: Promote the catalog-backed `MapBox.clear` row to the Unified value path.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md
  - docs/development/current/main/phases/phase-291x/291x-99-mapbox-write-return-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-104-mapbox-delete-remove-router-card.md
---

# MapBox Clear Router Card

## Decision

Promote only the remaining mutating MapBox row:

```text
MapBox.clear()
```

The row is already catalog-backed:

```text
MapMethodId::Clear
canonical: clear
arity: 0
slot: 208
effect: WriteHeap
return: receipt String
```

This closes the current MapBox router-only backlog. It does not change the
source-level state-owner route or the receipt string contract.

## Preconditions

- `src/boxes/map_surface_catalog.rs` owns the `clear` row.
- `MapBox::invoke_surface(...)` dispatches `MapMethodId::Clear`.
- `src/mir/builder/types/annotation.rs` publishes `MirType::String` for
  `MapBox.clear/0`.
- source-level vm-hako `MapBox.clear()` state reset is pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_extended_clear_vm.sh`.
- source-level write-return receipt strings are pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_write_return_vm.sh`.

## Implementation Slice

- add `MapMethodId::Clear` to the router Unified allowlist
- pin `clear/0` route policy as `Route::Unified`
- pin invalid `clear` arities as BoxCall fallback
- add a MIR shape fixture proving receiver-only Unified args and receipt
  `String` result publication for `clear`
- add a dispatch regression proving duplicate-receiver stripping for
  `MapBox.clear` at the method callee boundary

## Non-Goals

- do not change `MapBox.clear()` state reset semantics
- do not change the write-return receipt contract
- do not touch `MapBox.get(existing-key)` element typing
- do not touch compat ABI rows or `.hako` source-owner routes
- do not reopen `delete` / `remove`

## Landing Snapshot

- `src/mir/builder/router/policy.rs` now allowlists `MapMethodId::Clear`.
- `src/tests/mir_corebox_router_unified.rs` pins `clear/0` as a Unified
  receiver-only call with receipt-string result publication.
- old `clear` BoxCall sentinels in neighboring route fixtures now assert the
  Unified receiver-only shape.
- `src/backend/mir_interpreter/handlers/calls/method/tests.rs` pins
  duplicate-receiver stripping for `clear` dispatch.
- `lang/src/runtime/collections/map_state_core_box.hako` now filters
  `keys()` / `values()` collection through live presence values, so `clear()`
  tombstoned presence keys are not republished.
- the MapBox router-only backlog is closed for all current catalog rows.

## Acceptance

```bash
cargo test -q router --lib
cargo test -q map_value_clear --lib
cargo test -q method_callee_mapbox_clear_strips_duplicate_receiver_arg --lib
cargo test -q corebox_surface_aliases_use_catalog_return_type --lib
bash tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_extended_clear_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_write_return_vm.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Exit Condition

`MapBox.clear()` uses the Unified receiver-only shape, publishes the landed
receipt-string result type, and the CoreBox router backlog has no remaining
MapBox row that is blocked only on routing.
