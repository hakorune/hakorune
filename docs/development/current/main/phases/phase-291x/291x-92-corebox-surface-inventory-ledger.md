---
Status: Active
Date: 2026-04-22
Scope: CoreBox surface catalog の棚卸し ledger。結論だけ current docs に反映する。
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md
  - docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-93-mapbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md
---

# CoreBox Surface Inventory Ledger

## ArrayBox Baseline

Landed in phase-290x:

- Rust owner: `src/boxes/array/surface_catalog.rs`
- method id: `ArrayMethodId`
- invoke seam: `ArrayBox::invoke_surface(...)`
- stable smoke: `tools/smokes/v2/profiles/integration/apps/phase290x_arraybox_surface_catalog_vm.sh`

Stable rows:

- `length` with `size` / `len`
- `get`
- `set`
- `push`
- `pop`
- `slice`
- `remove`
- `insert`

Deferred:

- `clear/contains/indexOf/join/sort/reverse`
- direct source `slice()` result follow-up calls through `RuntimeDataBox` union receiver

## StringBox Landing Snapshot

Primary files:

- `src/boxes/basic/string_box.rs`
- `src/boxes/string_box.rs`
- `src/runtime/type_registry.rs`
- `src/runtime/core_box_ids/specs/basic.rs`
- `src/backend/mir_interpreter/handlers/boxes_string.rs`
- `src/backend/mir_interpreter/handlers/calls/method.rs`
- `src/backend/mir_interpreter/handlers/calls/method/dispatch.rs`
- `lang/src/runtime/collections/string_core_box.hako`
- `lang/src/vm/boxes/generated/abi_adapter_registry_defaults.hako`
- `apps/std/string.hako`

Landed fix:

- `src/boxes/basic/string_surface_catalog.rs` now owns the stable first row set.
- `StringBox::invoke_surface(...)` is the Rust invoke seam for those rows.
- TypeRegistry aliases now come from the catalog.
- Rust VM dispatch and `.hako` VM-facing `StringCoreBox` consume the same stable rows.
- `tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_surface_catalog_vm.sh` pins aliases, values, and no-stub VM drift.

Remaining drift:

- `lastIndexOf` one-arg and two-arg are implemented through the catalog and
  Unified value path; `291x-103` owns the landing snapshot.
- `apps/std/string.hako` implements `string_index_of` manually instead of being the semantic owner.
- `toUpper` / `toLower` exposure exists in TypeRegistry extras, but route ownership is not clear enough for the first catalog slice.

Completed first implementation:

- catalog the stable rows listed in `291x-91`
- route Rust dispatch and TypeRegistry through that catalog
- add a smoke for aliases and values
- leave wider method families for follow-up

Completed cleanup:

- legacy `std.string2.hako` diagnostic residue was retired in a follow-up cleanup

## Router / Value World Follow-up

Confirmed route seam:

- `src/mir/builder/router/policy.rs` routes only `StringBox.length` / `len` /
  `size`, `StringBox.substring` / `substr`, `StringBox.concat`, and
  `StringBox.trim`, `StringBox.contains`, one-arg and two-arg
  `StringBox.lastIndexOf`, `StringBox.replace`, and `StringBox.indexOf` /
  `find`, plus the current ArrayBox stable rows and MapBox
  `size` / `length` / `len` / `has` / `get` / `set` / `keys` / `values` /
  `delete` / `remove` through the Unified value path.
  Non-allowlisted `StringBox`, `ArrayBox`, and `MapBox` methods still use the
  family-wide `core_box` BoxCall fallback.
- `src/mir/builder/utils/boxcall_emit.rs` maps `MirType::String` receivers to
  `"StringBox"` before calling `choose_route(...)`.

Consequence:

- value-typed string calls can be seen as `StringBox` and kept on the BoxCall
  fallback path, even when the desired long-term path is Unified / Value World.
- the fallback remains behaviorally important because it also publishes return
  types and emits canonical method calls for legacy paths.

Landed first slice and follow-up:

- `docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md`
- first implementation was StringBox-only and method-allowlisted.
- second implementation moved `substring` / `substr` after adding a focused
  fixture and catalog-backed return-type alias publication.
- third implementation moved `concat` after adding a focused receiver-plus-arg
  fixture and keeping `indexOf` pinned as the BoxCall fallback sentinel.
