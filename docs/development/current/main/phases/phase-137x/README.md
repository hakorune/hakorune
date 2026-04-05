# Phase 137x: main kilo reopen selection

- Status: Active
- 目的: `phase-134x` で landed した `nyash_kernel` の 4 層 split を前提に、`main kilo` を split kernel 上で reopen する。
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
- current work is to reopen `main kilo` on the split `nyash_kernel`
- `vm-hako` stays parked as reference/conformance

## Fresh Read

- `exports/string.rs` is now a thin export shell with helpers split out
- `plugin/map_substrate.rs` is now raw substrate helpers only
- `plugin/map_aliases.rs` now owns the ABI alias surface
- `nyash_kernel` is ready to be re-baselined under the new responsibility split

## Next

1. refresh `kilo_kernel_small_hk` baselines
2. recheck `kilo_micro_substring_concat`
3. recheck `kilo_micro_array_getset`
4. choose the next hot leaf under the split kernel
