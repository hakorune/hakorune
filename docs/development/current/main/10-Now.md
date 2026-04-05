---
Status: SSOT
Date: 2026-04-05
Scope: current lane / blocker / next pointer だけを置く薄い mirror。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
---

# Self Current Task — Now (main)

## Current

- lane: `phase-140x map owner pilot`
- current front: `MapCoreBox` / `MapStateCoreBox` を visible semantics owner とし、Rust は thin map facade / raw substrate / compat forwarding に限定する
- blocker: `Array owner` seam は landed。次は `Map owner` の exact seam を current source に固定する
- recent landed:
  - `phase-139x array owner pilot`
  - `phase-138x nyash_kernel semantic owner cutover`
  - `phase-134x nyash_kernel layer recut selection`
  - `phase-133x micro kilo reopen selection`

## Current Read

- `vm` cleanup is no longer current work
- fixed perf order stays:
  - `leaf-proof micro`
  - `micro kilo`
  - `main kilo`
- `phase-133x` is closed:
  - `kilo_micro_substring_concat`: parity locked
  - `kilo_micro_array_getset`: parity locked
  - `kilo_micro_indexof_line`: frozen faster than C
- `phase-134x` re-cut `nyash_kernel` into four buckets:
  - `keep`
  - `thin keep`
  - `compat glue`
  - `substrate candidate`
- landed source slices:
  - `crates/nyash_kernel/src/exports/string.rs` split
  - `crates/nyash_kernel/src/plugin/map_substrate.rs` thin-alias recut
- current architecture target is fixed:
  - `Rust host microkernel`
  - `.hako semantic kernel`
  - `native accelerators`
  - `ABI facade` as thin keep
  - `compat quarantine` as non-owner
- current seam:
  - owner: `map_core_box.hako` / `map_state_core_box.hako`
  - substrate: `raw_map_core_box.hako`
  - thin facade: `map_aliases.rs`
  - observer shim: `map_substrate.rs`
  - compat/runtime forwarding: `map_runtime_facade.rs`
  - accelerators: `map_probe.rs` / `map_slot_load.rs` / `map_slot_store.rs`
- perf lane is paused, not cancelled:
  - `phase-137x main kilo reopen selection` remains the successor after semantic owner cutover

## Root Anchors

- root anchor: `CURRENT_TASK.md`
- quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- one-screen map: `docs/development/current/main/15-Workstream-Map.md`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/phases/phase-140x/README.md`
3. `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
