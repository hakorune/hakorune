---
Status: investigation
Date: 2026-03-28
Scope: `perf-kilo` current wave の string birth hot path について、accepted / rejected / stop-line を 1 枚で読めるようにする
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/concat3-array-store-placement-window-ssot.md
  - docs/development/current/main/design/retained-boundary-and-birth-placement-ssot.md
  - docs/development/current/main/design/post-store-observer-facts-ssot.md
  - docs/development/current/main/design/string-birth-sink-ssot.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/investigations/perf-kilo-string-leaf-rejected-followups-2026-03-28.md
  - crates/nyash_kernel/src/exports/string.rs
  - crates/nyash_kernel/src/exports/string_view.rs
  - crates/nyash_kernel/src/plugin/value_codec/string_store.rs
  - src/runtime/host_handles.rs
  - src/boxes/basic/string_box.rs
---

# Perf Kilo String Birth Hot Path Summary (2026-03-28)

## Goal

`perf-kilo` の string hot path について、今進んだもの・外れたもの・止めるべきところを 1 枚で読めるようにする。

このノートは shell history の代わりではなく、current lane の summary だよ。

## Current Accepted Slices

- placement helper landed
  - `crates/nyash_kernel/src/exports/string_birth_placement.rs` now owns the compile-time placement vocabulary
  - `substring_hii` / `concat_hs` / `insert_hsi` / `concat3_hhh` now read the same retention classes
- docs-first retained-boundary split landed
  - `retained-boundary-and-birth-placement-ssot.md` now owns `BoundaryKind` vs `RetainedForm`
  - `placement` and `sink` no longer share the same parent explanation
- `freeze.str` は canonical sink として残す
  - `concat_hs` と `insert_hsi` は `freeze_text_plan(...)` を共有
  - `concat3_hhh` は file-local plan/freeze split のまま
- `BorrowedSubstringPlan` は recipe-only / boundary-only まで縮んだ
  - `crates/nyash_kernel/src/exports/string_view.rs` が recipe 側を持つ
  - `crates/nyash_kernel/src/exports/string.rs` が dispatch + freeze を受ける
- store boundary の in-place source borrow は landed
  - `array_set_by_index_string_handle_value(...)` は hot path で一時 `Arc` clone を作らない
- sink-local hot branch の direct cut も landed
  - `Registry::alloc` は hot birth branch を registry 内で直展開
  - `Registry::get` は direct clone path へ縮退
- narrow branch-check trim landed
  - `concat_hs` const-suffix empty-path and `insert_hsi` source-empty lookup are no longer checked twice
  - kept recheck is `kilo_kernel_small_hk = 707 ms`, `kilo_meso_substring_concat_array_set = 68 ms`
- concat3 lock-safe fast path landed
  - `concat3_plan_from_fast_str(...)` and `concat_pair_from_fast_str(...)` now return a reuse-or-owned decision before freeze, so the registry read lock is no longer held across `freeze_text_plan(...)`
  - `resolve_string_span_triplet_from_handles(...)` plus `string_span_cache_get_triplet(...)` land the triple-span route
  - latest same-artifact recheck after this concat3 fix is `kilo_meso_substring_concat_len = 36 ms`, `kilo_meso_substring_concat_array_set = 67 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 67 ms`, `kilo_kernel_small_hk = 704 ms`
- short-slice substring freeze cut landed
  - `BorrowedSubstringPlan` now returns `FreezeSpan(StringSpan)` for short freeze-only slices instead of wrapping them in `TextPlan::from_span(...)`
  - `substring_hii` materializes those short spans directly via `string_handle_from_span(...)`
  - latest same-artifact recheck after this cut is `kilo_meso_substring_concat_len = 35 ms`, `kilo_meso_substring_concat_array_set = 67 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 69 ms`, `kilo_kernel_small_hk = 704 ms`
