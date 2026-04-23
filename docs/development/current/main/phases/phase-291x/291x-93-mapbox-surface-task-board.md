---
Status: Landed
Date: 2026-04-24
Scope: phase-291x second implementation target, `MapBox` surface catalog first slice.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/development/current/main/phases/phase-291x/291x-120-mapbox-taskboard-closeout-card.md
---

# MapBox Surface Task Board

## North Star

```text
MapBox surface catalog
  -> one method id table for current vtable rows
  -> one Rust invoke seam
  -> thin consumers
  -> one stable smoke
```

## Rules

- code slice is MapBox-only
- catalog current Rust vtable rows first
- do not add new aliases in this card
- do not collapse `size` slot `200` and `len` slot `201`
- do not change `set` / `delete` / `clear` return values
- do not touch compat ABI exports except to document their current separation
- keep `.hako` `MapCoreBox` as the visible owner for state/raw-handle orchestration

## Implementation Slice Order

| Card | Status | Goal |
| --- | --- | --- |
| `291x-M1a` | done | add `src/boxes/map_surface_catalog.rs` and `MapMethodId` |
| `291x-M1b` | done | add `MapBox::invoke_surface(...)` for current vtable rows |
| `291x-M1c` | done | convert TypeRegistry / method resolution / effect analysis readers |
| `291x-M1d` | done | route Rust VM slot dispatch through the cataloged surface rows |
| `291x-M1e` | done | add stable MapBox surface smoke |

## First Stable Surface Target

| Method | Arity | Slot | Effect | Return | Notes |
| --- | ---: | ---: | --- | --- | --- |
| `size` | 0 | 200 | Read | Value | keep legacy slot |
| `len` | 0 | 201 | Read | Value | keep legacy slot |
| `has` | 1 | 202 | Read | Value | boolean value |
| `get` | 1 | 203 | Read | Value | missing-key behavior unchanged |
| `set` | 2 | 204 | WriteHeap | Value | current receipt value unchanged |
| `delete` / `remove` | 1 | 205 | WriteHeap | Value | preserve existing TypeRegistry alias |
| `keys` | 0 | 206 | Read | Value | array value |
| `values` | 0 | 207 | Read | Value | array value |
| `clear` | 0 | 208 | WriteHeap | Value | current receipt value unchanged |

## Completed Follow-Ups

- `length` alias is landed in
  `docs/development/current/main/phases/phase-291x/291x-97-mapbox-length-alias-card.md`.
- write-return receipt rows are landed in
  `docs/development/current/main/phases/phase-291x/291x-99-mapbox-write-return-contract-card.md`.
- bad-key and missing-key contracts are landed in
  `docs/development/current/main/phases/phase-291x/291x-100-mapbox-bad-key-contract-card.md`
  and
  `docs/development/current/main/phases/phase-291x/291x-101-mapbox-get-missing-key-contract-card.md`.
- keys/values element publication is landed in
  `docs/development/current/main/phases/phase-291x/291x-102-mapbox-keys-values-element-publication-card.md`.
- source-route and compat/source cleanup are landed in
  `docs/development/current/main/phases/phase-291x/291x-95-mapbox-hako-extended-route-cleanup-card.md`
  and
  `docs/development/current/main/phases/phase-291x/291x-109-map-compat-source-cleanup-card.md`.
- router promotion for `delete` / `remove` and `clear` is landed in
  `docs/development/current/main/phases/phase-291x/291x-104-mapbox-delete-remove-router-card.md`
  and
  `docs/development/current/main/phases/phase-291x/291x-105-mapbox-clear-router-card.md`.

## Explicitly Deferred Future Rows

- `size` / `len` slot unification
- `getField` / `setField` policy
- `birth`
- `forEach`
- `toJSON`
- compat-only exports in `crates/nyash_kernel/src/plugin/map_compat.rs`
- bad-key validation unification across all runtime lanes

## Exit Condition

This MapBox slice is done when:

1. catalog is the clear Rust-side surface authoring point
2. TypeRegistry rows come from the catalog instead of a separate Map extras table
3. method resolution and effect analysis read `MapMethodId`
4. Rust VM slot dispatch delegates current MapBox rows to `MapBox::invoke_surface(...)`
5. one stable smoke proves Rust catalog rows plus the hako-visible VM subset without stub drift

## Landing Snapshot

- Rust catalog: `src/boxes/map_surface_catalog.rs`
- Rust invoke seam: `MapBox::invoke_surface(...)`
- Rust consumers: TypeRegistry, method resolution, effect analysis, and VM slot dispatch
- removed duplicate TypeRegistry `MAP_METHOD_EXTRAS` table
- smoke: `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_surface_catalog_vm.sh`

## Follow-Up Cleanup Candidates

- `length` is handled by
  `docs/development/current/main/phases/phase-291x/291x-97-mapbox-length-alias-card.md`.
- `MapBox.get(missing-key)` tagged read-miss contract is documented in
  `docs/development/current/main/phases/phase-291x/291x-101-mapbox-get-missing-key-contract-card.md`;
  `291x-110` lands the conservative successful-read rule: publish `V` only for
  receiver-local homogeneous Map facts with tracked literal keys; mixed,
  untyped, and missing-key reads stay `Unknown`.
- `.hako` VM source routes for `keys` / `values` / `remove` / `clear` are
  landed through separate state-owner cards; do not reopen them in router-only
  cleanup.
- `MapBox.delete` / `remove` router promotion is landed in
  `docs/development/current/main/phases/phase-291x/291x-104-mapbox-delete-remove-router-card.md`;
- `MapBox.clear` router promotion is landed in
  `docs/development/current/main/phases/phase-291x/291x-105-mapbox-clear-router-card.md`.
- `291x-109` is the landed boundary cleanup card for the surviving
  selfhost-runtime `OpsCalls.map_has(...)` wrapper and the compat-only Rust ABI
  quarantine in `crates/nyash_kernel/src/plugin/map_compat.rs`.
- `291x-110` is the successful-read typing card for `MapBox.get(existing-key)`;
  it adds conservative receiver-local publication without changing the landed
  missing-key text contract.
- legacy `apps/std/map_std.hako` JIT-only placeholder was deleted after inventory; do not recreate it as a second std owner.
- unused `lang/src/vm/hakorune-vm/map_keys_values_bridge.hako` prototype was deleted; do not recreate it outside the active VM route owner.
- `apps/lib/boxes/map_std.hako` was deleted after `OpsCalls.map_has(...)` took the remaining Map-only wrapper behavior.
- `crates/nyash_kernel/src/plugin/map_compat.rs` remains compat-only quarantine; do not delete in a surface catalog commit.
