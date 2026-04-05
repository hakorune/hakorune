# Phase 137x: main kilo reopen selection

- Status: Successor
- 目的: `phase-138x` で semantic ownership の最終形を固定した後、split kernel 上で `main kilo` を reopen する。
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
- `phase-138x` semantic owner cutover is intentionally one architecture corridor ahead of this lane
- current work is not `main kilo` yet
- `vm-hako` stays parked as reference/conformance

## Fresh Read

- `exports/string.rs` is now a thin export shell with helpers split out
- `plugin/map_substrate.rs` is now raw substrate helpers only
- `plugin/map_aliases.rs` now owns the ABI alias surface
- `nyash_kernel` is ready to be re-baselined under the new responsibility split
- `src/tests.rs` has been split into `tests/filebox.rs` and `tests/string.rs`, so the root test module is no longer a 1000+ line monolith

## Next

1. finish `phase-138x nyash_kernel semantic owner cutover`
2. refresh `kilo_kernel_small_hk` baselines
3. recheck `kilo_micro_substring_concat`
4. recheck `kilo_micro_array_getset`
5. choose the next hot leaf under the split kernel