- array string-length observer cut landed
  - `array_string_len_by_index(...)` now uses `handle_cache::with_array_box(...)` instead of `host_handles::with_handle(...)` plus `ArrayBox` downcast, so the read-only `nyash.array.string_len_hi` observer stays on the typed handle-cache path
  - latest `repeat=3` proof after this cut is `kilo_meso_substring_concat_len = 35 ms`, `kilo_meso_substring_concat_array_set = 68 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 69 ms`, `kilo_kernel_small_hk = 721 ms`
  - latest `repeat=20` WSL recheck is `kilo_meso_substring_concat_len = 36 ms`, `kilo_meso_substring_concat_array_set = 67 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 68 ms`, `kilo_kernel_small_hk = 688 ms`
  - latest microasm still keeps `nyash.array.string_len_hi` in the hot tier (`6.34%`), so this generic observer cut is worth keeping even though the lane is still noisy
- post-store observer facts landed
  - `array.set` remains the first `Store` proof boundary, and trailing `length()` is now read as a post-store observer fact in `post-store-observer-facts-ssot.md`
  - keep the stricter store-only consumer guard; do not merge the observer into the store boundary
- compiler-local placement window is the next large-cut design front
  - `concat3-array-store-placement-window-ssot.md` now fixes the next rollout contract as `concat3_hhh -> array.set -> trailing length()`
  - the next cut must be driven by compiler-local facts (`remember_string_concat_*`, `remember_string_substring_call(...)`, `remember_string_length_call(...)`, `has_direct_array_set_consumer(...)`, `analyze_array_string_len_window_candidate(...)`), not by another helper-local leaf retry
- concat3 reuse-only specialization landed
  - `concat3_plan_from_spans(...)` is fixed to the reuse-allowed lane, so the dead `allow_handle_reuse = false` branch is gone and span emptiness checks use byte-range length directly
  - latest same-artifact recheck after this specialization is `kilo_meso_substring_concat_len = 34 ms`, `kilo_meso_substring_concat_array_set = 66 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 65 ms`, `kilo_kernel_small_hk = 668 ms`
- rejected short-slice owned materialize retry
  - changing the short freeze lane to `FreezeOwned(String)` and materializing inside `borrowed_substring_plan_from_handle(...)` regressed stable main to `866 ms`; keep the span-backed short freeze contract for now

## Latest Same-Artifact Proof

The retained-boundary parent split was docs-only, so we re-ran the same-artifact proof before opening any code-side `RetainedForm` split. The result stayed flat.

- `kilo_meso_substring_concat_len = 34 ms`
- `kilo_meso_substring_concat_array_set = 66 ms`
- `kilo_meso_substring_concat_array_set_loopcarry = 65 ms`
- `kilo_kernel_small_hk = 668 ms`

Interpretation:

- `array_set` is still the first Store proof boundary
- sink-local tuning is still exhausted
- the code-side `RetainedForm` split remains deferred
- concat3 lock-freezing is no longer on the critical path; the next step is still upstream birth-density proof
- the reuse-only concat3 specialization improved the same-artifact lane a bit; the remaining gap is still mostly birth-density / registry motion

## Current Trace Hooks

- lowering trace is available under `NYASH_LLVM_ROUTE_TRACE=1`
- the narrow placement tags are `string_direct_array_set_consumer`, `string_insert_mid_window`, and `string_concat_add_route`
- Rust string trace is split into `placement`, `carrier`, `sink`, and `observer` lines under the same gate; use it to inspect `BoundaryKind` / `RetainedForm`, carrier lineage, freeze sinks, and post-store observer resolution
- trace gate split: `#[cfg(test)]` probes read `NYASH_LLVM_ROUTE_TRACE`, while non-test runtime code reads `NYASH_VM_ROUTE_TRACE`; bench compare suppresses both stdout and stderr, so use the unit probes for visible trace capture
- canonical probe entrypoint: `tools/perf/run_kilo_string_trace_probe.sh`
  - bundles the unit trace contracts into one probe-only summary and keeps timing lanes clean
  - do not move trace capture back into `bench_compare_c_py_vs_hako.sh`
