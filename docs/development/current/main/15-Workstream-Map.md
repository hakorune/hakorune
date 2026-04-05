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
| Now | `phase-139x array owner pilot` |
| Front | `ArrayCoreBox` / `ArrayStateCoreBox` を visible owner に固定し、Rust を ABI facade / substrate / accelerators に保つ |
| Blocker | final owner graph は fixed。次は Array owner の exact seam と compat/runtime forwarding の shrink line を source-backed に決める |
| Next | `phase-140x map owner pilot` |
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
- `phase-138x` landed the final owner graph:
  - `Rust host microkernel`
  - `.hako semantic kernel`
  - `native accelerators`
  - `ABI facade` thin keep
  - `compat quarantine` non-owner
- landed source slices:
  - `crates/nyash_kernel/src/exports/string.rs` split
  - `crates/nyash_kernel/src/plugin/map_substrate.rs` thin-alias recut
- current work is the first narrow `.hako` owner cut:
  - `.hako` owner = `array_core_box.hako` / `array_state_core_box.hako`
  - substrate below = `raw_array_core_box.hako` / `ptr_core_box.hako`
  - Rust ABI facade = `array_substrate.rs`
  - Rust compat/runtime forwarding = `array_runtime_facade.rs`
  - Rust accelerators = `array_handle_cache.rs` / `array_string_slot.rs`
- next fixed corridor:
  1. `phase-139x array owner pilot`
  2. `phase-140x map owner pilot`
  3. `phase-141x string semantic boundary review`
  4. `phase-137x main kilo reopen selection`
  5. `phase-kx vm-hako small reference interpreter recut`

## Successor Corridor

1. `phase-139x array owner pilot`
2. `phase-140x map owner pilot`
3. `phase-141x string semantic boundary review`
4. `phase-137x main kilo reopen selection`
5. `phase-kx vm-hako small reference interpreter recut`

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
  - `docs/development/current/main/phases/phase-139x/README.md`
  - `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
  - `docs/development/current/main/phases/phase-138x/README.md`
  - `docs/development/current/main/phases/phase-137x/README.md`
  - `docs/development/current/main/phases/phase-134x/README.md`
  - `docs/development/current/main/phases/phase-133x/README.md`
  - `docs/development/current/main/phases/phase-132x/README.md`
