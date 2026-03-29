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

## Rejected Cut I

### Name

concat_pair span-length emptiness check

### Intent

- change `concat_pair_from_spans(...)` to use span byte lengths instead of `as_str().is_empty()`
- avoid the extra `as_str()` read when deciding whether to reuse `a_h` or `b_h`

### Result

- stable `kilo_kernel_small_hk`: `668 ms -> 904 ms`

### Judgment

reject

### Why

- this lane does not hit the hot path cleanly enough to pay for its bookkeeping
- the regression is large enough that the span-length simplification is not justified on this machine

### Reopen Condition

- only if a fresh same-artifact proof shows `concat_pair_from_spans(...)` itself becoming a dominant hot tier symbol
- and only if the span-length emptiness check improves both meso and main on the same artifact pair

## Rejected Cut J

### Name

`StringViewBox::new(...)` stable-id derivation to avoid `BoxBase::new()`

### Intent

- replace the atomic `BoxBase::new()` inside `StringViewBox::new(...)` with a derived stable id from the source handle / span
- keep the view contract otherwise unchanged

### Result

- stable `kilo_kernel_small_hk`: `668 ms -> 814 ms` under `repeat=3`

### Judgment

reject

### Why

- the derived id did not buy back enough whole-program cost to justify the identity change
- `BoxBase::new()` is still hot in the `StringViewBox` birth path, but this rewrite was not the right fix on this machine

### Reopen Condition

- only after fresh asm evidence shows the atomic id allocation itself is the main cost and view identity can be changed without regressing main

## Rejected Cut K

### Name

`StringViewBox` borrow/retarget expansion at the store boundary

### Intent

- extend `maybe_borrow_string_handle_with_epoch(...)` and `try_retarget_borrowed_string_slot_with_source(...)` so `StringViewBox` is treated like `StringBox`
- keep the consumer boundary otherwise unchanged

### Result

- stable `kilo_kernel_small_hk`: `814 ms -> 844 ms` under `repeat=3`

### Judgment

reject

### Why

- the widened borrow/retarget lane did not pay back on this machine
- the consumer boundary already recognized `StringViewBox` as string-like, but extending the aliasing path did not reduce the hot birth / handle density enough

### Reopen Condition

- only after fresh asm evidence shows the current StringBox-only borrow/retarget lane itself is the dominant cost

## Rejected Cut L

### Name

ArrayBox / RuntimeDataBox string-pointer store boundary route (`nyash.array.set_his_p`)

### Intent

- keep the generic first Store boundary and let LLVM-Py pass string pointer carriers directly into the array store helper
- add `nyash.array.set_his_p` on the Rust side and route ArrayBox / RuntimeDataBox `set` calls to it when the value VID is known string-pointer-backed
- reduce the `array_set_by_index_string_handle_value(...)` hop by letting the caller hand the raw `i8*` carrier down

### Result

- same-artifact meso recheck stayed flat-to-worse: `kilo_meso_substring_concat_len = 35 ms`, `kilo_meso_substring_concat_array_set = 69 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 73 ms`
- stable main stayed worse than the kept line: `kilo_kernel_small_hk = 720 ms`
- ASM probe still showed the existing `Registry::get` / `Registry::alloc` / `BoxBase::new` / `array_set_his` tiers, but not the new `nyash.array.set_his_p` symbol in the hot set

### Judgment

reject

### Why

- the route is functionally correct and unit-tested, but it did not become part of the hot benchmark path on this machine
- the benchmark's remaining cost is still upstream birth / handle motion, not this late array-store carrier hop
- keeping this slice live would add complexity without a corresponding perf win

### Reopen Condition

- only if a fresh ASM probe shows the `set_his_p` alias on the hot path
- and only if the same-artifact meso/main pair shows a measurable gain over the kept line

## Rejected Cut M

### Name

C-side direct-store consumer widening with trailing `length()` observer

### Intent

- widen the C shim's direct-store consumer test so `array.set(row, out)` followed by one trailing `out.length()` observer still counts as the same direct store window
- keep the concat lowering in the store-oriented recipe even when a read-only observer remains

### Result

- `kilo_meso_substring_concat_len = 36 ms`
- `kilo_meso_substring_concat_array_set = 70 ms`
- `kilo_meso_substring_concat_array_set_loopcarry = 70 ms`
- stable `kilo_kernel_small_hk = 706 ms` (`repeat=3`)

### Judgment

reject

### Why

- the widened guard did not beat the kept concat3 reuse-only line
- the direct-store observer window is structurally attractive, but it did not move the same-artifact lane enough on this machine

### Reopen Condition

- only if a future placement wave proves the trailing `length()` observer is the last reason the direct store recipe cannot stay live

