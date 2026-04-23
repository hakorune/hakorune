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

Owner choice: promote source-level vm-hako v1 `mir_call` rows through
`lang/src/runtime/collections/map_core_box.hako`, reached from
`lang/src/vm/boxes/mir_call_v1_handler.hako`.

`MapCoreBox` owns the source-level visible MapBox method contract and the
ArrayBox-like result shape for these v1 `mir_call` rows. `MapStateCoreBox`
remains the S0 BoxCall state owner reached from
`mir_vm_s0_boxcall_builtin.hako`. `MirCallMapOpsBox` remains the separate core
`mir_call` executor owner, and Rust `MapBox::invoke_surface(...)` remains the
Rust VM/vtable owner. Do not cross-call between those lanes.

First promotion slice: empty-map `MapBox.values()` as an ArrayBox-like shape
whose `size()` is `0`. Non-empty state parity is deferred because source-level
vm-hako `MapBox.set(...)` still has an independent multi-arg BoxCall blocker.
Content enumeration is deferred until a separate shape contract pins key/value
ordering and element publication.

Implementation note: the source route returns an ArrayBox-like value through
ordinary MIR `copy` instructions before `values().size()` observes it. Therefore
`MirVmS0StateOpsBox.copy_reg_payload(...)` must propagate VM-local receiver
metadata such as `__vm.recv.box:*`; otherwise the later `RuntimeDataBox.size`
call loses the ArrayBox hint and falls into String size behavior.

## Current Facts

- Rust catalog rows exist for `keys`, `values`, `delete/remove`, and `clear`.
- The Rust catalog smoke intentionally pins the direct Rust VM route for
  `size/length/len/set/get/has`; it must not own vm-hako source-route behavior.
- source-level vm-hako v1 `mir_call` currently routes MapBox visible methods
  through `MapCoreBox`; missing rows fall to `[vm/method/stub:*]`.
- S0 BoxCall rows for `keys`, `delete`, and `clear` already route through
  `MapStateCoreBox`.
- S0 BoxCall rows for `values` and the `remove` alias were absent before this
  card and must not be silently promoted.
- source-level vm-hako `MapBox.set(...)` still exposes a separate multi-arg
  BoxCall blocker; do not fold that into the `values()` shape card.
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
- Smoke:
  `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_extended_values_vm.sh`

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
2. Fix or replace the source-level vm-hako multi-arg `MapBox.set(...)` witness
   before pinning non-empty `keys()` / `values()` state parity.
3. Promote `remove(key)` as an alias of `delete(key)` with its own smoke.
4. Reactivate or replace stale archive witnesses only when they match the new
   owner path and have a valid helper source path.
5. Decide whether `keys()/values()` content enumeration is ordered, unordered,
   or intentionally size-only in vm-hako.

## Out Of Scope

- `length` alias for MapBox Rust vtable; landed in `291x-97`.
- `size`/`len` slot unification.
- `crates/nyash_kernel/src/plugin/map_compat.rs` deletion.
