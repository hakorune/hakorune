# Phase 157x: observe feature split

- Status: Active
- 目的: observer を authority stack の外に固定し、default release から compile-out する。
- 対象:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/design/semantic-optimization-authority-ssot.md`
  - `docs/reference/environment-variables.md`
  - `crates/nyash_kernel/Cargo.toml`
  - `crates/nyash_kernel/src/observe/**`

## Goal

- observer は `.hako owner -> MIR canonical contract -> Rust executor -> LLVM` の authority stack に入れない
- `perf-observe` feature で observer code を compile-in / compile-out する
- `NYASH_PERF_COUNTERS=1` は feature-on build の runtime gate としてだけ使う

## Current Shape

- default build
  - observer code は compile-out
  - `observe::flush()` / `record_*()` は no-op
- `--features perf-observe`
  - contract identity:
    - `store.array.str`
    - `const_suffix`
  - backend:
    - atomic exact counters
  - sink:
    - stderr summary

## Next

1. `phase-158x observe tls backend`
2. `phase-159x observe trace split`
3. `phase-137x main kilo reopen selection`

## Exit

- default build で observer code が compile-out している
- feature-on build で `NYASH_PERF_COUNTERS=1` summary が出る
- current docs が `perf-observe` build を current truth として読める

## First Proof

- default build:
  - `cargo check -p nyash_kernel` PASS
  - `cargo test -p nyash_kernel string_concat_hs_contract -- --nocapture` PASS
- feature-on build:
  - `cargo check -p nyash_kernel --features perf-observe` PASS
  - `cargo test -p nyash_kernel --features perf-observe set_his_alias_sets_string_handle_value -- --nocapture` PASS
  - `cargo build --release -p nyash_kernel --features perf-observe` PASS
- feature-on AOT probe:
  - `bench_kilo_micro_array_string_store.hako`
  - stderr:
    - `[perf/counter][store.array.str] total=800000 cache_hit=800000 cache_miss_handle=0 cache_miss_epoch=0 retarget_hit=800000 source_store=0 non_string_source=0`
    - `[perf/counter][const_suffix] total=800000 cached_handle_hit=799999 text_cache_reload=1 freeze_fallback=0`
