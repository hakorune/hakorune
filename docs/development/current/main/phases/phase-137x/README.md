# Phase 137x: main kilo reopen selection

- Status: Active
- 目的: semantic ownership の最終形と canonical lowering visibility lock が landed した split kernel 上で `main kilo` を reopen する。llvmlite object emit retreat は landed し、現在は canonical perf front freeze の後続 consumer として待機。
- 対象:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `crates/nyash_kernel/src/exports/string.rs`
  - `crates/nyash_kernel/src/plugin/map_substrate.rs`
  - `crates/nyash_kernel/src/plugin/map_aliases.rs`
  - `crates/nyash_kernel/src/exports/string_helpers.rs`

## Decision Now

- fixed perf order remains:
  1. `leaf-proof micro`
  2. `micro kilo`
  3. `main kilo`
- `phase-134x` structural split is landed
- `phase-138x` / `phase-139x` / `phase-140x` / `phase-141x` semantic-owner corridor is landed
- contract-first corridor は landed
- Birth / Placement vocabulary lock is now landed in design SSOT
- perf consumer は llvmlite object emit retreat の後で reopen
- `vm-hako` stays parked as reference/conformance

## Fresh Read

- `exports/string.rs` is now a thin export shell with helpers split out
- `plugin/map_substrate.rs` is now raw substrate helpers only
- `plugin/map_aliases.rs` now owns the ABI alias surface
- `nyash_kernel` is ready to be re-baselined under the new responsibility split
- `src/tests.rs` has been split into `tests/filebox.rs` and `tests/string.rs`, so the root test module is no longer a 1000+ line monolith
- reopened perf read:
  - baseline: `kilo_kernel_small_hk`: `c_ms=81 / ny_aot_ms=1529`
  - after string const-path branch collapse: `c_ms=82 / ny_aot_ms=775`
  - after const-handle cache follow-up: `c_ms=84 / ny_aot_ms=731`
  - after const empty-flag cache: `c_ms=81 / ny_aot_ms=723`
  - after shared text-based const-handle helper: `c_ms=80 / ny_aot_ms=903`
  - after single-closure const suffix fast path: `c_ms=83 / ny_aot_ms=820`
  - latest whole-kilo reread after visibility lock: `c_ms=83 / ny_aot_ms=762`
  - `kilo_micro_indexof_line`: `c_ms=4 / ny_aot_ms=4`
  - `kilo_micro_substring_concat`: `c_ms=3 / ny_aot_ms=3`
  - `kilo_micro_array_getset`: `c_ms=4 / ny_aot_ms=4`
  - `kilo_micro_concat_const_suffix`: `c_ms=3 / ny_aot_ms=84`
  - `kilo_micro_concat_hh_len`: `c_ms=3 / ny_aot_ms=57`
  - `kilo_micro_concat_birth`: `c_ms=6 / ny_aot_ms=47`
  - `kilo_micro_array_string_store`: `c_ms=10 / ny_aot_ms=181`
  - whole-kilo recheck after array-cache epoch pass-through: `c_ms=81 / ny_aot_ms=741`
