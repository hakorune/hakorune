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
| Now | `phase-134x nyash_kernel layer recut selection` |
| Front | `exports/string.rs` split inventory + `plugin/map_substrate.rs` thin-alias inventory |
| Blocker | `.hako` 移植を始める前に `ABI / glue / substrate` を Rust 側で分ける |
| Next | `phase-135x string export split` |
| After Next | `phase-136x map substrate thin-alias recut` |

## Current Read

- `phase-132x` landed:
  - `--backend` default is now `mir`
  - explicit `vm` / `vm-hako` proof-debug lanes stay frozen keep
- `phase-133x` landed:
  - `kilo_micro_substring_concat`: `c_ms=3 / ny_aot_ms=3`
  - `kilo_micro_array_getset`: `c_ms=4 / ny_aot_ms=4`
  - `kilo_micro_indexof_line`: `c_ms=4 / ny_aot_ms=3`
- current work is not broad optimization yet
- current work is not broad `.hako` migration either
- current work is a structural cut of `crates/nyash_kernel` into:
  - `keep`
  - `thin keep`
  - `compat glue`
  - `substrate candidate`
- first exact slices:
  - `crates/nyash_kernel/src/exports/string.rs`
  - `crates/nyash_kernel/src/plugin/map_substrate.rs`
- next fixed corridor:
  1. `phase-135x string export split`
  2. `phase-136x map substrate thin-alias recut`
  3. `phase-137x main kilo reopen selection`

## Successor Corridor

1. `phase-135x string export split`
2. `phase-136x map substrate thin-alias recut`
3. `phase-137x main kilo reopen selection`
4. `phase-kx vm-hako small reference interpreter recut`

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
  - `docs/development/current/main/phases/phase-134x/README.md`
  - `docs/development/current/main/phases/phase-133x/README.md`
  - `docs/development/current/main/phases/phase-132x/README.md`
