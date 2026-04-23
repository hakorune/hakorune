---
Status: Active
Date: 2026-04-23
Scope: `.hako` MapBox extended-route cleanup decision after Rust catalog and std scaffold cleanup.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/development/current/main/phases/phase-291x/291x-93-mapbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-94-map-std-prelude-cleanup-card.md
---

# MapBox Hako Extended Route Cleanup Card

## Decision

Treat `.hako` VM extended routes as a separate owner decision, not as part of the
Rust MapBox vtable catalog or std scaffold cleanup.

Owner choice: source-level vm-hako `MapBox` extended rows that observe mutable
map state must be normalized at the vm-hako payload boundary into S0
`boxcall(...)` rows, then handled by
`lang/src/runtime/collections/map_state_core_box.hako` from
`lang/src/vm/boxes/mir_vm_s0_boxcall_builtin.hako`.

`MapStateCoreBox` owns non-empty state parity because `set/delete/clear` already
mutate its S0 state store. `MapCoreBox` remains the v1 visible-owner fallback
for direct `mir_call` rows and the empty `values()` compatibility shape, but it
must not be the non-empty source-level owner for rows whose result depends on
S0 mutation state. `MirCallMapOpsBox` remains the separate core `mir_call`
executor owner, and Rust `MapBox::invoke_surface(...)` remains the Rust
VM/vtable owner. Do not cross-call between those lanes.

Landed promotion slices: empty-map `MapBox.values()` as an ArrayBox-like shape,
source-level vm-hako `MapBox.set(...)` duplicate receiver stripping,
non-empty `values()` parity, and non-empty `keys()` parity through the same S0
state owner as `set()` via
`src/runner/reference/vm_hako/payload_normalize.rs`. `remove(key)` is also
landed as a `delete(key)` owner alias. The next slice is `clear()`. Content
enumeration is deferred until a separate shape contract pins key/value ordering
and element publication.

Implementation note: the source route returns an ArrayBox-like value through
ordinary MIR `copy` instructions before `values().size()` observes it. Therefore
`MirVmS0StateOpsBox.copy_reg_payload(...)` must propagate VM-local receiver
metadata such as `__vm.recv.box:*`; otherwise the later `RuntimeDataBox.size`
call loses the ArrayBox hint and falls into String size behavior.
The ArrayBox-like shape must also carry per-receiver length metadata
(`__vm_len:*`), and `ArrayCoreBox.size/len/length` must prefer that VM-local
metadata over treating the synthetic register id as a runtime array handle.

## Current Facts

- Rust catalog rows exist for `keys`, `values`, `delete/remove`, and `clear`.
- The Rust catalog smoke intentionally pins the direct Rust VM route for
  `size/length/len/set/get/has`; it must not own vm-hako source-route behavior.
- source-level vm-hako v1 `mir_call` currently routes MapBox visible methods
  through `MapCoreBox`; missing rows fall to `[vm/method/stub:*]`.
- S0 BoxCall rows for `keys`, `values`, `delete`, and `clear` already route through
  `MapStateCoreBox`.
- The S0 BoxCall row for the `remove` alias is still absent and must not be
  silently promoted.
- source-level vm-hako `MapBox.set(...)` used to expose a multi-arg BoxCall
  blocker when Unified MIR passed `[receiver_alias, key, value]`; this card
  strips that duplicate receiver arg in the S0 MapBox owner.
- non-empty `MapBox.values()` now goes through payload normalization to
  S0 `boxcall(values)` and reads the same `MapStateCoreBox` length store
  written by `set()`.
- non-empty `MapBox.keys()` now goes through payload normalization to
  S0 `boxcall(keys)` and reads the same `MapStateCoreBox` length store
  written by `set()`.
- `MapBox.remove(key)` now goes through payload normalization to
  S0 `boxcall(delete)`, so it mutates the same `MapStateCoreBox` state store
  as `delete(key)`.
- MIR `copy` previously copied scalar/kind/handle/file payload but not
  VM-local receiver metadata; this card may extend copy metadata propagation
  only for existing local metadata keys.
