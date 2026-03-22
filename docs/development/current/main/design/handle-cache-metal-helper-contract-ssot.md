---
Status: SSOT
Decision: provisional
Date: 2026-03-23
Scope: `phase-29ct` の metal helper contract lock として、`handle_cache.rs` の責務・不変条件・非目標を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29ct/README.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - crates/nyash_kernel/src/plugin/handle_cache.rs
  - crates/nyash_kernel/src/plugin/runtime_data.rs
  - crates/nyash_kernel/src/plugin/array_slot_load.rs
  - crates/nyash_kernel/src/plugin/array_slot_store.rs
  - crates/nyash_kernel/src/plugin/map_slot_load.rs
  - crates/nyash_kernel/src/plugin/map_slot_store.rs
  - crates/nyash_kernel/src/plugin/map_probe.rs
---

# Handle Cache Metal Helper Contract (SSOT)

## Goal

- `handle_cache.rs` を `.hako` owner や ABI manifest truth と混同せず、thin metal helper として固定する。
- typed handle cache / typed dispatch / array i64 re-encode helper の責務境界を 1 枚で読む。
- future `hako.ptr` / `hako.tls` widening の前に、native keep helper の最小 contract を lock する。

## Role

`handle_cache.rs` は次の3責務だけを持つ。

1. typed handle cache
   - `handle -> Arc<dyn NyashBox>` の短寿命 TLS cache
   - `drop_epoch` で stale entry を無効化する
2. typed dispatch helper
   - `with_array_box`
   - `with_map_box`
   - `with_instance_box`
   - `with_array_or_map`
3. array i64 encoding helper
   - `array_get_index_encoded_i64`
   - `encode_array_item_to_i64`

## Current Call Surface

current direct callers は次で固定する。

- `array.rs`
- `array_slot_append.rs`
- `array_slot_load.rs`
- `array_slot_store.rs`
- `map.rs`
- `map_probe.rs`
- `map_slot_load.rs`
- `map_slot_store.rs`
- `runtime_data.rs`

この helper は上記 plugin substrate の内側だけで使う。

## Invariants

1. `handle <= 0` は即 `None`
2. cache entry は `handle` と `drop_epoch` が両方一致したときだけ再利用する
3. cache hit でも downcast/type check は省略しない
4. `with_array_box` / `with_map_box` / `with_instance_box` は型不一致なら `None`
5. `with_array_or_map` は `ArrayBox` / `MapBox` 以外を受け付けない
6. `array_get_index_encoded_i64` は `idx < 0` を拒否する
7. `encode_array_item_to_i64` の canonical order は次
   - `as_i64_fast`
   - `as_bool_fast`
   - borrowed-handle fast path
   - `runtime_i64_from_box_ref`
8. borrowed string alias validity は `value_codec` 側の `drop_epoch` 契約に従う

## Failure Contract

- invalid handle
  - `None`
- stale cache entry
  - cache miss として再取得
- type mismatch
  - `None`
- invalid array index
  - `None`

ここでは fail-fast panic ではなく、plugin substrate 向け fail-safe return を維持する。

## Non-Goals

- ABI manifest truth を持つこと
- value representation policy owner になること
- `.hako` policy owner をここへ移すこと
- generic object registry に広げること
- array/map algorithm policy を決めること
- cache internals を public API 化すること

## Native Keep Reading

`handle_cache.rs` は current lane では native keep helper として読む。

- policy owner: no
- semantic owner: no
- metal service provider: yes

この helper は future `hako.ptr` / `hako.tls` へそのまま移す対象ではなく、先に contract を固定してから capability widening の input にする。

## Acceptance

minimum acceptance は次で固定する。

- `cargo test -q -p nyash_kernel cache_invalidates_on_drop_epoch_when_handle_is_reused --lib`
- `cargo test -q -p nyash_kernel invalid_handle_short_circuits_all_routes --lib`
- `cargo test -q -p nyash_kernel array_get_index_fail_safe_contract --lib`
- `cargo check -q --lib`

