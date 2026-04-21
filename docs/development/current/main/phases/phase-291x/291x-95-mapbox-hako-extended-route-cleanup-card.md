---
Status: Ready
Date: 2026-04-22
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

The next implementation must choose one owner path for `keys`, `values`,
`remove`, and `clear` before promoting them in source-level `.hako` VM routes.

## Current Facts

- Rust catalog rows exist for `keys`, `values`, `delete/remove`, and `clear`.
- The hako-visible MapBox smoke intentionally pins only `size/len/set/get/has`.
- Source-level `.hako` VM routes for `keys/values/remove/clear` still have
  stub/debt behavior and must not be silently promoted.
- `apps/std/map_std.hako`, `apps/lib/boxes/map_std.hako`, and the unused
  `map_keys_values_bridge.hako` prototype have been deleted.

## Candidate Owner Choices

1. Promote through the existing `.hako` Map state/raw-handle owner.
2. Keep `.hako` routes stubbed and document Rust-only visibility until the VM
   source route can share the same state owner.
3. Add a narrow adapter only if it is owned by the active Map state route and
   pinned by smoke.

## Acceptance For Promotion

- No by-name hardcoded bypass outside the selected owner.
- `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_surface_catalog_vm.sh`
  is extended only after the selected `.hako` route is real.
- `remove` alias must be tested separately from `delete`.
- `clear` return behavior must remain current or get a separate return-contract
  decision.
- `keys/values` result shape must be pinned in smoke.

## Out Of Scope

- `length` alias for MapBox Rust vtable.
- `size`/`len` slot unification.
- `crates/nyash_kernel/src/plugin/map_compat.rs` deletion.
