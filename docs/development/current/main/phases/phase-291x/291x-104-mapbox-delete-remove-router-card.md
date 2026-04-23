---
Status: Landed
Date: 2026-04-23
Scope: Promote the catalog-backed `MapBox.delete` / `remove` row to the Unified value path.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md
  - docs/development/current/main/phases/phase-291x/291x-99-mapbox-write-return-contract-card.md
---

# MapBox Delete/Remove Router Card

## Decision

Promote only the landed mutating delete row:

```text
MapBox.delete(key)
MapBox.remove(key) alias
```

Both names resolve through the same catalog row:

```text
MapMethodId::Delete
canonical: delete
alias: remove
arity: 1
slot: 205
effect: WriteHeap
return: receipt String
```

`MapBox.clear()` stays on the BoxCall fallback in this card. It has the same
write-return contract family, but it is a separate arity-zero mutating row and
needs its own route fixture before promotion.

## Preconditions

- `src/boxes/map_surface_catalog.rs` owns the delete/remove row.
- `MapBox::invoke_surface(...)` dispatches the row by slot.
- `src/mir/builder/types/annotation.rs` publishes `MirType::String` for
  `MapBox.delete/1` and `MapBox.remove/1`.
- source-level vm-hako `MapBox.remove(key)` parity is already pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_extended_remove_vm.sh`.
- source-level write-return receipt strings are already pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_write_return_vm.sh`.

## Implementation Slice

- add `MapMethodId::Delete` to the router Unified allowlist
- pin `delete` and `remove` route policy as `Route::Unified`
- pin invalid delete/remove arities and `clear/0` as BoxCall fallback
- add a MIR shape fixture proving receiver-plus-key Unified args and receipt
  `String` result publication for both names
- replace old `delete` fallback sentinels in neighboring fixtures with
  `clear` fallback sentinels

## Non-Goals

- do not promote `MapBox.clear()`
- do not change the write-return receipt contract
- do not change `MapBox.get(existing-key)` element typing
- do not normalize `delete` / `remove` beyond the existing catalog alias row
- do not touch compat ABI rows or `.hako` source-owner routes

## Landing Snapshot

- `src/mir/builder/router/policy.rs` now allowlists
  `MapMethodId::Delete`, which covers canonical `delete` and alias `remove`.
- `src/tests/mir_corebox_router_unified.rs` pins both names as Unified
  receiver-plus-key calls and receipt-string result publishers.
- old `delete` fallback sentinels in neighboring route fixtures were replaced
  with `clear` fallback sentinels.
- `src/backend/mir_interpreter/handlers/calls/method/tests.rs` pins
  duplicate-receiver stripping for `delete` and `remove` dispatch.
- `MapBox.clear()` remains the only mutating MapBox row still on the
  route-only fallback backlog.

## Acceptance

```bash
cargo test -q router --lib
cargo test -q map_value_delete --lib
cargo test -q method_callee_mapbox_delete_remove_strips_duplicate_receiver_arg --lib
cargo test -q corebox_surface_aliases_use_catalog_return_type --lib
bash tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_extended_remove_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_write_return_vm.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Exit Condition

`MapBox.delete(key)` and `MapBox.remove(key)` use the Unified receiver-plus-key
shape, publish the landed receipt-string result type, and leave `MapBox.clear()`
as the next separate route-only card.
