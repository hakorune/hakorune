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
| Now | `phase-138x nyash_kernel semantic owner cutover` |
| Front | `Rust host microkernel` / `.hako semantic kernel` / `native accelerators` の最終 owner model を固定する |
| Blocker | `nyash_kernel` の4層 split は landed。次に reopen する前に semantic ownership の stop-line を current SSOT に落とす |
| Next | `phase-139x array owner pilot` |
| After Next | `phase-137x main kilo reopen selection` |

## Current Read

- `phase-132x` landed:
  - `--backend` default is now `mir`
  - explicit `vm` / `vm-hako` proof-debug lanes stay frozen keep
- `phase-133x` landed:
  - `kilo_micro_substring_concat`: `c_ms=3 / ny_aot_ms=3`
  - `kilo_micro_array_getset`: `c_ms=4 / ny_aot_ms=4`
  - `kilo_micro_indexof_line`: `c_ms=4 / ny_aot_ms=3`
- the structural cut of `crates/nyash_kernel` is landed:
  - `keep`
  - `thin keep`
  - `compat glue`
  - `substrate candidate`
- landed source slices:
  - `crates/nyash_kernel/src/exports/string.rs` split
  - `crates/nyash_kernel/src/plugin/map_substrate.rs` thin-alias recut
- current work is not broad `.hako` migration:
  - first lock the permanent owner graph
  - then move `Array owner`
  - then move `Map owner`
- final architecture reading:
  - `Rust host microkernel`
  - `.hako semantic kernel`
  - `native accelerators`
  - `ABI facade` thin keep
  - `compat quarantine` non-owner
- next fixed corridor:
  1. `phase-138x nyash_kernel semantic owner cutover`
  2. `phase-139x array owner pilot`
  3. `phase-140x map owner pilot`
  4. `phase-141x string semantic boundary review`
  5. `phase-137x main kilo reopen selection`
  6. `phase-kx vm-hako small reference interpreter recut`

## Successor Corridor

1. `phase-138x nyash_kernel semantic owner cutover`
2. `phase-139x array owner pilot`
3. `phase-140x map owner pilot`
4. `phase-141x string semantic boundary review`
5. `phase-137x main kilo reopen selection`
6. `phase-kx vm-hako small reference interpreter recut`

## Parked After Optimization

- `vm-hako` small reference interpreter recut

## Structural Stop Lines

- `rust-vm`
  - mainline retirement: achieved
  - residual explicit keep: frozen
- `vm-hako`
  - reference/conformance keep
- `nyash_kernel`
  - `Rust host microkernel` stays in Rust
  - `ABI facade` stays thin keep in Rust
  - lifetime-sensitive hot leaves and native accelerators stay in Rust until proven otherwise
  - semantic ownership moves toward `.hako`
  - compat quarantine must not become a permanent owner layer

## Reference

- current lane docs:
  - `docs/development/current/main/phases/phase-138x/README.md`
  - `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
  - `docs/development/current/main/phases/phase-137x/README.md`
  - `docs/development/current/main/phases/phase-134x/README.md`
  - `docs/development/current/main/phases/phase-133x/README.md`
  - `docs/development/current/main/phases/phase-132x/README.md`