- `apps/std/map_std.hako`, `apps/lib/boxes/map_std.hako`, and the unused
  `map_keys_values_bridge.hako` prototype have been deleted.

## Landed Slice

- `MapCoreBox.try_handle(..., "values")` returns an ArrayBox-like empty shape.
- `MapStateCoreBox.apply_values(...)` and the S0 BoxCall dispatcher row exist
  for the same surface method.
- `MirVmS0StateOpsBox.copy_reg_payload(...)` propagates existing VM-local
  receiver metadata across MIR `copy`.
- `MirVmS0BoxcallBuiltinBox` strips a duplicate receiver arg for MapBox
  `set/get/has/delete` rows when Unified MIR emits `expected_arity + 1` args.
- vm-hako payload normalization rewrites source-level `mir_call(MapBox.values)`
  into S0 `boxcall(values)`, preserving the optional receiver-mirror arg.
- vm-hako payload normalization rewrites source-level `mir_call(MapBox.keys)`
  into S0 `boxcall(keys)`, preserving the optional receiver-mirror arg.
- vm-hako payload normalization rewrites source-level `mir_call(MapBox.remove)`
  into S0 `boxcall(delete)`.
- `MapStateCoreBox` writes ArrayBox-like per-receiver length metadata for
  `keys()` / `values()`, and `ArrayCoreBox` consumes that metadata before
  runtime handle length for VM-local ArrayBox-like shapes.
- source-level non-empty `MapBox.values().size()` is pinned at `2`.
- source-level non-empty `MapBox.keys().size()` is pinned at `2`.
- source-level `MapBox.remove(key)` is pinned by `has(key)==false` and
  `size()==1`.
- Smoke:
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_extended_values_vm.sh`
- Smoke:
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_extended_keys_vm.sh`
- Smoke:
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_extended_remove_vm.sh`
- Smoke:
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_set_multiarg_vm.sh`

## Rejected Owner Choices

- Do not add a new by-name adapter outside the selected source owner
  `MapCoreBox` or the S0 BoxCall owner `MapStateCoreBox`.
- Do not make the Rust catalog smoke prove vm-hako behavior.
- Do not revive the deleted `map_keys_values_bridge.hako` prototype.
- Do not make `MirCallMapOpsBox` or a deleted bridge file the source-level
  owner for these rows.

## Acceptance For Promotion

- No by-name hardcoded bypass outside the selected owner.
- Add a companion phase-291x vm-hako source-route smoke for each promoted row.
- Keep `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_surface_catalog_vm.sh`
  as the direct Rust VM catalog smoke.
- `remove` alias must be tested separately from `delete`.
- `clear` return behavior must remain current or get a separate return-contract
  decision.
- `keys/values` result shape must be pinned in smoke.

## Next Slices

1. Landed: promote empty-map `values()` through `MapCoreBox` and pin
   `values().size() == 0`; add the matching S0 BoxCall row through
   `MapStateCoreBox`.
2. Landed: fix the source-level vm-hako multi-arg `MapBox.set(...)` witness by
   stripping the Unified duplicate receiver arg in the S0 MapBox BoxCall owner.
   This mirrors the Rust VM `execute_method_callee(...)` contract and must stay
   local to MapBox method rows.
3. Landed: land non-empty `values()` state-owner parity through payload
   normalization into the S0 `MapStateCoreBox` owner.
4. Landed: land non-empty `keys()` state-owner parity through payload
   normalization into the same S0 owner.
5. Landed: promote `remove(key)` as an alias of `delete(key)` with its own smoke.
6. Active next: promote `clear()` through the same S0 state owner with its own
   smoke.
7. Reactivate or replace stale archive witnesses only when they match the new
   owner path and have a valid helper source path.
8. Decide whether `keys()/values()` content enumeration is ordered, unordered,
   or intentionally size-only in vm-hako.

## Out Of Scope

- `length` alias for MapBox Rust vtable; landed in `291x-97`.
- `size`/`len` slot unification.
- `crates/nyash_kernel/src/plugin/map_compat.rs` deletion.