- trace+asm bundle entrypoint: `tools/perf/run_kilo_string_trace_asm_bundle.sh`
  - keeps trace and asm in the same out-dir while leaving `bench_compare` timing-only
  - resolves annotate symbols from the perf report before emitting asm notes, so the note files stop relying on stale Rust-path guesses
  - the latest bundle hot symbols are `nyash.string.concat_hh`, `nyash.string.concat3_hhh`, `nyash.string.substring_hii`, `nyash.array.set_his`, `nyash.array.string_len_hi`, `nyash_kernel::exports::string::string_handle_from_owned`, and `nyash_rust::box_trait::BoxBase::new`
- compiler route bundle note:
  - after rebuilding `libhako_llvmc_ffi.so`, `trace_optimization_bundle.sh` now exposes the placement fields in the route trace itself
  - `string_direct_array_set_consumer` now shows `producer_kind=Concat3` / `boundary_kind=Store` / `post_store_use=None` / `known_len=-1` on the direct-set hit path
  - `array_string_len_window` now shows `producer_kind=ArrayGet` / `boundary_kind=Store` / `post_store_use=LenObserver` / `known_len=-1`
- use the trace only as a probe; do not treat it as a new acceptance lane
- probe result snapshot:
  - `NYASH_LLVM_ROUTE_TRACE=1 cargo test -p nyash_kernel string_concat_hs_contract -- --nocapture`
    - `placement=keep_transient -> sink=freeze_plan -> sink=fresh_handle -> placement=return_handle`
  - `NYASH_LLVM_ROUTE_TRACE=1 cargo test -p nyash_kernel string_insert_hsi_contract -- --nocapture`
    - `observer=fast_hit -> placement=keep_transient -> sink=freeze_plan -> sink=fresh_handle -> observer=fast_hit -> placement=return_handle`
  - `NYASH_LLVM_ROUTE_TRACE=1 cargo test -p nyash_kernel substring_hii_short_slice_materializes_under_fast_contract -- --nocapture`
    - `placement=must_freeze -> carrier=freeze_span -> sink=fresh_handle -> sink=span_materialize -> observer=fast_hit`
  - `tools/perf/bench_compare_c_py_vs_hako.sh` suppresses trace output via `perf_collect_series`; use the unit probes above when you need the placement / carrier / sink / observer lines
  - bench repeat snapshot for the current kept lane:
    - `repeat=3`: `c_ms=75`, `py_ms=105`, `ny_vm_ms=978`, `ny_aot_ms=725`, `aot_status=ok`
    - `repeat=20`: `c_ms=76`, `py_ms=107`, `ny_vm_ms=990`, `ny_aot_ms=741`, `aot_status=ok`

## Current Rejected Slices

詳細は [perf-kilo-string-leaf-rejected-followups-2026-03-28.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/perf-kilo-string-leaf-rejected-followups-2026-03-28.md) に置く。

ここでは current wave で再び触らないものだけ要約するよ。

- `freeze.str` canonical sink を `string_store.rs` に移す試行
- direct `concat_hs` / `concat3` copy materialization
- piece-preserving `insert_inline` plus store/freeze reshaping
- blanket `#[inline(always)]` on host registry / hako-forward string wrappers
- `concat_hs` duplicate span-resolution removal plus span-resolver inlining
- specialized `StringBox`-only store leaf under `nyash.array.set_his`
- short-slice threshold `<= 7 bytes` plus `StringViewBox` borrow expansion
  - lowering `SUBSTRING_VIEW_MATERIALIZE_MAX_BYTES` to `7` and widening string-source borrowing to `StringViewBox` did not improve the current same-artifact lane; keep the flat `<= 8 bytes` policy for this wave
