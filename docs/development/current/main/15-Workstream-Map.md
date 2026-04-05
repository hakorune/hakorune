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
| Front | split kernel 上の `kilo_kernel_small_hk` を再ベースラインして next hot leaf を pin する |
| Blocker | string const-path と array string-store path の優先順位を bundle/asm で再確定する |
| Next | `phase-kx vm-hako small reference interpreter recut` |
| After Next | parked optimization follow-ups |

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
- `phase-139x` landed the first pilot:
  - `.hako` owner = `array_core_box.hako` / `array_state_core_box.hako`
  - substrate below = `raw_array_core_box.hako` / `ptr_core_box.hako`
  - Rust ABI facade = `array_substrate.rs`
  - Rust compat/runtime forwarding = `array_runtime_facade.rs`
  - Rust accelerators = `array_handle_cache.rs` / `array_string_slot.rs`
- current implementation corridor:
  - `phase-142x` = landed Array owner cutover implementation
  - `phase-143x` = landed Map owner cutover implementation
  - `phase-144x` = landed String semantic owner follow-up
  - `phase-145x` = landed compat quarantine shrink
  - `phase-146x` = landed string semantic boundary tighten
- `phase-140x` landed the second pilot:
  - `.hako` owner = `map_core_box.hako` / `map_state_core_box.hako`
  - substrate below = `raw_map_core_box.hako`
  - Rust thin facade = `map_aliases.rs`
  - Rust observer shim = `map_substrate.rs`
  - Rust compat/runtime forwarding = `map_runtime_facade.rs`
  - Rust accelerators = `map_probe.rs` / `map_slot_load.rs` / `map_slot_store.rs`
- landed source slices:
  - `crates/nyash_kernel/src/exports/string.rs` split
  - `crates/nyash_kernel/src/plugin/map_substrate.rs` thin-alias recut
- `phase-141x` landed the final string boundary review:
  - `.hako` semantic owner = `runtime/kernel/string/**`
  - VM-facing wrapper = `string_core_box.hako`
  - Rust thin facade = `string.rs`
  - Rust lifetime/native substrate = `string_view.rs` / `string_helpers.rs` / `string_plan.rs`
  - `module_string_dispatch/**` stays quarantine, not owner
- next fixed corridor:
  1. `phase-137x main kilo reopen selection`
  2. `phase-kx vm-hako small reference interpreter recut`
- current reopen read:
  - baseline: `kilo_kernel_small_hk = 1529ms`
  - string const fast-path: `905ms`
  - const-handle cache follow-up: `731ms`
  - const empty-flag cache: `723ms`
  - shared text-based const-handle helper: `903ms`
  - single-closure const suffix fast path: `820ms`
  - latest sampled whole-kilo reread: `905ms`
  - first leaf: `crates/nyash_kernel/src/exports/string_helpers.rs::concat_const_suffix_fallback`
  - second leaf: `crates/nyash_kernel/src/plugin/array_string_slot.rs::array_string_store_handle_at`
  - exact micro:
    - `kilo_micro_concat_const_suffix = 85ms`
    - `kilo_micro_array_string_store = 217ms`

## Successor Corridor

1. `phase-kx vm-hako small reference interpreter recut`

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
  - do not reopen broad perf tuning before compat/string cleanup is complete

## Reference

- current lane docs:
  - `docs/development/current/main/phases/phase-143x/README.md`
  - `docs/development/current/main/phases/phase-145x/README.md`
  - `docs/development/current/main/phases/phase-146x/README.md`
  - `docs/development/current/main/phases/phase-141x/README.md`
  - `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
  - `docs/development/current/main/phases/phase-142x/README.md`
  - `docs/development/current/main/phases/phase-140x/README.md`
  - `docs/development/current/main/phases/phase-139x/README.md`
  - `docs/development/current/main/phases/phase-137x/README.md`
  - `docs/development/current/main/phases/phase-138x/README.md`
  - `docs/development/current/main/phases/phase-134x/README.md`
  - `docs/development/current/main/phases/phase-133x/README.md`
  - `docs/development/current/main/phases/phase-132x/README.md`
