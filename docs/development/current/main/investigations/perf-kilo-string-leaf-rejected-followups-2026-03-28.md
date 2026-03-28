---
Status: investigation
Date: 2026-03-28
Scope: `perf-kilo` current wave で rejected にした string-leaf follow-up を、shell history ではなく docs に固定する
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/design/transient-text-pieces-ssot.md
  - crates/nyash_kernel/src/exports/string.rs
  - crates/nyash_kernel/src/exports/string_view.rs
  - crates/nyash_kernel/src/plugin/array_string_slot.rs
  - crates/nyash_kernel/src/plugin/value_codec/string_store.rs
---

# Perf Kilo String Leaf Rejected Follow-ups (2026-03-28)

## Goal

2026-03-28 の `perf-kilo` wave で試したが keep しなかった cut を固定する。

目的は 2 つだけ。

1. 同じ外れを次 wave で繰り返さない。
2. `current exact leaf` を 다시 `concat_hs` / registry helper density へ戻す。

## Rejected Cut A

### Name

direct `concat_hs` / `concat3` copy materialization

### Intent

- `TextPlan::from_two/from_three(...).into_owned()` をやめて
- `concat_two_str(...)` / `concat_three_str(...)` の direct copy へ寄せる
- `concat_pair_*` / `concat3_plan_from_*` / `concat_const_suffix_from_handle(...)` を straight-line にする

### Result

- stable `kilo_kernel_small_hk`: `736 ms -> 757 ms`
- micro:
  - `kilo_micro_indexof_line = 7 ms`
  - `kilo_micro_substring_concat = 4 ms`
  - `kilo_micro_array_getset = 4 ms`

### Judgment

reject

### Why

- whole-program stable が悪化した
- micro でも meaningful な改善が出なかった
- `concat_hs` hot path の本丸は direct copy ではなく、bridge/registry/helper density の可能性が高い

### Reopen Condition

- fresh asm で `TextPlan` flatten / `into_owned()` 自体が top reason と確認できた時だけ

## Rejected Cut B

### Name

piece-preserving `insert_inline` plus store/freeze reshaping

### Intent

- `insert_inline(...)` を span/piece のまま保持する
- `array_set_by_index_string_handle_value(...)` と `string_store` の freeze/store boundary を詰める
- intermediate `owned String` birth を further reduce する

### Result

- stable `kilo_kernel_small_hk`: `895 ms`
- micro も揺れて keep 理由なし

### Judgment

reject

### Why

- `insert_hsi` 自体は main top で `0.6-0.7%` 程度しかなく、active hot leaf を読み違えた
- store/freeze reshaping は branch density を増やしたが、mainline cost を十分に回収できなかった

### Reopen Condition

- `concat_hs` / `array_set_by_index_string_handle_value` が top から退いた後
- かつ `insert_hsi` / transient carrier が asm top reason に上がった後

## Active Exact Leaf After Rejection

次に reopen する leaf はこれだけだよ。

- `nyash.string.concat_hs`
- `nyash_kernel::exports::string_view::resolve_string_span_pair_from_handles`
- `nyash_rust::runtime::host_handles::with_str_pair`
- `nyash_kernel::plugin::array_string_slot::array_set_by_index_string_handle_value`

## Rejected Cut C

### Name

blanket `#[inline(always)]` on host registry / hako-forward string wrappers

### Intent

- force inlining on `host_handles::{get,with_handle,get_pair,with_pair,with_str_pair,...}`
- force inlining on `hako_forward_bridge::call_string_dispatch(...)`
- reduce helper call density around the existing `concat_hs` fast path without changing carrier shape

### Result

- stable `kilo_kernel_small_hk`: around `740 ms`
- did not beat the current `736 ms` line

### Judgment

reject

### Why

- no measurable whole-program win
- the broad blanket changed many helper bodies at once, so the cut was not narrow enough to justify staying live without a clear improvement

### Reopen Condition

- only if a future asm read shows wrapper call/ret density dominating over span resolution or alloc/copy cost

## Rejected Cut D

### Name

`concat_hs` duplicate span-resolution removal plus span-resolver inlining

### Intent

- collapse `TextPlan::from_handle(...)` plus the following `resolve_string_span_from_handle(...)` branch into a single span lookup
- inline `resolve_string_span_from_handle(...)` and `resolve_string_span_pair_from_handles(...)`
- shrink the hot `concat_hs` path without touching fallback policy or carrier structure

### Result

- stable `kilo_kernel_small_hk`: `796 ms`
- micro stayed green but provided no compensating win

### Judgment

reject

### Why

- `concat_hs` got slower in stable whole-program reading
- the existing `TextPlan::from_handle(...)` route, while redundant-looking in source, was not the real cost center on this machine

### Reopen Condition

- only if future asm shows duplicated span lookup itself dominating after `with_str_pair` / pair-span resolution are exhausted

## Operational Rule

- 1 cut = 1 hot leaf に戻す
- carrier redesign と leaf direct-copy cut を混ぜない
- stable `kilo_kernel_small_hk` が悪化したら、その場で revert する
