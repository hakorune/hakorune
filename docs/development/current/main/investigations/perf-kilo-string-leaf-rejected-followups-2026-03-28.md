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

## Rejected Cut E

### Name

specialized `StringBox` store leaf for `nyash.array.set_his`

### Intent

- add a monomorphic `StringBox`-only branch under `array_set_by_index_string_handle_value(...)`
- bypass the generic string-source helper for the hot store path
- specialize retarget/store into `StringBox`-only helper leaves

### Result

- `kilo_meso_substring_concat_array_set`: `66 ms -> 69 ms`
- stable `kilo_kernel_small_hk`: `708 ms -> 791 ms`

### Judgment

reject

### Why

- the `StringBox`-only split increased branch/helper density without paying back on this machine
- the kept in-place source borrow cut already captured the useful part; the extra monomorphic helper split did not improve store-boundary cost

### Reopen Condition

- only if a future asm read shows the generic `store_string_box_from_string_source(...)` / `try_retarget_borrowed_string_slot_with_source(...)` path itself dominating after the current in-place source borrow cut

## Rejected Cut F

### Name

short-slice threshold 8→7 plus `StringViewBox` borrow expansion

### Intent

- lower `SUBSTRING_VIEW_MATERIALIZE_MAX_BYTES` from `8` to `7`
- let `maybe_borrow_string_handle_with_epoch(...)` / `try_retarget_borrowed_string_slot_with_source(...)` borrow `StringViewBox` sources too
- reduce birth density by retaining the 8-byte substring halves as views

### Result

- stable `kilo_kernel_small_hk`: `825 ms`
- meso:
  - `kilo_meso_substring_concat_len = 37 ms`
  - `kilo_meso_substring_concat_array_set = 67 ms`
  - `kilo_meso_substring_concat_array_set_loopcarry = 69 ms`

### Judgment

reject

### Why

- the current bench shapes still make `<= 8 bytes` the better tradeoff for this wave
- the view-retain experiment did not unlock the hot path enough to offset extra view / borrow machinery
- the borrowed alias expansion is structurally reasonable, but it does not touch the current hot birth path enough to keep

### Reopen Condition

- only if a fresh same-artifact proof shows `<= 7` materially improves stable main without increasing `Registry::alloc/get` or `BoxBase::new` pressure
- and only if the relevant path actually exercises `StringViewBox` borrowing in the hot lane

## Operational Rule

- 1 cut = 1 hot leaf に戻す
- carrier redesign と leaf direct-copy cut を混ぜない
- stable `kilo_kernel_small_hk` が悪化したら、その場で revert する

## Rejected Cut G

### Name

borrowed triple-span miss resolution via `handles::with3(...)`

### Intent

- make `resolve_string_span_triplet_from_handles(...)` resolve triplet cache misses under one registry read lock
- locally flatten `StringViewBox` chains without falling back to `handles::get(...)` on the hot miss wave
- reduce `Registry::get` density on the `concat3` triple-span route without reopening sink-local tuning

### Result

- `kilo_meso_substring_concat_array_set`: `67 ms -> 68 ms`
- stable `kilo_kernel_small_hk`: `704 ms -> 745 ms -> 819 ms` on back-to-back `1x3` checks

### Judgment

reject

### Why

- the meso store-boundary proof stayed flat
- stable main regressed materially, so the narrower borrowed miss wave did not pay back on this machine
- the accepted lock-safe `concat3` fast path remains the kept slice; further triplet miss rewiring is not justified right now

### Reopen Condition

- only if a fresh asm read shows `resolve_string_span_triplet_from_handles(...)` miss handling dominating again after the current accepted `concat3` lock-safe path
- and only if the reopened cut proves a same-artifact improvement on both `kilo_meso_substring_concat_array_set` and `kilo_kernel_small_hk`

## Rejected Cut H

### Name

short-slice freeze direct owned materialization

### Intent

- change `BorrowedSubstringPlan` short freeze-only slices from `FreezeSpan(StringSpan)` to `FreezeOwned(String)`
- materialize short substring slices inside `borrowed_substring_plan_from_handle(...)` instead of carrying a span out to `string.rs`
- remove one `StringSpan` / `string_handle_from_span(...)` hop on the short freeze lane

### Result

- `kilo_meso_substring_concat_len`: `35 ms -> 35 ms` (flat)
- `kilo_meso_substring_concat_array_set`: `67 ms -> 68 ms`
- `kilo_meso_substring_concat_array_set_loopcarry`: `69 ms -> 66 ms`
- stable `kilo_kernel_small_hk`: `704 ms -> 866 ms`

### Judgment

reject

### Why

- the direct owned materialize did not improve the meso store boundary
- stable main regressed materially, so the short-slice freeze lane should keep the span-backed contract for now
- the extra owned materialize hop was not the real limiter on this machine

### Reopen Condition

- only if a fresh same-artifact proof shows the short freeze lane itself dominates after current accepted placement
- and only if the direct owned materialize improves both `kilo_meso_substring_concat_len` and `kilo_kernel_small_hk` on the same artifact pair
