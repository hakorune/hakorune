---
Status: Ready
Date: 2026-04-22
Scope: `apps/lib/boxes/map_std.hako` cleanup card; docs-first boundary before code.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/development/current/main/phases/phase-291x/291x-93-mapbox-surface-task-board.md
---

# MapStd Prelude Cleanup Card

## Decision

`apps/lib/boxes/map_std.hako` is the only remaining live Map std scaffold. Do
not delete it as a pure file cleanup: it still carries `OpsCalls` behavior for
`pref == "ny"` and is imported by the selfhost prelude/module registry.

The cleanup is allowed only as a behavior-preserving prelude/registry card.

## Current Owner Boundary

- `src/boxes/map_surface_catalog.rs` owns Rust MapBox vtable rows.
- `lang/src/runtime/collections/map_core_box.hako` remains the visible `.hako`
  state/raw-handle owner for this phase.
- `apps/lib/boxes/map_std.hako` is a temporary selfhost-runtime wrapper, not a
  semantic MapBox owner.

## Blocking References

- `apps/selfhost-runtime/selfhost_prelude.hako`
- `apps/selfhost-runtime/ops_calls.hako`
- `hako.toml`
- `nyash.toml`
- `tools/checks/module_registry_hako_top_only_allowlist.txt`
- `tools/checks/module_registry_nyash_top_only_allowlist.txt`
- `src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json`

## Minimum Cleanup Plan

1. Replace `new MapStd()` calls in `OpsCalls.dispatch(...)` with a
   behavior-preserving Map path.
2. Preserve current null/default behavior unless a separate semantics card
   changes it:
   - `size(null) -> 0`
   - `has(null, key) -> 0`
   - `get(null, key) -> null`
   - `set(null, key, value) -> 0`
   - `toString(null) -> "{}"`
3. Remove `using apps.lib.boxes.map_std` from `selfhost_prelude`.
4. Remove `apps.lib.boxes.map_std` from `hako.toml`, `nyash.toml`, and both
   module-registry top-only allowlists.
5. Refresh `src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json`.
6. Delete `apps/lib/boxes/map_std.hako`.

## Acceptance

- `cargo check -q`
- `tools/checks/current_state_pointer_guard.sh`
- `bash tools/checks/module_registry_hygiene_guard.sh` after the existing
  unrelated top-only alias debt is resolved or explicitly scoped out.
- `cargo test embedded_snapshot_matches_registry_doc`
- one focused selfhost-runtime smoke proving `pref == "ny"` Map dispatch for
  `size/has/get/set/toString`.

## Out Of Scope

- MapBox alias normalization (`length`, `size`/`len` unification).
- `.hako` VM source route promotion for `keys/values/remove/clear`.
- `crates/nyash_kernel/src/plugin/map_compat.rs` deletion.