- borrowed triple-span miss resolution via `handles::with3(...)` plus local `StringViewBox` flattening
  - the narrow `resolve_string_span_triplet_from_handles(...)` borrow wave kept meso flat (`67 -> 68 ms`) and regressed stable main (`704 -> 745 -> 819 ms` on back-to-back checks); keep the explicit uncached miss path for triplet cache misses
- direct array-slot insert helper from `string_insert_mid_window`
  - wiring `nyash.array.string_insert_hisi` when both substrings traced back to the same `array.get` source regressed stable main to `1020 ms` on `repeat=3`; the `repeat=20` recheck still stayed above the kept `668 ms` line at `716 ms`
  - the quick ASM probe still centered on `string_handle_from_owned`, `concat3_hhh`, `substring_hii`, `array.set_his`, `string_len_from_handle`, and `BoxBase::new`, so this helper did not displace the real birth-density residue
- rejected small carrier cleanup retry
  - sending owned fast paths directly through `string_handle_from_owned(...)`, removing the `resolve_string_span_from_handle(...)` fallback after `TextPlan::from_handle(...)`, and using the relative range length directly inside `borrowed_substring_plan_from_handle(...)` regressed stable main to `777 ms`; keep the span-backed / helper-backed current lane for now
- rejected pair span-length retry
  - changing `concat_pair_from_spans(...)` to use span byte lengths instead of `as_str().is_empty()` regressed stable main to `904 ms`; keep the existing span-read check there for now
- rejected direct-store consumer widening
  - allowing the C-side concat lowering to treat `array.set(...)` followed by one trailing `length()` observer as the same direct-store consumer window kept the lane flat-to-worse (`kilo_meso_substring_concat_len = 36 ms`, `kilo_meso_substring_concat_array_set = 70 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 70 ms`, `kilo_kernel_small_hk = 706 ms` under `repeat=3`)
  - keep the stricter store-only consumer guard for this wave
- rejected direct-set-preferring concat3 route ordering
  - changing `string_concat_add_route` so a direct-set window prefers `concat3_hhh` before the const-suffix route looked promising in trace, but the timing-only 3-run regressed to `kilo_kernel_small_hk = 745 ms` (`c_ms = 74`, `aot_status=ok`)
  - keep the existing fallback order; the trace hit was not a wall-clock win
- rejected length-aware store-boundary classifier retry
  - changing `has_direct_array_set_consumer(...)` to classify `array.set` plus trailing `length()` as a combined store boundary regressed stable main to `kilo_kernel_small_hk = 746 ms` on `repeat=3` and `757 ms` on `repeat=20`
  - keep the direct-set-only guard for this wave; the longer classifier did not recover the upstream placement cost on this machine
- rejected known-len propagation retry
  - threading `known_len` / post-store facts from `concat_hs` / `array.set` into `length()` lowering kept the lane flat-to-worse (`kilo_meso_substring_concat_len = 38 ms`, `kilo_meso_substring_concat_array_set = 66 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 70 ms`, `kilo_kernel_small_hk = 705 ms` on `repeat=3`; `692 ms` on `repeat=20`)
  - keep `array_set` as the first Store boundary and keep trailing `length()` as a separate post-store observer fact
- rejected compiler-side insert-recipe length arithmetic
  - lowering `string.length()` on the insert-shaped concat recipe into `suffix_len + const_middle_len` improved meso (`kilo_meso_substring_concat_len = 33 ms`, `kilo_meso_substring_concat_array_set = 63 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 65 ms`)
  - the same artifact pair still regressed main to `kilo_kernel_small_hk = 695 ms` versus the kept `668 ms` concat3 reuse-only line, so the arithmetic observer rewrite is not keepable on this machine
  - keep the runtime `nyash.string.len_h` observer until a future upstream placement wave changes the retained/store boundary
