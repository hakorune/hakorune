---
Status: SSOT
Date: 2026-04-06
Scope: current lane / blocker / next pointer だけを置く薄い mirror。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
---

# Self Current Task — Now (main)

## Current

- lane: `phase-149x concat const-suffix vertical slice`
- current front: `const_suffix` route の first contract-first consumer を実装する
- blocker: current concrete helper `nyash.string.concat_hs` を authority ではなく executor detail に押し込むこと
- recent landed:
  - `phase-140x map owner pilot`
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
- landed final string seam:
  - semantic owner: `runtime/kernel/string/**`
  - VM-facing wrapper: `string_core_box.hako`
  - thin facade: `string.rs`
  - lifetime/native substrate: `string_view.rs` / `string_helpers.rs` / `string_plan.rs`
  - quarantine: `module_string_dispatch/**`
- current architecture follow-up is implementation-first:
  - `phase-142x` = landed Array owner cutover implementation
  - `phase-143x` = landed Map owner cutover implementation
  - `phase-144x` = landed String semantic owner follow-up
- current cleanup lane:
  - `phase-145x` = landed compat quarantine shrink
  - `phase-146x` = landed string semantic boundary tighten
- current optimization authority lock:
  - `.hako` owns route / retained-form / boundary
  - MIR owns canonical substrate contract
  - Rust owns executor / accelerator only
- landed contract freeze:
  - `const_suffix -> thaw.str + lit.str + str.concat2 + freeze.str`
  - `ArrayStoreString -> store.array.str`
  - `MapStoreAny -> store.map.value`
- perf lane is paused, not cancelled:
  - `phase-137x main kilo reopen selection` is the first consumer after contract freeze
  - current perf truth:
    - baseline `1529ms`
    - after string const fast path `775ms`
    - after const-handle cache follow-up `731ms`
    - after const empty-flag cache `723ms`
    - after shared text-based const-handle helper `903ms`
    - after single-closure const suffix fast path `820ms`
    - exact micro `concat_const_suffix` `85ms`
    - exact micro `array_string_store` `217ms`

## Root Anchors

- root anchor: `CURRENT_TASK.md`
- quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- one-screen map: `docs/development/current/main/15-Workstream-Map.md`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/design/semantic-optimization-authority-ssot.md`
3. `docs/development/current/main/phases/phase-149x/README.md`
4. `docs/development/current/main/phases/phase-137x/README.md`
5. `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
