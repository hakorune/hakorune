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

- lane: `phase-137x main kilo reopen selection`
- current front: `kilo_kernel_small_hk` 再ベースライン + `kilo_micro_substring_concat` / `kilo_micro_array_getset` 再確認
- blocker: `nyash_kernel` の構造分割は landed。split kernel 上で `main kilo` を reopen する
- recent landed:
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
- before `main kilo`, current work re-cut `nyash_kernel` into four buckets:
  - `keep`
  - `thin keep`
  - `compat glue`
  - `substrate candidate`
- landed source slices:
  - `crates/nyash_kernel/src/exports/string.rs` split
  - `crates/nyash_kernel/src/plugin/map_substrate.rs` thin-alias recut
- target shape:
  - `phase-137x main kilo reopen selection`

## Root Anchors

- root anchor: `CURRENT_TASK.md`
- quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- one-screen map: `docs/development/current/main/15-Workstream-Map.md`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/phases/phase-137x/README.md`