## Rejected Cut N

### Name

`insert_hsi` one-resolve helper

### Intent

- collapse the `insert_hsi` fast-path decision into one helper so source-empty / const-middle / freeze outcomes are chosen with a single resolve path
- remove duplicate small lookups before `freeze_text_plan(...)`

### Result

- promising first probe: `kilo_kernel_small_hk = 694 ms` (`repeat=3`)
- WSL recheck: `kilo_kernel_small_hk = 727 ms` (`repeat=20`)

### Judgment

reject

### Why

- the first 3-run read looked good, but the required 20-run WSL proof did not hold
- keep/reject policy for noisy lanes now requires the stable `repeat=20` recheck, and this slice did not survive it

### Reopen Condition

- only if a future asm read shows `insert_hsi` helper decision density dominating after the current substring / concat3 observer cuts

## Rejected Cut O

### Name

birth-time `string_span_cache` seeding from `materialize_owned_string(...)`

### Intent

- cache the full `StringSpan` immediately when `materialize_owned_string(...)` births a fresh `StringBox` handle
- let the first `len/is_empty` observer after concat / insert hit the TLS span cache instead of resolving through the normal path

### Result

- promising first probe: `kilo_meso_substring_concat_len = 35 ms`, `kilo_meso_substring_concat_array_set = 69 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 71 ms`, `kilo_kernel_small_hk = 692 ms` (`repeat=3`)
- WSL recheck: `kilo_meso_substring_concat_len = 36 ms`, `kilo_meso_substring_concat_array_set = 70 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 68 ms`, `kilo_kernel_small_hk = 730 ms` (`repeat=20`)

### Judgment

reject

### Why

- the first probe looked good, but the required `repeat=20` recheck did not hold
- it also did not remove `string_len_from_handle` / `string_is_empty_from_handle` from the hot tier strongly enough to justify keeping the extra cache seed

## Rejected Cut P

### Name

compiler-side insert-recipe `string.length()` arithmetic lowering

### Intent

- recognize `left + "xx" + right` when it is the insert-shaped substring recipe
- replace the trailing `string.length()` observer with `suffix_len + const_middle_len`
- keep the optimization generic and compiler-side instead of adding another runtime helper split

### Result

- same-artifact proof: `kilo_meso_substring_concat_len = 33 ms`, `kilo_meso_substring_concat_array_set = 63 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 65 ms`
- stable main: `kilo_kernel_small_hk = 695 ms` (`repeat=3`)
- kept comparison line remains `kilo_kernel_small_hk = 668 ms`

### Judgment

reject

### Why

- the observer rewrite helped meso, but it still lost to the kept concat3 reuse-only line on main
- this wave does not reduce retained-store birth density enough; it only changes one length observer after the store boundary

### Reopen Condition

- only if a future upstream placement wave proves the `out.length()` observer is still the dominant residue after retained/store boundary restructuring

## Rejected Cut Q

### Name

combined direct-store consumer widening plus insert-recipe `string.length()` arithmetic

### Intent

- reopen the `array.set(..., out)` plus trailing `out.length()` consumer window only for the insert-shaped concat recipe
- pair that widening with the compiler-side `suffix_len + const_middle_len` arithmetic rewrite so the extra observer disappears with the wider store recipe

### Result

- same-artifact proof: `kilo_meso_substring_concat_len = 34 ms`, `kilo_meso_substring_concat_array_set = 66 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 69 ms`
- stable main: `kilo_kernel_small_hk = 732 ms` (`repeat=3`)
- kept comparison line remains `kilo_kernel_small_hk = 668 ms`

### Judgment

reject

### Why

- the combined compiler-side rewrite still loses to the kept concat3 reuse-only line on main
- reopening the widened consumer window does not buy enough retained-store birth-density reduction on this machine

### Reopen Condition

- only if a future placement wave materially changes the retained/store boundary and a fresh ASM read still shows the trailing `length()` observer as a dominant residue

## Rejected Cut R

### Name

substring planner cache-first retry

### Intent

- make `borrowed_substring_plan_from_handle(...)` consult `string_span_cache_get(...)` before `handles::with_handle(...)`
- reuse cached root spans for repeated `substring_hii` calls on the same source handle
- keep the cut generic and upstream of the sink-local `Registry::alloc/get` stop-line

### Result

- same-artifact proof: `kilo_meso_substring_concat_len = 33 ms`, `kilo_meso_substring_concat_array_set = 64 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 67 ms`
- stable main: `kilo_kernel_small_hk = 706 ms` (`repeat=3`)
- microasm: `nyash.string.substring_hii = 6.57%`, but `Registry::alloc`, `Registry::get`, `BoxBase::new`, and the store-boundary closures still dominated