- latest bundle read:
  - string contracts remain `keep_transient -> fresh_handle` for non-empty const concat/insert
  - `20260406-024104` still shows `crates/nyash_kernel/src/exports/string_helpers.rs::concat_const_suffix_fallback` as the top explicit hot symbol (`11.70%`)
  - `crates/nyash_kernel/src/plugin/array_string_slot.rs::array_string_store_handle_at` remains second (`5.68%`)
  - exact micro gap is currently larger on `array_string_store`
 - deeper observe drill-down now exists for:
   - `store.array.str`: `existing_slot / append_slot / source_string_box / source_string_view / source_missing`
   - `const_suffix`: `empty_return / cached_fast_str_hit / cached_span_hit`
   - generic string consumer:
     - `str.concat2.route`: `total / dispatch_hit / fast_str_owned / fast_str_return_handle / span_freeze / span_return_handle / materialize_fallback / unclassified`
     - `str.len.route`: `total / dispatch_hit / fast_str_hit / fallback_hit / miss / latest_fresh_handle_fast_str_hit / latest_fresh_handle_fallback_hit / unclassified`
   - `birth.placement`: `return_handle / borrow_view / freeze_owned / fresh_handle / materialize_owned / store_from_source`
   - `birth.backend`: `freeze_text_plan_total / view1 / pieces2 / pieces3 / pieces4 / owned_tmp / materialize_owned_total / materialize_owned_bytes / string_box_new_total / string_box_new_bytes / string_box_ctor_total / string_box_ctor_bytes / arc_wrap_total / handle_issue_total / gc_alloc_called / gc_alloc_bytes / gc_alloc_skipped`
 - exact observe read:
   - `kilo_micro_array_string_store` AOT direct probe is saturated on one shape:
     - `cache_hit=800000`
     - `retarget_hit=800000`
     - `existing_slot=800000`
     - `source_string_box=800000`
   - current cache-churn hypothesis is not supported on that exact micro
   - `kilo_micro_concat_const_suffix` AOT direct probe does not hit `const_suffix`
   - the current AOT workload lowers through `nyash.string.concat_hh` and `nyash.string.len_h`
   - next local trim should therefore target the generic string concat/len consumer, not `concat_hs`
   - `kilo_micro_concat_hh_len` now isolates that generic consumer without substring carry
   - `kilo_micro_concat_birth` now isolates fresh concat birth/materialize with only final `len`
   - `kilo_micro_concat_birth` direct probe currently shows:
     - `birth.placement`: `fresh_handle=800000`
     - `birth.backend`: `materialize_owned_total=800000`, `materialize_owned_bytes=14400000`, `gc_alloc_called=800000`, `gc_alloc_bytes=14400000`
     - `str.concat2.route`: `fast_str_owned=800000`, other classified routes `0`, `unclassified=0`
     - `str.len.route`: `fast_str_hit=1`, `latest_fresh_handle_fast_str_hit=1`, other classified routes `0`, `unclassified=0`
   - `kilo_micro_concat_hh_len` Birth / Placement direct probe currently shows:
     - `birth.placement`: `fresh_handle=800000`
      - `birth.backend`: `materialize_owned_total=800000`, `materialize_owned_bytes=14400000`, `string_box_new_total=800000`, `string_box_new_bytes=14400000`, `handle_issue_total=800000`, `gc_alloc_called=800000`, `gc_alloc_bytes=14400000`
     - `str.concat2.route`: `fast_str_owned=800000`, other classified routes `0`, `unclassified=0`
     - `str.len.route`: `fast_str_hit=800002`, `latest_fresh_handle_fast_str_hit=800000`, other classified routes `0`, `unclassified=0`
     - `return_handle / borrow_view / freeze_owned = 0`
   - `NYASH_PERF_BYPASS_GC_ALLOC=1` diagnostic observe lane shows:
     - `kilo_micro_concat_birth`: `50 -> 51 ms`
     - `kilo_micro_concat_hh_len`: `72 -> 70 ms`
     - observe-build `kilo_kernel_small_hk`: `1077 -> 1084 ms`
     - direct probe cleanly flips:
       - `gc_alloc_called=800000 -> 0`
       - `gc_alloc_skipped=0 -> 800000`
   - current evidence does not support `gc_alloc(...)` call overhead as the next main driver
   - route conservation now also says:
     - both exact fronts stay on `fast_str_owned`
     - both `len` consumers stay on `fast_str_hit`
     - `concat_hh + len_h` exact path usually reads the latest fresh handle directly after issue
     - `unclassified=0`
   - external design lock after the latest exact/whole split:
     - do not treat birth as one fused event
     - read current backend as:
       - byte birth = `MaterializeOwned`
       - object birth = `StableBoxNow`
       - publication birth = `FreshRegistryHandle`
     - next backend-private carriers are:
       - `OwnedBytes`
       - `TextReadSession`
     - next structural goal is to reduce `StableBoxNow` demand before trying to
       make `next_box_id` or registry issue cheaper again
   - source-backed private seam slice is now in place:
     - `OwnedBytes` exists in `string_store.rs`
     - `TextReadSession` exists in `host_handles.rs`
     - `string_len_from_handle(...)`, `string_is_empty_from_handle(...)`,
       `concat_pair_from_fast_str(...)`, and `concat3_plan_from_fast_str(...)`
       now read through the session seam
     - this slice does not reintroduce deferred objectization behavior
   - `StableBoxNow` demand probe now also exists:
     - `kilo_micro_concat_birth`
       - `object_get_latest_fresh=0`
       - `object_with_handle_latest_fresh=0`
       - `object_pair_latest_fresh=0`
       - `object_triple_latest_fresh=0`
       - `text_read_handle_latest_fresh=1`
       - `text_read_pair_latest_fresh=0`
       - `text_read_triple_latest_fresh=0`
     - `kilo_micro_concat_hh_len`
       - `object_get_latest_fresh=0`
       - `object_with_handle_latest_fresh=0`
       - `object_pair_latest_fresh=0`
       - `object_triple_latest_fresh=0`
       - `text_read_handle_latest_fresh=800000`
       - `text_read_pair_latest_fresh=0`
       - `text_read_triple_latest_fresh=0`
     - latest fresh handles are staying inside the single-handle text-read seam on the current exact fronts
     - exact micro evidence does not support object-world leakage as the current first cause
   - delayed `StableBoxNow` retry truth:
     - exact micro improved:
       - `kilo_micro_concat_birth`: `50 -> 37 ms`
       - `kilo_micro_concat_hh_len`: `67 -> 57 ms`
     - whole-kilo still regressed:
       - `kilo_kernel_small_hk`: `764 ms`
     - whole observe probe points at early object-world escalation instead of exact-path leakage:
       - `stable_box_demand.object_with_handle_latest_fresh=540000`
       - `stable_box_demand.object_get_latest_fresh=0`
       - `stable_box_demand.object_pair_latest_fresh=0`
       - `stable_box_demand.object_triple_latest_fresh=0`
       - `stable_box_demand.text_read_handle_latest_fresh=0`
       - `stable_box_demand.text_read_pair_latest_fresh=938`
     - current read:
       - exact micro stays inside the single-handle text-read seam
       - whole-kilo quickly promotes latest fresh string handles into generic object `with_handle(...)`
       - delayed objectization must not be relanded until that consumer is widened or bypassed
     - caller-attributed whole-kilo truth:
       - `stable_box_demand.object_with_handle_array_store_str_source_latest_fresh=540000`
       - `stable_box_demand.object_with_handle_substring_plan_latest_fresh=0`
       - `stable_box_demand.object_with_handle_decode_array_fast_latest_fresh=0`
       - `stable_box_demand.object_with_handle_decode_any_arg_latest_fresh=0`
       - `stable_box_demand.object_with_handle_decode_any_index_latest_fresh=0`
     - source-backed `store.array.str` split confirms that this whole-kilo latest-fresh demand is entirely retarget-side:
       - `store.array.str latest_fresh_retarget_hit=540000`
       - `store.array.str latest_fresh_source_store=0`
     - borrowed alias whole-kilo truth:
       - `borrowed.alias.borrowed_source_fast=540000`
       - `borrowed.alias.as_str_fast=540064`
       - `borrowed.alias.as_str_fast_live_source=540064`
       - `borrowed.alias.as_str_fast_stale_source=0`
       - `borrowed.alias.array_len_by_index_latest_fresh=1`
       - `borrowed.alias.array_indexof_by_index_latest_fresh=938`
       - `borrowed.alias.encode_epoch_hit=0`
       - `borrowed.alias.encode_ptr_eq_hit=0`
       - `borrowed.alias.encode_to_handle_arc=0`
     - current read:
       - retargeted latest-fresh aliases are not escaping through encoder fallback
       - `BorrowedHandleBox::as_str_fast()` stays entirely on the live-source side in whole-kilo
       - `array_string_len_by_index(...)` / `array_string_indexof_by_index(...)` are not the 540k latest-fresh culprit
       - the remaining stable object pressure stays on `store.array.str -> with_handle(ArrayStoreStrSource)` itself, not alias runtime encode
     - current first widening target is therefore:
       - `store.array.str` source read under `array_string_slot.rs`
     - attempted widening truth:
       - redirecting `store.array.str` source read into `TextReadSession` moved latest fresh demand out of `object_with_handle(...)`
       - but plain release regressed:
         - `kilo_micro_array_string_store: 181 -> 187 ms`
         - `kilo_kernel_small_hk: 757 -> 916 ms`
       - the behavior change is reverted; keep the caller attribution only
     - narrow `retarget` retry truth:
       - a no-op guard in `try_retarget_borrowed_string_slot_verified(...)` for unchanged `(source_handle, source_drop_epoch)` did not materially move the front
       - plain release recheck:
         - `kilo_micro_array_string_store: 183 ms`
         - `kilo_kernel_small_hk: 746 ms`
       - the behavior change is reverted; keep the counter truth only
     - latest-fresh stable object cache truth:
       - caching the newest `Arc<dyn NyashBox>` in TLS and short-circuiting `with_handle(ArrayStoreStrSource)` regressed exact and whole
       - plain release 3-run:
         - `kilo_micro_array_string_store: 210 ms`
         - `kilo_micro_concat_hh_len: 78 ms`
         - `kilo_kernel_small_hk: 760 ms`
       - the behavior change is reverted
     - borrowed alias raw string cache truth:
       - caching source string addr/len inside `BorrowedHandleBox` and bypassing `inner.as_str_fast()` regressed exact and whole
       - plain release 3-run:
         - `kilo_micro_array_string_store: 196 ms`
         - `kilo_micro_concat_hh_len: 69 ms`
         - `kilo_kernel_small_hk: 798 ms`
       - the behavior change is reverted
   - next observation order is fixed:
     1. split the `store.array.str -> with_handle(ArrayStoreStrSource)` object contract again before changing behavior
     2. keep borrowed alias string-read trimming closed; live-source fast read was not enough
     3. only then retry delayed `StableBoxNow`
   - `DeferredString` experiment truth:
     - exact micro improved:
       - `kilo_micro_concat_hh_len`: `57 -> 51 ms`
       - `kilo_micro_concat_birth`: `47 -> 35 ms`
     - whole-kilo probe regressed:
       - `kilo_kernel_small_hk`: `741 -> 952 ms`
     - code was reverted
     - next widening choice is now:
       1. explain the whole-kilo regression first
       2. only then reconsider pair/span widening
   - `host_handles` now has a source-backed payload seam:
     - slot storage reads through `HandlePayload::StableBox(...)`
     - public registry APIs still return `Arc<dyn NyashBox>`
     - this does not change behavior yet; it only narrows the future widening point for `DeferredStableBox`
     - single-handle string-only access is also separated now:
       - `host_handles::with_str_handle(...)`
       - `string_len_from_handle(...)` and `string_is_empty_from_handle(...)` consume that seam
   - current exact backend front is therefore:
     - `FreshHandle`
     - `MaterializeOwned`
   - current birth backend split now reads:
     - `StringBox` ctor side before registry issue
     - direct probe now also shows:
       - `string_box_ctor_total=800000`
       - `string_box_ctor_bytes=14400000`
       - `arc_wrap_total=800000`
     - observe-build `kilo_micro_concat_birth` microasm top:
       - `birth_string_box_from_owned`: `38.23%` to `41.46%`
       - `issue_string_handle_from_arc`: `27.66%` to `31.54%`
       - `__memmove_avx512_unaligned_erms`: `9.10%` to `10.88%`
       - `string_concat_hh_export_impl`: `11.53%` to `12.73%`
   - release observe direct probe now confirms second-axis counters too:
     - `objectize_stable_box_now_total=800000`
     - `objectize_stable_box_now_bytes=14400000`
     - `issue_fresh_handle_total=800000`
   - `kilo_micro_concat_birth` observe-build microasm after backend split now reads:
     - `materialize_owned_bytes`: `25.81%`
     - `issue_fresh_handle`: `24.62%`
     - `StringBox::perf_observe_from_owned`: `21.27%`
     - `__memmove_avx512_unaligned_erms`: `14.67%`
     - `nyash.string.concat_hh`: `5.81%`
   - annotate for `issue_fresh_handle(...)` shows the dominant local leaf is the final registry unlock/release path
   - next backend front is therefore:
     1. `materialize_owned_bytes`
     2. `issue_fresh_handle`
     3. `StringBox::perf_observe_from_owned`
   - do not spend more time on concat/len route guessing for these exact fronts unless a future counter contradicts the current read
   - `objectize_stable_string_box` stays as the seam name, but most runtime cost is currently absorbed by ctor/issue leaves
   - backend second-axis lock:
     - top-level Birth / Placement vocabulary stays unchanged
     - `box_id` is not promoted into that vocabulary
     - backend-only reading is now:
       - `Objectization = None | StableBoxNow | DeferredStableBox`
       - `RegistryIssue = None | ReuseSourceHandle | FreshRegistryHandle`
     - current `concat_birth` path still couples:
       - `FreshHandle`
       - `MaterializeOwned`
       - `StableBoxNow`
       - `FreshRegistryHandle`
     - current source-backed backend split is now visible in `string_store.rs`:
       - `materialize_owned_bytes`
       - `objectize_stable_string_box`
       - `issue_fresh_handle`
     - second-axis counters now also exist for:
       - `objectize_stable_box_now_total / bytes`
       - `issue_fresh_handle_total`
     - observe lane contract is now fail-fast:
       - default perf AOT lane aborts unless `target/release/.perf_release_sync` is newer than both `target/release/libnyash_kernel.a` and `target/release/hakorune`
       - `NYASH_PERF_COUNTERS=1` / `NYASH_PERF_TRACE=1` still require `target/release/.perf_observe_release_sync`
       - canonical rebuild orders are fixed in `tools/perf/build_perf_release.sh` and `tools/perf/build_perf_observe_release.sh`
       - helper-local ranking rule:
         - plain release asm = real cost ranking
         - observe build = counts and symbol split
         - `materialize_owned_bytes(...)` observe annotate is currently dominated by TLS counter work, so it is not sufficient by itself for first-front ordering
   - current microasm read:
     - `string_concat_hh_export_impl`: `54.04%`
     - `string_len_from_handle`: `21.37%`
     - `__memmove_avx512_unaligned_erms`: `15.40%`

