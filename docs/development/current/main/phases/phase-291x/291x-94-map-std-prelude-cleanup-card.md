---
Status: Landed
Date: 2026-04-22
Scope: `apps/lib/boxes/map_std.hako` cleanup card; docs-first boundary before code.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/development/current/main/phases/phase-291x/291x-93-mapbox-surface-task-board.md
---

# MapStd Prelude Cleanup Card

## Decision

`apps/lib/boxes/map_std.hako` was the only remaining live Map std scaffold. Do
not reintroduce it as a pure wrapper: its only active `OpsCalls` behavior was
the `has` null/get fallback, now owned by `OpsCalls.map_has(...)`.

The cleanup is allowed only as a behavior-preserving prelude/registry card.

## Current Owner Boundary

- `src/boxes/map_surface_catalog.rs` owns Rust MapBox vtable rows.
- `lang/src/runtime/collections/map_core_box.hako` remains the visible `.hako`
  state/raw-handle owner for this phase.
- `OpsCalls.map_has(...)` owns the remaining `pref == "ny"` Map-only wrapper
  behavior.

## Blocking References

- `apps/selfhost-runtime/selfhost_prelude.hako`
- `apps/selfhost-runtime/ops_calls.hako`
- `hako.toml`
- `nyash.toml`
- `tools/checks/module_registry_hako_top_only_allowlist.txt`
- `tools/checks/module_registry_nyash_top_only_allowlist.txt`
- `src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json`

## Landed Cleanup

- Replaced `new MapStd().has(...)` with `OpsCalls.map_has(...)`.
- Removed unreachable `new MapStd()` rows for `size/get/set/toString`; those
  names were already handled earlier by the generic receiver wrappers.
- Routed `OpsCalls` `print/println/log` through `ConsoleStd.log(...)`, which
  already has the same null/default emit behavior and avoids the reserved
  `print` selector parse issue.
- Removed `using apps.lib.boxes.map_std` from `selfhost_prelude`.
- Removed `apps.lib.boxes.map_std` from `hako.toml`, `nyash.toml`, and both
  module-registry top-only allowlists.
- Refreshed `src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json`.
- Deleted `apps/lib/boxes/map_std.hako`.

## Acceptance

Passed:

- `cargo check -q`
- `tools/checks/current_state_pointer_guard.sh`
- `bash tools/checks/module_registry_hygiene_guard.sh`
- `cargo test embedded_snapshot_matches_registry_doc --lib`
- `bash tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_surface_catalog_vm.sh`

Known blocked verification:

- direct `OpsCalls` import smoke still stops on the existing dynamic fallback
  checker issue for `recv[method]`; fix that in a separate selfhost-runtime
  dispatcher card before promoting broader `pref == "ny"` coverage.

## Out Of Scope

- MapBox alias normalization (`length`, `size`/`len` unification).
- `.hako` VM source route promotion for `keys/values/remove/clear`.
- `crates/nyash_kernel/src/plugin/map_compat.rs` deletion.