- fourth implementation moved `trim` after adding a focused arity-zero
  String-return fixture.
- fifth implementation moved `contains` after adding a focused
  receiver-plus-needle fixture and Bool return-type publication assertion.
- sixth implementation moved one-arg `lastIndexOf` after adding a focused
  receiver-plus-needle fixture, Integer return-type publication assertion, and
  a `lastIndexOf/2` fallback assertion.
- seventh implementation moved `replace` after adding a focused
  receiver-plus-old-plus-new fixture and String return-type publication
  assertion.
- eighth implementation moved `indexOf` / `find` after adding focused one-arg
  and two-arg receiver-shape fixtures and Integer return-type publication
  assertions.
- ninth implementation moved `ArrayBox.length` / `size` / `len` after adding
  focused arity-zero receiver-shape fixtures and Integer return-type
  publication assertions.
- tenth implementation moved `ArrayBox.push` after adding a focused
  receiver-plus-value fixture and preserving the remaining ArrayBox fallback
  sentinels.
- eleventh implementation moved `ArrayBox.slice` after adding a focused
  receiver-plus-start-plus-end fixture and preserving the generic/value and
  write fallback sentinels.
- twelfth implementation moved `MapBox.size` after adding a focused
  receiver-shape fixture and preserving `MapBox.len` / `MapBox.has` fallback
  sentinels.
- thirteenth implementation moved `MapBox.len` after adding a focused
  receiver-shape fixture and preserving `MapBox.has` fallback sentinels.
- fourteenth implementation moved `MapBox.has` after adding a focused
  receiver-plus-key fixture and preserving `MapBox.get` fallback sentinels.
- fifteenth implementation moved `ArrayBox.get` after adding a focused
  receiver-plus-index fixture while keeping its generic result type
  `Unknown` and preserving `ArrayBox.pop` fallback sentinels.
- sixteenth implementation moved `ArrayBox.pop` after adding a focused
  receiver-only fixture while keeping its generic result type `Unknown` and
  preserving `ArrayBox.set` fallback sentinels.
- seventeenth implementation moved `ArrayBox.set` after adding a focused
  receiver-plus-index-plus-value fixture and preserving `ArrayBox.remove`
  fallback sentinels.
- eighteenth implementation moved `ArrayBox.remove` after adding a focused
  receiver-plus-index fixture while keeping its generic result type `Unknown`
  and preserving `ArrayBox.insert` fallback sentinels.
- nineteenth implementation moved `ArrayBox.insert` after adding a focused
  receiver-plus-index-plus-value fixture and preserving MapBox fallback
  sentinels.
- twentieth implementation moved `MapBox.get` after adding a focused
  receiver-plus-key fixture while keeping its stored-value result type
  `Unknown` and preserving `MapBox.set` fallback sentinels.
- twenty-first implementation moved `MapBox.set` after adding a focused
  receiver-plus-key-plus-value fixture while keeping its write-return type
  `Unknown` and preserving `MapBox.delete` fallback sentinels.
- twenty-second implementation added `MapBox.length` as a read-only alias for
  the existing size surface, with no new slot and no extended-row promotion.
- twenty-third implementation moved StringBox `lastIndexOf/2` after adding
  focused catalog, return-type, duplicate-receiver, and vm-hako smoke coverage:
  `docs/development/current/main/phases/phase-291x/291x-103-stringbox-lastindexof-start-card.md`
- twenty-fourth implementation moved `MapBox.delete` / `remove` after adding
  focused route, MIR shape, receipt-string type, duplicate-receiver, and
  source smoke coverage:
  `docs/development/current/main/phases/phase-291x/291x-104-mapbox-delete-remove-router-card.md`
- remaining route-only CoreBox rows are closed for ArrayBox stable rows and
  MapBox `size/length/len/has/get/set/keys/values/delete/remove`.
- next implementation should choose one remaining CoreBox method family and
  keep it separate from the other route flips.
- hold MapBox `clear` router promotion until its route fixture and
  return/write contract witness are pinned.

## MapBox Current Duplication

Primary files to inventory before coding:

