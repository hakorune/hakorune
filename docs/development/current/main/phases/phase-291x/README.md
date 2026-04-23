---
Status: Active
Date: 2026-04-22
Scope: CoreBox surface catalog を ArrayBox から StringBox / MapBox へ広げる phase front。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-290x/README.md
  - docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md
  - docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/development/current/main/phases/phase-291x/291x-93-mapbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-94-map-std-prelude-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-95-mapbox-hako-extended-route-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md
  - docs/development/current/main/phases/phase-291x/291x-97-mapbox-length-alias-card.md
---

# Phase 291x: CoreBox surface catalog

- Status: Active
- Date: 2026-04-22
- Purpose: phase-290x の `ArrayBox` catalog/invoke seam を、CoreBox surface の横断ルールへ上げる。
- Landed implementation targets:
  - `StringBox`
  - `MapBox` first current-vtable slice
- Next implementation target: MapBox non-empty extended state parity before
  `remove` / content enumeration
- Sibling guardrail:
  - `docs/development/current/main/phases/phase-137x/README.md`
  - phase-137x remains observe-only unless app work produces a real blocker

## Decision

ArrayBox で固定した読み方を CoreBox 全体へ広げる。

```text
surface contract
  -> canonical name / aliases / arity / slot / effect / return

execution dispatch
  -> one invoke seam per Box family

exposure state
  -> runtime / VM / std sugar / docs / smoke pinned state
```

ただし、最初の code slice で `StringBox` と `MapBox` を同時に触らない。
phase-291x の初回実装は `StringBox` だけに閉じる。

## Reading Order

1. `docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md`
2. `docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md`
3. `docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md`
4. `docs/development/current/main/phases/phase-291x/291x-93-mapbox-surface-task-board.md`
5. `docs/development/current/main/phases/phase-291x/291x-94-map-std-prelude-cleanup-card.md`
6. `docs/development/current/main/phases/phase-291x/291x-95-mapbox-hako-extended-route-cleanup-card.md`
7. `docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md`
8. `docs/development/current/main/phases/phase-291x/291x-97-mapbox-length-alias-card.md`

## Current Rule

- docs-first before code
- `StringBox.length()` is canonical; `len()` and `size()` are compatibility aliases
- `StringBox.indexOf(needle, start)` is stable; `find` is compatibility alias
- `StringBox.lastIndexOf(needle, start_pos)` remains explicitly deferred until a separate card
- `apps/std/string.hako` is sugar, not the semantic owner
- legacy `apps/std/string2.hako` diagnostic residue was deleted by an explicit cleanup card
- `MapBox` first slice cataloged current Rust vtable rows only
- do not add `length` as a Rust vtable alias in the first MapBox commit
- do not collapse `size` and `len` slots in the first MapBox commit
- do not normalize `set` / `delete` / `clear` return contracts in the first MapBox commit
- `MapBox.length` is now a separate contract-first alias slice; it must not
  promote `keys` / `values` / `delete` / `remove` / `clear`

## Implementation State

Landed first implementation card:

```text
String surface catalog
  -> StringMethodId
  -> StringBox::invoke_surface(...)
  -> thin registry / method-resolution / dispatch consumers
  -> stable String surface smoke
```

Landed smoke:

- `tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_surface_catalog_vm.sh`

MapBox is now the next CoreBox catalog target.

Landed MapBox card:

```text
Map surface catalog
  -> MapMethodId
  -> MapBox::invoke_surface(...)
  -> thin registry / method-resolution / effect-analysis / VM dispatch consumers
  -> stable MapBox surface smoke for Rust catalog + hako-visible VM subset
```

Landed smoke:

- `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_surface_catalog_vm.sh`

Remaining MapBox follow-up:

- source-level vm-hako empty `MapBox.values().size()` shape is landed and
  pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_extended_values_vm.sh`.
- source-level vm-hako `MapBox.set(...)` duplicate receiver stripping is landed
  and pinned by
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_set_multiarg_vm.sh`.
- non-empty `keys()` / `values()` state parity still needs one owner decision:
  `set()` updates S0 `MapStateCoreBox` state, while v1 `values()` currently
  reads `MapCoreBox` state.
- `.hako` VM `keys` / `remove` / `clear` source-route behavior must still be
  promoted one row at a time and smoke-pinned.
- legacy `apps/std/map_std.hako` JIT-only placeholder was deleted; it was not an active module-registry/prelude route.
- unused `lang/src/vm/hakorune-vm/map_keys_values_bridge.hako` prototype was deleted; it was not an active VM route.
- `apps/lib/boxes/map_std.hako` prelude/module-registry dependency was deleted by the phase-291x cleanup card.
- landed alias card:
  `docs/development/current/main/phases/phase-291x/291x-97-mapbox-length-alias-card.md`
- active extended-route card:
  `docs/development/current/main/phases/phase-291x/291x-95-mapbox-hako-extended-route-cleanup-card.md`

Landed CoreBox router first slice:

- `src/mir/builder/router/policy.rs` now routes only the catalog-backed
  `StringBox.length` / `len` / `size` and `StringBox.substring` / `substr`
  and `StringBox.concat`, `StringBox.trim`, `StringBox.contains`, and one-arg
  `StringBox.lastIndexOf`, `StringBox.replace`, and `StringBox.indexOf` /
  `find`, plus `ArrayBox.length` / `size` / `len`, `ArrayBox.push`,
  `ArrayBox.slice`, `ArrayBox.get`, `ArrayBox.pop`, `ArrayBox.set`,
  `ArrayBox.remove`, `ArrayBox.insert`, `MapBox.size`, `MapBox.length`,
  `MapBox.len`, and `MapBox.has`, `MapBox.get`, and `MapBox.set` rows through
  `Route::Unified`.
- `src/mir/builder/utils/boxcall_emit.rs` still bridges `MirType::String` to
  `StringBox` before route selection; uncovered methods remain on the BoxCall
  fallback.
- `ArrayBox.get` intentionally stays `MirType::Unknown`; the returned element
  type remains data-dependent.
- `ArrayBox.pop` intentionally stays `MirType::Unknown` for the same
  data-dependent element-return reason.
- `ArrayBox.set` follows the write-`Void` contract already used by
  `ArrayBox.push`.
- `ArrayBox.remove` intentionally stays `MirType::Unknown` for the same
  data-dependent element-return reason as `get` / `pop`.
- `ArrayBox.insert` follows the same write-`Void` contract already used by
  `ArrayBox.push` / `ArrayBox.set`.
- `MapBox.get` intentionally stays `MirType::Unknown` because stored map values
  are data-dependent.
- `MapBox.set` intentionally stays `MirType::Unknown`; current visible
  write-return and bad-key behavior stay contract-first cleanup.
- two-arg `lastIndexOf`, MapBox extended rows, and MapBox write-return /
  bad-key normalization remain contract-first cleanup cards.
- task card: `docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md`
