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
   - `birth.placement`: `return_handle / borrow_view / freeze_owned / fresh_handle / materialize_owned / store_from_source`
   - `birth.backend`: `freeze_text_plan_total / view1 / pieces2 / pieces3 / pieces4 / owned_tmp / materialize_owned_total / materialize_owned_bytes / gc_alloc_called / gc_alloc_bytes`
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
   - read this front through Birth / Placement outcome names first:
     - `ReturnHandle`
     - `BorrowView`
     - `FreezeOwned`
     - `FreshHandle`
     - `MaterializeOwned`
3. keep canonical `store.array.str` as the next exact front
   - current executor: `array_string_store_handle_at(...)`
4. keep canonical `const_suffix` / `thaw.str + lit.str + str.concat2 + freeze.str` as a separate route, but do not assume the current exact micro exercises it
5. use exact micro + whole-kilo together before moving to a new leaf