## Next

1. keep canonical contract corridor landed and immutable
2. treat `kilo_micro_concat_const_suffix` as the current generic string concat/len front
   - current AOT consumer: `nyash.string.concat_hh` + `nyash.string.len_h`
   - current executor: `string_concat_hh_export_impl(...)` + `string_len_from_handle(...)`
   - use `kilo_micro_concat_hh_len` as the exact isolated repro before changing this front
   - use `kilo_micro_concat_birth` when the patch only targets fresh birth/materialize backend cost
   - read this front through Birth / Placement outcome names first:
     - `FreshHandle`
     - `MaterializeOwned`
   - current direct probe says this front is not exercising:
     - `ReturnHandle`
     - `BorrowView`
     - `FreezeOwned`
   - current direct probe also says:
     - latest fresh handles are consumed through `text_read_handle`
     - they are not currently leaking into object get/with/pair/triple APIs on the exact fronts
   - next backend trim order:
     1. widen or bypass `store.array.str` source read so latest fresh string handles stay inside `TextReadSession`
     2. only after that retry delayed `StableBoxNow`
     3. only then trim:
        - `materialize_owned_bytes`
        - `issue_fresh_handle`
        - `next_box_id`
3. keep canonical `store.array.str` as the next exact front
   - current executor: `array_string_store_handle_at(...)`
4. keep canonical `const_suffix` / `thaw.str + lit.str + str.concat2 + freeze.str` as a separate route, but do not assume the current exact micro exercises it
5. use exact micro + whole-kilo together before moving to a new leaf
