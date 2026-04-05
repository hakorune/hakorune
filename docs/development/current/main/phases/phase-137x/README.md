# Phase 137x: main kilo reopen selection

- Status: Paused
- 目的: semantic ownership の最終形が landed した split kernel 上で `main kilo` を reopen する。現在は optimization authority contract freeze の後続 consumer として待機。
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
- current work is paused until `phase-147x` / `148x` / `149x` contract-first corridor is fixed
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
  - latest sampled whole-kilo reread: `c_ms=82 / ny_aot_ms=775`
  - `kilo_micro_indexof_line`: `c_ms=4 / ny_aot_ms=4`
  - `kilo_micro_substring_concat`: `c_ms=3 / ny_aot_ms=3`
  - `kilo_micro_array_getset`: `c_ms=4 / ny_aot_ms=4`
  - `kilo_micro_concat_const_suffix`: `c_ms=2 / ny_aot_ms=85`
  - `kilo_micro_array_string_store`: `c_ms=9 / ny_aot_ms=217`
- latest bundle read:
  - string contracts remain `keep_transient -> fresh_handle` for non-empty const concat/insert
  - `20260406-004537` still shows `crates/nyash_kernel/src/exports/string_helpers.rs::concat_const_suffix_fallback` as the top explicit hot symbol (`10.63%`)
  - next independent leaf is `crates/nyash_kernel/src/plugin/array_string_slot.rs::array_string_store_handle_at` (`5.69%`)

## Next

1. consume `phase-147x` contract lock
2. consume `phase-148x` borrowed text / sink contract freeze
3. consume `phase-149x` concat const-suffix vertical slice
4. reopen `concat_const_suffix_fallback(...)`
5. recheck `array_string_store_handle_at(...)` only after string const-path stalls
