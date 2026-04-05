# Phase 137x: main kilo reopen selection

- Status: Active
- 目的: semantic ownership の最終形が landed したので、split kernel 上で `main kilo` を reopen する。
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
- current work is `main kilo` reopen
- `vm-hako` stays parked as reference/conformance

## Fresh Read

- `exports/string.rs` is now a thin export shell with helpers split out
- `plugin/map_substrate.rs` is now raw substrate helpers only
- `plugin/map_aliases.rs` now owns the ABI alias surface
- `nyash_kernel` is ready to be re-baselined under the new responsibility split
- `src/tests.rs` has been split into `tests/filebox.rs` and `tests/string.rs`, so the root test module is no longer a 1000+ line monolith
- reopened perf read:
  - baseline: `kilo_kernel_small_hk`: `c_ms=81 / ny_aot_ms=1529`
  - after `concat_const_suffix_fallback` fast path: `c_ms=83 / ny_aot_ms=905`
  - after const-handle cache follow-up: `c_ms=84 / ny_aot_ms=731`
  - `kilo_micro_indexof_line`: `c_ms=4 / ny_aot_ms=4`
  - `kilo_micro_substring_concat`: `c_ms=3 / ny_aot_ms=3`
  - `kilo_micro_array_getset`: `c_ms=4 / ny_aot_ms=4`
- latest bundle read:
  - string contracts remain `keep_transient -> fresh_handle` for non-empty const concat/insert
  - next independent leaf is `crates/nyash_kernel/src/plugin/array_string_slot.rs::array_string_store_handle_at`

## Next

1. optimize `array_string_store_handle_at(...)`
2. refresh `kilo_kernel_small_hk`
3. re-bundle and decide whether string or array-handle-cache is next
4. hand off to the next optimization lane