### Judgment

reject

### Why

- the cache-first planner helped meso and reduced `substring_hii` share, but it still did not beat the kept `668 ms` line on main
- this wave trims one repeated lookup, but it does not remove enough retained-store birth density to matter at `kilo_kernel_small_hk` scale

### Reopen Condition

- only if a future placement wave keeps more than one substring/transient value live across the same store boundary and `substring_hii` remains a dominant residue afterward

## Rejected Cut S

### Name

`len/is_empty` observer cache-first retry

### Intent

- flip `string_len_from_handle(...)` / `string_is_empty_from_handle(...)` to consult `string_len_impl(...)` / `string_is_empty_impl(...)` before the direct `handles::with_handle(...)` fast-string path
- let repeated post-concat / post-insert observers reuse the span-cache-aware helper before falling back to the direct handle read
- keep the cut generic and localized to the observer helpers without reopening sink-local `Registry::alloc/get`

### Result

- same-artifact proof: `kilo_meso_substring_concat_len = 35 ms`, `kilo_meso_substring_concat_array_set = 68 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 71 ms`
- stable main: `kilo_kernel_small_hk = 764 ms` (`repeat=3`)
- quick asm still kept `Registry::alloc`, `Registry::get`, `array_set_his`, `substring_hii`, and `concat3_hhh` above the observer helpers

### Judgment

reject

### Why

- the helper-order flip did not remove enough retained/store-boundary work to matter on main
- meso stayed near the existing line, but stable main regressed badly enough that the cut is not keepable on this machine
- this remains an observer-local tweak; it does not reduce birth density or boundary crossings in the way the current upstream placement lane needs

### Reopen Condition

- only if a future retained/store-boundary placement wave keeps more than one observer on the same transient carrier and a fresh asm read still shows `string_len_from_handle(...)` / `string_is_empty_from_handle(...)` dominating afterward

## Rejected Cut T

### Name

latest+previous `handle_cache` widening with detached array-store lookup

### Intent

- widen `handle_cache` from one slot to a tiny latest+previous cache so alternating array/string handles can stay hot together
- route `array_set_by_index_string_handle_value(...)` through a detached array cache helper, then look up the source string handle via `object_from_handle_cached(...)`
- keep the cut generic and structural instead of adding another benchmark-shaped observer/helper split

### Result

- same-artifact proof: `kilo_meso_substring_concat_len = 35 ms`, `kilo_meso_substring_concat_array_set = 65 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 69 ms`
- stable main: `kilo_kernel_small_hk = 701 ms` (`repeat=3`)
- quick asm still kept `Registry::alloc`, `Registry::get`, `array_set_his`, `substring_hii`, and `concat3_hhh` in the top tier

### Judgment

reject

### Why

- the wider cache trimmed meso slightly, but it still did not beat the kept `668 ms` line on main
- making `array_set` use a detached cache helper introduced more structure without changing the retained/store-boundary contract enough to matter at `kilo_kernel_small_hk` scale
- this remains a cache-local retry, not the larger upstream placement cut the current lane needs

### Reopen Condition

- only if a future upstream placement wave keeps both the array handle and more than one transient string handle live across the same store boundary, and a fresh asm read still shows `Registry::get` dominating afterward

## Rejected Cut U

### Name

compiler-local `has_direct_array_set_consumer(...)` first-use relaxation

### Intent

- relax the concat3/string-insert lowering predicate so `array.set` counts as the first consumer even when `out.length()` remains afterward
- keep the change compiler-local, without adding a new runtime token or widening the runtime store boundary
- let the `concat3` route remain backend-local until the first store boundary while preserving later observer uses

### Result

- same-artifact proof: `kilo_meso_substring_concat_len = 35 ms`, `kilo_meso_substring_concat_array_set = 67 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 67 ms`
- stable main back-to-back checks: `kilo_kernel_small_hk = 698 ms`, then `697 ms` (`repeat=3`)
- quick asm still kept `Registry::alloc`, `Registry::get`, `BoxBase::new`, `substring_hii`, `array_set_his`, and `string_handle_from_owned` above the residual observer route

### Judgment

reject

### Why

- the relaxed first-use predicate still did not beat the kept `668 ms` main line
- it trimmed a little compiler-side shape pressure, but not enough retained/store-boundary work to matter
- the change is still only a lowering heuristic; it does not give the carrier a longer life past the handle-shaped `concat3_hhh` export surface

### Reopen Condition

- only if a future upstream placement wave proves the first consumer is sufficient to keep a transient carrier alive across the store boundary without regressing the trailing observer, and a fresh asm read shows that the later `length()` observer has become the dominant residue
