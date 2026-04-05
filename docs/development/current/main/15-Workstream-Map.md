---
Status: Active
Date: 2026-04-05
Scope: current mainline / next lane / parked corridor の one-screen map。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
---

# Workstream Map

## Current Lane

| Item | State |
| --- | --- |
| Now | `phase-137x main kilo reopen selection` |
| Front | `kilo_kernel_small_hk` 再ベースライン + `kilo_micro_substring_concat` / `kilo_micro_array_getset` 再確認 |
| Blocker | `nyash_kernel` の構造分割は landed。split kernel 上で `main kilo` を reopen する |
| Next | `phase-kx vm-hako small reference interpreter recut` |
| After Next | `—` |

## Current Read

- `phase-132x` landed:
  - `--backend` default is now `mir`
  - explicit `vm` / `vm-hako` proof-debug lanes stay frozen keep
- `phase-133x` landed:
  - `kilo_micro_substring_concat`: `c_ms=3 / ny_aot_ms=3`
  - `kilo_micro_array_getset`: `c_ms=4 / ny_aot_ms=4`
  - `kilo_micro_indexof_line`: `c_ms=4 / ny_aot_ms=3`
- current work is broad optimization again, but on top of the landed structural cut
- current work is not broad `.hako` migration
- the structural cut of `crates/nyash_kernel` is landed:
  - `keep`
  - `thin keep`
  - `compat glue`
  - `substrate candidate`
- landed source slices:
  - `crates/nyash_kernel/src/exports/string.rs` split
  - `crates/nyash_kernel/src/plugin/map_substrate.rs` thin-alias recut
- next fixed corridor:
  1. `phase-137x main kilo reopen selection`
  2. `phase-kx vm-hako small reference interpreter recut`

## Successor Corridor

1. `phase-137x main kilo reopen selection`
2. `phase-kx vm-hako small reference interpreter recut`

## Parked After Optimization

- `vm-hako` small reference interpreter recut

## Structural Stop Lines

- `rust-vm`
  - mainline retirement: achieved
  - residual explicit keep: frozen
- `vm-hako`
  - reference/conformance keep
- `nyash_kernel`
  - host/kernel keep stays in Rust
  - ABI keep-thin stays in Rust
  - hot leaf / lifetime-sensitive string-array-map leaves stay in Rust until proven otherwise
  - owner/glue split happens before any broader `.hako` move

## Reference

- current lane docs:
  - `docs/development/current/main/phases/phase-137x/README.md`
  - `docs/development/current/main/phases/phase-134x/README.md`
  - `docs/development/current/main/phases/phase-133x/README.md`
  - `docs/development/current/main/phases/phase-132x/README.md`
