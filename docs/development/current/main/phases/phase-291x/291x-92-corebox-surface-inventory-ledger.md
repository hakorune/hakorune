---
Status: Active
Date: 2026-04-22
Scope: CoreBox surface catalog の棚卸し ledger。結論だけ current docs に反映する。
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md
  - docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-93-mapbox-surface-task-board.md
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

- `lastIndexOf` one-arg is implemented; two-arg is documented as a gap.
- `apps/std/string.hako` implements `string_index_of` manually instead of being the semantic owner.
- `toUpper` / `toLower` exposure exists in TypeRegistry extras, but route ownership is not clear enough for the first catalog slice.

Completed first implementation:

- catalog the stable rows listed in `291x-91`
- route Rust dispatch and TypeRegistry through that catalog
- add a smoke for aliases and values
- leave wider method families for follow-up

Completed cleanup:

- legacy `std.string2.hako` diagnostic residue was retired in a follow-up cleanup

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
  `.hako` owner paths also accept `length`, but `length` is not currently a
  vtable-registered Rust alias.
- `remove` is registered as the same slot as `delete`, but not all dispatch
  paths accept it as a visible alias.
- `set` / `delete` / `clear` return contracts differ between Rust VM, core
  specs, and `.hako` state paths.
- bad-key validation is enforced in the Rust `boxes_map.rs` path but can be
  bypassed by direct slot dispatch.
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

- `.hako` VM source route still stubs `keys` / `values` / `remove` / `clear`.
- `length` remains `.hako` compatibility/debt and is not a Rust vtable alias.
- `size` and `len` keep separate slots.
- `set` / `delete` / `clear` current Rust receipt values are unchanged.
- `apps/lib/boxes/map_std.hako` is a P0 scaffold imported by the selfhost prelude,
  so deletion requires a separate module-registry/prelude cleanup card.

Completed cleanup:

- legacy `apps/std/map_std.hako` JIT-only placeholder was deleted after reference
  inventory showed no active import/module-registry route.
