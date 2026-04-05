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

- lane: `phase-139x array owner pilot`
- current front: `ArrayCoreBox` / `ArrayStateCoreBox` を visible semantics owner とし、Rust は `ABI facade` / `raw substrate` / `native accelerators` に限定する
- blocker: final owner graph は fixed。次は Array owner の exact seam を current source に固定する
- recent landed:
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
  - owner: `array_core_box.hako` / `array_state_core_box.hako`
  - substrate: `raw_array_core_box.hako` / `ptr_core_box.hako`
  - ABI facade: `array_substrate.rs`
  - compat/runtime forwarding: `array_runtime_facade.rs`
  - accelerators: `array_handle_cache.rs` / `array_string_slot.rs`
- perf lane is paused, not cancelled:
  - `phase-137x main kilo reopen selection` remains the successor after semantic owner cutover

## Root Anchors

- root anchor: `CURRENT_TASK.md`
- quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- one-screen map: `docs/development/current/main/15-Workstream-Map.md`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/phases/phase-139x/README.md`
3. `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
