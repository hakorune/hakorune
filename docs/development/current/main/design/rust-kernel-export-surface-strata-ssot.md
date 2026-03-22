# Rust Kernel Export Surface Strata SSOT

Status: provisional
Date: 2026-03-23
Scope: `crates/nyash_kernel/src/plugin/{array,map,runtime_data}.rs` の Rust-side export surface を、ABI 名は変えずに contract strata ごとへ薄く分ける。

## Purpose

- Rust kernel の export surface を、compat / runtime-facade / substrate に分けて読みやすくする。
- `.hako` owner 側の visible semantics や ABI symbol 名は変えない。
- `array.rs` / `map.rs` は thin facade として残し、実装は sibling module に退がせる。

## Current Shape

- `array.rs`
  - thin facade / test host
  - re-export:
    - `array_compat.rs`
    - `array_runtime_facade.rs`
    - `array_substrate.rs`
- `map.rs`
  - thin facade / test host
  - re-export:
    - `map_compat.rs`
    - `map_substrate.rs`
- `runtime_data.rs`
  - separate thin facade のまま維持

## Contract Strata

### compat

- legacy / historical symbols
- target:
  - `nyash.array.get_h`
  - `nyash.array.set_h`
  - `nyash.array.push_h`
  - `nyash.array.len_h`
  - `nyash.map.get_h`
  - `nyash.map.set_h`
  - `nyash.map.has_h`
  - `nyash.map.size_h`

### runtime-facade

- runtime-data style / proven key-shape routes
- target:
  - `nyash.array.get_hh`
  - `nyash.array.set_hhh`
  - `nyash.array.has_hh`
  - `nyash.array.push_hh`
  - `nyash.array.push_hi`
  - `nyash.array.get_hi`
  - `nyash.array.set_hih`
  - `nyash.array.set_hii`
  - `nyash.array.set_his`
  - `nyash.array.has_hi`

### substrate

- mainline daily raw seams
- target:
  - `nyash.array.slot_load_hi`
  - `nyash.array.slot_store_hii`
  - `nyash.array.slot_append_hh`
  - `nyash.array.slot_len_h`
  - `nyash.array.slot_cap_h`
  - `nyash.array.slot_reserve_hi`
  - `nyash.array.slot_grow_hi`
  - `nyash.map.entry_count_h`
  - `nyash.map.slot_load_hi`
  - `nyash.map.slot_load_hh`
  - `nyash.map.slot_store_hih`
  - `nyash.map.slot_store_hhh`
  - `nyash.map.probe_hi`
  - `nyash.map.probe_hh`

## Non-goals

- rename ABI symbols
- change `.hako` owner behavior
- move `handle_cache.rs`
- move `value_codec/*`
- change value representation classes
- add new package-manager or distribution semantics

## Acceptance

- `cargo check -q --lib`
- `git diff --check`
- existing array/map route contract checks stay green
- docs point to the new sibling-module layout instead of pretending `array.rs` / `map.rs` are monolithic
