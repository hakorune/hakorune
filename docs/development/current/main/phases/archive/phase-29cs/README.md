---
Status: Parked
Decision: accepted
Date: 2026-03-22
Scope: kernel / plugin naming cleanup lane。`helpers` / `route` / `rust` の transitional naming を責務名へ寄せる。semantic owner cutover や broad package rename は扱わない。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/repo-physical-structure-cleanup-ssot.md
  - docs/development/current/main/design/collection-raw-substrate-contract-ssot.md
  - docs/reference/runtime/runtime-data-dispatch.md
  - crates/nyash_kernel/README.md
  - lang/src/runtime/collections/README.md
---

# Phase 29cs: Kernel Naming Cleanup

## Goal

- Transitional naming を責務名へ寄せる。
- `helpers` / `route` / `rust` を責務名で置き換え、将来の multi-language kernel 化でも読みやすい箱にする。
- kernel/plugin の public surface は変えず、名前の層だけを整える。
- This lane is complete and parked after the rename batch landed.

## Non-Goals

- `.hako` kernel authority migration の semantics 変更
- broad `nyash-rust` package rename
- `array.rs` / `map.rs` / `runtime_data.rs` の public surface restructuring
- ABI / fallback contract の意味変更

## Fixed Order

1. Docs-first inventory
   - transitional names / keep names / parked names を固定する
2. Plugin helper / route rename batch
   - `array_index_dispatch.rs`
   - `array_write_dispatch.rs`
   - `handle_cache.rs`
   - `runtime_data_array_dispatch.rs`
   - `runtime_data_map_dispatch.rs`
3. Binary alias cleanup
   - `src/bin/hakorune_compat.rs`
   - binary 名は `hakorune-compat`
   - ここは stage0 direct alias としての必要性を確認してから維持/撤去を決める
4. Docs cleanup
   - SSOT / README / phase docs を rename 後の名前へ揃える

## Recommended Naming

- `helpers` -> `dispatch` / `cache` / `raw`
- `route` -> `dispatch`
- `rust` -> responsibility name or keep as historical package label only
- `core`, `cache`, `dispatch`, `bridge`, `compat`, `raw` are preferred

## Keep As-Is

- `array_slot_load.rs`
- `array_slot_store.rs`
- `array_slot_append.rs`
- `map_slot_load.rs`
- `map_slot_store.rs`
- `map_probe.rs`
- `runtime_data.rs`
- `value_codec/`
- `module_string_dispatch.rs`

## Parking

- `nyash-rust` package name is historical and parked for now.
- `array.rs` / `map.rs` / `runtime_data.rs` remain as core public entry points.
- `abi_adapter_registry.hako` and `hako_llvmc_ffi.c` remain contract surfaces, not naming cleanup targets.
