---
Status: Active
Decision: provisional
Date: 2026-03-23
Scope: `phase-29ct` の V0 として、current kernel/plugin ABI export surface を inventory 化し、mainline substrate / runtime-facade / compat-only / adapter-default を分けて読む。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29ct/README.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - lang/src/vm/boxes/abi_adapter_registry.hako
  - crates/nyash_kernel/src/plugin/array.rs
  - crates/nyash_kernel/src/plugin/map.rs
  - crates/nyash_kernel/src/plugin/runtime_data.rs
  - crates/nyash_kernel/src/exports/birth.rs
  - crates/nyash_kernel/src/exports/string.rs
---

# ABI Export Inventory (V0)

## Goal

- current export surface を `AbiAdapterRegistryBox` の default rows と混同せず、docs-side inventory として固定する。
- current symbols を次の4種に分けて読む。
  - `mainline substrate`
  - `runtime-facade`
  - `compat-only`
  - `adapter-default consumer`
- future manifest generation の入力を 1 枚に寄せる。

## Reading Rule

- この文書は symbol inventory の正本だよ。
- [`abi_adapter_registry.hako`](/home/tomoaki/git/hakorune-selfhost/lang/src/vm/boxes/abi_adapter_registry.hako) は runtime-side consumer であって、manifest の正本ではない。
- `args` / `ret` は V0 では最小読みで固定する。
  - `handle_owned`
  - `imm_i64`
  - `value_public`