- `src/boxes/map_box.rs`
- `src/runtime/type_registry.rs`
- `src/backend/mir_interpreter/handlers/calls/method/dispatch.rs`
- `lang/src/runtime/collections/map_core_box.hako`
- `lang/src/runtime/collections/map_state_core_box.hako`
- `lang/src/runtime/substrate/raw_map/raw_map_core_box.hako`
- `crates/nyash_kernel/src/plugin/map_compat.rs`
- `lang/src/mir/builder/internal/lower_method_map_get_set_box.hako`
- `lang/src/mir/builder/internal/lower_method_map_size_box.hako`
- `docs/development/current/main/phases/phase-29cm/README.md`

Known drift:

- visible surface and compat ABI are split.
- current vtable rows register `size` at slot `200` and `len` at slot `201`;
  `length` is now a catalog alias for the existing size surface.
- `remove` is registered as the same slot as `delete`; source-level remove
  parity is landed, while router promotion remains a separate mutating-row
  card.
- `set` / `delete` / `remove` / `clear` source-level write-return receipt
  contracts are landed; router promotion for `delete` / `remove` is landed in
  `291x-104`, while `clear` remains separate.
- bad-key validation is normalized for the source-visible rows.
- raw substrate helpers are already better separated than StringBox, so MapBox should be cataloged after StringBox rather than before it.

Current slot inventory:

| Canonical | Aliases / routes | Arity | Slot | Notes |
| --- | --- | ---: | ---: | --- |
| `size` | `.hako` also accepts `length` | 0 | 200 | read-only count |
| `len` | size-equivalent | 0 | 201 | read-only count |
| `has` |  | 1 | 202 | read-only boolean |
| `get` | `getField` bridge outside vtable | 1 | 203 | read-only value/missing path |
| `set` | `setField` bridge outside vtable | 2 | 204 | mutates; return contract drift |
| `delete` | `remove` in TypeRegistry | 1 | 205 | mutates; alias drift |
| `keys` |  | 0 | 206 | read-only array |
| `values` |  | 0 | 207 | read-only array |
| `clear` |  | 0 | 208 | mutates; return contract drift |

MapBox first safe slice after StringBox:

- create catalog rows for current vtable rows only
- add a guard that TypeRegistry slot lookup matches the catalog
- do not normalize aliases or return contracts in the first MapBox commit
- keep `length`, `birth`, `getField`, `setField`, `forEach`, and `toJSON` in a
  non-vtable/debt section until a policy card accepts them

First slice owner decision:

- Rust catalog owner: `src/boxes/map_surface_catalog.rs`
- Rust invoke seam: `MapBox::invoke_surface(...)`
- Rust consumers to thin in the same commit:
  - `src/runtime/type_registry.rs`
  - `src/mir/builder/calls/method_resolution.rs`
  - `src/mir/builder/calls/effects_analyzer.rs`
  - `src/backend/mir_interpreter/handlers/calls/method.rs`
  - `src/backend/mir_interpreter/handlers/calls/method/dispatch.rs`
- `.hako` visible owner remains `lang/src/runtime/collections/map_core_box.hako`
  for state/raw-handle orchestration in this slice.

## MapBox Landing Snapshot

Landed first implementation:

- `src/boxes/map_surface_catalog.rs` owns the current Rust vtable row set.
- `MapBox::invoke_surface(...)` is the Rust invoke seam for those rows.
- TypeRegistry rows now come from `MAP_SURFACE_METHODS`; the old `MAP_METHOD_EXTRAS`
  table was removed.
- Rust VM slot dispatch delegates cataloged MapBox slots to the invoke seam.
- method resolution and effect analysis read `MapMethodId`.
- `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_surface_catalog_vm.sh`
  pins Rust catalog rows and the hako-visible `size/len/set/get/has` VM subset.

Remaining drift:

- `size` and `len` keep separate slots.
- visible surface and compat ABI remain split.
- `delete` / `remove` router promotion is landed in `291x-104`; `clear`
  router promotion remains separate from the landed source-owner/write-return
  contracts.

Completed cleanup:

- legacy `apps/std/map_std.hako` JIT-only placeholder was deleted after reference
  inventory showed no active import/module-registry route.
- unused `lang/src/vm/hakorune-vm/map_keys_values_bridge.hako` prototype was
  deleted after reference inventory showed no active import/module-registry route.
- `apps/lib/boxes/map_std.hako` was deleted after moving the remaining live
  `pref == "ny"` Map-only wrapper to `OpsCalls.map_has(...)` and refreshing the
  stage1 module snapshot.