- rejected combined direct-store widening plus insert-recipe length arithmetic
  - reopening the direct-store consumer window only for insert-shaped concat and pairing it with the arithmetic `length()` rewrite kept meso acceptable (`kilo_meso_substring_concat_len = 34 ms`, `kilo_meso_substring_concat_array_set = 66 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 69 ms`)
  - the same artifact pair still regressed main to `kilo_kernel_small_hk = 732 ms`, so the combined compiler-side rewrite is still worse than the kept `668 ms` line
  - keep both slices rejected until a future placement wave changes the retained/store boundary enough to justify reopening them together
- rejected substring planner cache-first retry
  - making `borrowed_substring_plan_from_handle(...)` consult `string_span_cache_get(...)` before `handles::with_handle(...)` improved meso (`kilo_meso_substring_concat_len = 33 ms`, `kilo_meso_substring_concat_array_set = 64 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 67 ms`)
  - the same artifact pair still left main at `kilo_kernel_small_hk = 706 ms`, and microasm kept `Registry::alloc`, `Registry::get`, `BoxBase::new`, `array_set_his`, and `substring_hii` ahead of the observer residue
  - keep the direct planner shape for now; the next win still needs a larger upstream placement cut, not just a cache-first substring lookup
- rejected `len/is_empty` observer cache-first retry
  - flipping `string_len_from_handle(...)` / `string_is_empty_from_handle(...)` so `string_len_impl(...)` / `string_is_empty_impl(...)` ran before the direct fast-string observer path kept meso at `35 / 68 / 71`
  - the same artifact pair regressed main to `kilo_kernel_small_hk = 764 ms`, while the quick asm probe still showed `Registry::alloc`, `Registry::get`, `array_set_his`, `substring_hii`, `concat3_hhh`, and `string_handle_from_owned` above the residual observer helpers
  - keep the current fast-str-first observer order; this wave still needs a larger upstream retained/store-boundary cut, not another observer-local cache retry
- rejected latest+previous handle-cache widening
  - widening `handle_cache` to keep the latest+previous handles and routing `array_set_by_index_string_handle_value(...)` through a detached array cache path lowered meso to `35 / 65 / 69`
  - the same artifact pair still left main at `kilo_kernel_small_hk = 701 ms`, and the quick asm probe kept `Registry::alloc`, `Registry::get`, `array_set_his`, `substring_hii`, `nyash.array.string_len_hi`, and `concat3_hhh` above the cache helper residue
  - keep the current one-slot cache; this wave does not remove enough retained/store-boundary work to matter on main
- rejected compiler-local first-use relaxation for `array.set`
  - relaxing `has_direct_array_set_consumer(...)` so `array.set` counted as the first consumer even when `out.length()` remained afterward only reached `35 / 67 / 67` on meso
  - back-to-back main checks stayed at `kilo_kernel_small_hk = 698 ms` and `697 ms`, so the stricter single-use predicate still wins on this machine
  - keep the current compiler predicate; this is still only a first-use observer relaxation, not a real upstream placement win
- rejected `insert_hsi` one-resolve helper
  - the helper-backed single-decision route improved the first `repeat=3` probe (`kilo_kernel_small_hk = 694 ms`) but drifted back to `727 ms` under `repeat=20`
  - keep the current helper-backed `insert_hsi` lane and use the documented `repeat=20` recheck rule on WSL before closing similar slices
- rejected birth-time string span cache seeding
  - seeding `string_span_cache` directly from `materialize_owned_string(...)` improved the first `repeat=3` probe (`kilo_meso_substring_concat_len = 35 ms`, `kilo_meso_substring_concat_array_set = 69 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 71 ms`, `kilo_kernel_small_hk = 692 ms`)
  - the required `repeat=20` WSL recheck drifted back to `kilo_meso_substring_concat_len = 36 ms`, `kilo_meso_substring_concat_array_set = 70 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 68 ms`, `kilo_kernel_small_hk = 730 ms`
  - keep span-cache admission on resolve-side only for now