- `value_public` は V0 の umbrella class で、`imm_i64` / `imm_bool` / `handle_owned` / `handle_borrowed_string` をまとめて表す。
- ownership の厳密 lock は [`value-repr-and-abi-manifest-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md) の次 slice で固定する。

## Birth Group

| box_type | method | symbol | args | ret | status | source |
| --- | --- | --- | --- | --- | --- | --- |
| `StringBox` | `birth` | `nyash.string.birth_h` | `-` | `handle_owned` | `mainline substrate` | `crates/nyash_kernel/src/exports/birth.rs` |
| `IntegerBox` | `birth` | `nyash.integer.birth_h` | `-` | `handle_owned` | `mainline substrate` | `crates/nyash_kernel/src/exports/birth.rs` |
| `ConsoleBox` | `birth` | `nyash.console.birth_h` | `-` | `handle_owned` | `mainline substrate` | `crates/nyash_kernel/src/exports/birth.rs` |
| `ArrayBox` | `birth` | `nyash.array.birth_h` | `-` | `handle_owned` | `mainline substrate` | `crates/nyash_kernel/src/exports/birth.rs` |
| `MapBox` | `birth` | `nyash.map.birth_h` | `-` | `handle_owned` | `mainline substrate` | `crates/nyash_kernel/src/exports/birth.rs` |

## ArrayBox

| method_view | symbol | args | ret | status | source | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `slot_load` | `nyash.array.slot_load_hi` | `handle_owned, imm_i64` | `value_public` | `mainline substrate` | `crates/nyash_kernel/src/plugin/array.rs` | canonical daily `ArrayBox.get` target |
| `slot_store_i64` | `nyash.array.slot_store_hii` | `handle_owned, imm_i64, imm_i64` | `imm_i64` | `mainline substrate` | `crates/nyash_kernel/src/plugin/array.rs` | canonical `.hako` i64 store seam |
| `slot_append` | `nyash.array.slot_append_hh` | `handle_owned, value_public` | `imm_i64` | `mainline substrate` | `crates/nyash_kernel/src/plugin/array.rs` | canonical daily append seam |
| `slot_len` | `nyash.array.slot_len_h` | `handle_owned` | `imm_i64` | `mainline substrate` | `crates/nyash_kernel/src/plugin/array.rs` | canonical daily observer |
| `get` | `nyash.array.get_hh` | `handle_owned, value_public` | `value_public` | `runtime-facade` | `crates/nyash_kernel/src/plugin/array.rs` | runtime-data style dynamic key route |
| `set` | `nyash.array.set_hhh` | `handle_owned, value_public, value_public` | `imm_i64` | `runtime-facade` | `crates/nyash_kernel/src/plugin/array.rs` | runtime-data style dynamic key route |
| `has` | `nyash.array.has_hh` | `handle_owned, value_public` | `imm_i64` | `runtime-facade` | `crates/nyash_kernel/src/plugin/array.rs` | runtime-data style dynamic key route |
| `push` | `nyash.array.push_hh` | `handle_owned, value_public` | `imm_i64` | `runtime-facade` | `crates/nyash_kernel/src/plugin/array.rs` | runtime-data style append route |
| `push_i64` | `nyash.array.push_hi` | `handle_owned, imm_i64` | `imm_i64` | `runtime-facade` | `crates/nyash_kernel/src/plugin/array.rs` | proven integer append route |
| `get_i64` | `nyash.array.get_hi` | `handle_owned, imm_i64` | `value_public` | `runtime-facade` | `crates/nyash_kernel/src/plugin/array.rs` | proven integer-key route |
| `set_i64_any` | `nyash.array.set_hih` | `handle_owned, imm_i64, value_public` | `imm_i64` | `runtime-facade` | `crates/nyash_kernel/src/plugin/array.rs` | accepted keep |
| `set_i64_i64` | `nyash.array.set_hii` | `handle_owned, imm_i64, imm_i64` | `imm_i64` | `runtime-facade` | `crates/nyash_kernel/src/plugin/array.rs` | accepted keep |
| `set_i64_string_handle` | `nyash.array.set_his` | `handle_owned, imm_i64, handle_owned` | `imm_i64` | `runtime-facade` | `crates/nyash_kernel/src/plugin/array.rs` | string-handle specialized route |
| `has_i64` | `nyash.array.has_hi` | `handle_owned, imm_i64` | `imm_i64` | `runtime-facade` | `crates/nyash_kernel/src/plugin/array.rs` | proven integer-key route |
| `get` | `nyash.array.get_h` | `handle_owned, imm_i64` | `value_public` | `compat-only` | `crates/nyash_kernel/src/plugin/array.rs` | historical compat alias |
| `set` | `nyash.array.set_h` | `handle_owned, imm_i64, imm_i64` | `imm_i64` | `compat-only` | `crates/nyash_kernel/src/plugin/array.rs` | legacy return is fixed `0` |
| `push` | `nyash.array.push_h` | `handle_owned, imm_i64` | `imm_i64` | `compat-only` | `crates/nyash_kernel/src/plugin/array.rs` | historical compat alias |
| `len` | `nyash.array.len_h` | `handle_owned` | `imm_i64` | `compat-only` | `crates/nyash_kernel/src/plugin/array.rs` | old observer alias |

## MapBox

| method_view | symbol | args | ret | status | source | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `entry_count` | `nyash.map.entry_count_h` | `handle_owned` | `imm_i64` | `mainline substrate` | `crates/nyash_kernel/src/plugin/map.rs` | canonical daily size observer |
| `slot_load_i64` | `nyash.map.slot_load_hi` | `handle_owned, imm_i64` | `value_public` | `mainline substrate` | `crates/nyash_kernel/src/plugin/map.rs` | raw i64 key seam |
| `slot_load_any` | `nyash.map.slot_load_hh` | `handle_owned, value_public` | `value_public` | `mainline substrate` | `crates/nyash_kernel/src/plugin/map.rs` | canonical daily `MapBox.get` target |
| `slot_store_i64_any` | `nyash.map.slot_store_hih` | `handle_owned, imm_i64, value_public` | `imm_i64` | `mainline substrate` | `crates/nyash_kernel/src/plugin/map.rs` | raw i64 key seam |
| `slot_store_any` | `nyash.map.slot_store_hhh` | `handle_owned, value_public, value_public` | `imm_i64` | `mainline substrate` | `crates/nyash_kernel/src/plugin/map.rs` | canonical daily `MapBox.set` target |
| `probe_i64` | `nyash.map.probe_hi` | `handle_owned, imm_i64` | `imm_i64` | `mainline substrate` | `crates/nyash_kernel/src/plugin/map.rs` | raw i64 key seam |
| `probe_any` | `nyash.map.probe_hh` | `handle_owned, value_public` | `imm_i64` | `mainline substrate` | `crates/nyash_kernel/src/plugin/map.rs` | canonical daily `MapBox.has` target |
| `size` | `nyash.map.size_h` | `handle_owned` | `imm_i64` | `compat-only` | `crates/nyash_kernel/src/plugin/map.rs` | canonical observer is `entry_count_h` |
| `get_i64` | `nyash.map.get_h` | `handle_owned, imm_i64` | `value_public` | `compat-only` | `crates/nyash_kernel/src/plugin/map.rs` | historical compat alias |
| `get_any` | `nyash.map.get_hh` | `handle_owned, value_public` | `value_public` | `compat-only` | `crates/nyash_kernel/src/plugin/map.rs` | historical compat alias |
| `set_i64` | `nyash.map.set_h` | `handle_owned, imm_i64, value_public` | `imm_i64` | `compat-only` | `crates/nyash_kernel/src/plugin/map.rs` | legacy return is fixed `0` |
| `set_any` | `nyash.map.set_hh` | `handle_owned, value_public, value_public` | `imm_i64` | `compat-only` | `crates/nyash_kernel/src/plugin/map.rs` | legacy return is fixed `0` |
| `has_i64` | `nyash.map.has_h` | `handle_owned, imm_i64` | `imm_i64` | `compat-only` | `crates/nyash_kernel/src/plugin/map.rs` | historical compat alias |
| `has_any` | `nyash.map.has_hh` | `handle_owned, value_public` | `imm_i64` | `compat-only` | `crates/nyash_kernel/src/plugin/map.rs` | historical compat alias |

## RuntimeDataBox

| method_view | symbol | args | ret | status | source | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `get` | `nyash.runtime_data.get_hh` | `handle_owned, value_public` | `value_public` | `runtime-facade` | `crates/nyash_kernel/src/plugin/runtime_data.rs` | not a raw substrate family |
| `set` | `nyash.runtime_data.set_hhh` | `handle_owned, value_public, value_public` | `imm_i64` | `runtime-facade` | `crates/nyash_kernel/src/plugin/runtime_data.rs` | dynamic array/map dispatch |
| `has` | `nyash.runtime_data.has_hh` | `handle_owned, value_public` | `imm_i64` | `runtime-facade` | `crates/nyash_kernel/src/plugin/runtime_data.rs` | dynamic array/map dispatch |
| `push` | `nyash.runtime_data.push_hh` | `handle_owned, value_public` | `imm_i64` | `runtime-facade` | `crates/nyash_kernel/src/plugin/runtime_data.rs` | array-only append through facade |

## StringBox

| method_view | symbol | args | ret | status | source | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `len` | `nyash.string.len_h` | `handle_owned` | `imm_i64` | `mainline substrate` | `crates/nyash_kernel/src/exports/string.rs` | canonical StringBox observer |

## Adapter Defaults (Consumer Rows)

[`AbiAdapterRegistryBox`](/home/tomoaki/git/hakorune-selfhost/lang/src/vm/boxes/abi_adapter_registry.hako) の defaults は distinct export rows ではなく、current symbol inventory を消費する adapter rows として読む。

| box_type | method | symbol | status | notes |
| --- | --- | --- | --- | --- |
| `MapBox` | `birth` | `nyash.map.birth_h` | `adapter-default consumer` | mainline birth row を参照 |
| `MapBox` | `set` | `nyash.map.slot_store_hhh` | `adapter-default consumer` | mainline raw seam を参照 |
| `MapBox` | `get` | `nyash.map.slot_load_hh` | `adapter-default consumer` | current default unbox is `integer` |
| `MapBox` | `has` | `nyash.map.probe_hh` | `adapter-default consumer` | mainline raw seam を参照 |
| `MapBox` | `size` / `len` | `nyash.map.entry_count_h` | `adapter-default consumer` | canonical observer を参照 |
| `ArrayBox` | `birth` | `nyash.array.birth_h` | `adapter-default consumer` | mainline birth row を参照 |
| `ArrayBox` | `push` | `nyash.array.slot_append_hh` | `adapter-default consumer` | canonical append seam を参照 |
| `ArrayBox` | `len` / `length` / `size` | `nyash.array.slot_len_h` | `adapter-default consumer` | canonical observer を参照 |
| `ArrayBox` | `get` | `nyash.array.slot_load_hi` | `adapter-default consumer` | canonical daily get seam を参照 |
| `ArrayBox` | `set` | `nyash.array.set_hih` | `adapter-default consumer` | accepted keep/fallback route を参照 |
| `StringBox` | `len` / `length` / `size` | `nyash.string.len_h` | `adapter-default consumer` | canonical StringBox observer を参照 |

## Out Of Scope For This V0 Slice

- `nyash.string.charCodeAt_h`
- `nyash.string.concat_hh`
- `nyash.string.concat3_hhh`
- `nyash.string.eq_hh`
- `nyash.string.substring_hii`
- `nyash.string.indexOf_hh`
- `nyash.string.lastIndexOf_hh`
- `nyash.string.lt_hh`
- `nyash.string.from_u64x2`
- `nyrt_string_length`

これらは string export family / neutral bridge alias として別 lane で inventory してよいが、current `phase-29ct` collection capability slice には混ぜない。
