# Phase 134x: nyash_kernel layer recut selection

- Status: ✅ landed
- 目的: 最適化を再開する前に `crates/nyash_kernel` を `keep / thin keep / compat glue / substrate candidate` の4層で再分類し、最初の source slice を固定する。
- 対象:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `crates/nyash_kernel/src/exports/string.rs`
  - `crates/nyash_kernel/src/plugin/map_substrate.rs`
  - `crates/nyash_kernel/src/plugin/array_substrate.rs`
  - `crates/nyash_kernel/src/plugin/future.rs`
  - `crates/nyash_kernel/src/plugin/invoke_core.rs`
  - `crates/nyash_kernel/src/hako_forward_bridge.rs`
- success:
  - `phase-133x` is landed
  - current no longer reads like direct perf work
  - `nyash_kernel` four-bucket split is source-backed
  - the first two source slices are landed
  - `phase-137x main kilo reopen selection` is next

## Decision Now

- fixed perf order remains:
  1. `leaf-proof micro`
  2. `micro kilo`
  3. `main kilo`
- but `main kilo` is intentionally delayed by one structural corridor
- current work is:
  - classify `nyash_kernel` by responsibility
  - not broad `.hako` migration
  - not broad hot-path tuning
- landed source slices:
  - `exports/string.rs` split
  - `plugin/map_substrate.rs` thin-alias recut

## Fresh Read

- `crates/nyash_kernel` still mixes:
  - ABI entrypoints
  - runtime glue / compat
  - hot leaf substrate
- `array_substrate.rs` is already close to the target thin-keep shape
- `map_substrate.rs` still mixes alias surface and compat observer/mutator pressure
- `exports/string.rs` still mixes ABI surface, fast path, materialization, and concat helpers
- `future.rs`, `invoke_core.rs`, `hako_forward_bridge.rs`, and `module_string_dispatch/**` read as frozen glue, not substrate candidates

## Next

1. reopen `main kilo` on the split kernel
2. refresh `kilo_kernel_small_hk` baseline
3. recheck `kilo_micro_substring_concat` and `kilo_micro_array_getset`