## Current Stop-Line

- `BoxBase::new` は stop-line
  - identity semantics に結びつくので、現 wave の safe cut ではない
- `StringBox::new` も単独 target ではない
  - thin wrapper に近く、ここだけを切っても whole-program の win 根拠が弱い
- `Registry::alloc` / `Registry::get` は landed
  - sink-local lane としての追加安全 cut は現時点で無い
- current placement lane has now landed, but perf has not yet moved
  - the next step is upstream birth-density proof, not another sink-local cut
- the latest same-artifact proof stayed flat, so do not reopen code-side `RetainedForm` split yet
- do not reopen helper-local widening before the compiler-local placement window yields a same-artifact win

## Latest ASM Read

2026-03-29 の latest `kilo_meso_substring_concat_array_set` microasm 読みでは、次が top tier だったよ。

- `nyash_rust::runtime::host_handles::Registry::alloc` (`15.39%`)
- `nyash_rust::runtime::host_handles::get` (`11.67%`)
- `nyash_rust::box_trait::BoxBase::new` (`11.39%`)
- `nyash.string.substring_hii` (`6.34%`)
- `nyash_kernel::plugin::handle_cache::array_get_index_encoded_i64::_$u7b$$u7b$closure$u7d$$u7d$::h9cb324abceb701a7` (`6.19%`)
- `nyash_kernel::plugin::array_string_slot::array_set_by_index_string_handle_value::_$u7b$$u7b$closure$u7d$$u7d$::h56da430ce90ccabb` (`6.00%`)
- `nyash.array.string_len_hi` (`6.34%`)
- `nyash.string.insert_hsi` (`4.17%`)
- `nyash_kernel::exports::string_span_cache::string_span_cache_get` (`3.81%`)
- `nyash.array.set_his` (`3.72%`)
- `nyash.string.concat3_hhh` (`3.35%`)
- `nyash.array.slot_load_hi` (`3.24%`)
- `nyash_kernel::exports::string::string_is_empty_from_handle` (`3.15%`)
- `nyash_kernel::exports::string::string_len_from_handle` (`2.47%`)
- `libc.so.6::_int_malloc` (`2.46%`)
- `nyash_kernel::exports::string::string_handle_from_owned` (`2.30%`)
- `nyash_rust::runtime::global_hooks::gc_alloc` (`1.30%`)
- `__memmove_avx512_unaligned_erms` (`0.89%`)

The later birth-cache retry still left `string_len_from_handle` (`3.42%`) / `string_is_empty_from_handle` (`3.34%`) visible in the hot tier, the follow-up observer-order retry still regressed main to `764 ms`, the latest+previous handle-cache widening still stopped at `701 ms`, and the compiler-local first-use relaxation only reached `698 / 697 ms`, so the observer/cache/compiler-local slices stay rejected.

読みとしては、sink-local leaf ではなく、`Registry::alloc/get` と birth-boundary / handle registry の組み合わせがまだ支配的だよ。
ただし latest same-artifact proof が flat だったので、この lane では code-side `RetainedForm` split を再開しない。
次に触るなら、`string_birth_placement.rs` を retained-boundary parent SSOT に従って再整理し、upstream birth-density を下げる側になる。

## Next Move

- この lane では sink-local tuning を止める
- 次の候補は upstream の birth-density / placement 側
  - `BoundaryKind` と `RetainedForm` を分けた parent contract を維持する
  - `array_set` を first `Store` proof boundary として扱う
  - `StringViewBox::new` / `freeze_text_plan(...)` への到達回数を減らせるかを見る
- そこへ行く前に、docs は current truth に追従しておく

## Acceptance

- accepted / rejected / stop-line を 1 枚で辿れる
- current lane の再実装を避けられる
- 新しい leaf を探す時に、どこを見て何を見ないかが明確になる
