# Phase 156x: perf counter instrumentation

- Status: Landed
- 目的: `phase-137x` の canonical perf front を推定ではなく route-tagged counter で読めるようにする。
- 対象:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/reference/environment-variables.md`
  - `crates/nyash_kernel/src/perf_counters.rs`
  - `crates/nyash_kernel/src/plugin/handle_cache.rs`
  - `crates/nyash_kernel/src/plugin/array_string_slot.rs`
  - `crates/nyash_kernel/src/exports/string_helpers.rs`
  - `crates/nyash_kernel/src/entry.rs`

## Goal

- opt-in counter を canonical contract 単位で取る
- `store.array.str` は cache hit/miss と store path を読めること
- `const_suffix` は cached-handle hit / text reload / freeze fallback を読めること
- verbose trace ではなく exit summary で 1 回読めること

## Canonical Counters

1. `store.array.str`
   - `total`
   - `cache_hit`
   - `cache_miss_handle`
   - `cache_miss_epoch`
   - `retarget_hit`
   - `source_store`
   - `non_string_source`
2. `const_suffix`
   - `total`
   - `cached_handle_hit`
   - `text_cache_reload`
   - `freeze_fallback`

## Env

- `NYASH_PERF_COUNTERS=1`
  - stderr に route-tagged summary を出す
  - authority は増やさず、Rust executor の内部観測だけを追加する

## Exit

- `NYASH_PERF_COUNTERS=1` で whole-kilo/exact-micro の両方に summary が出る
- current docs が `phase-156x` を current として読める
- その後に `phase-137x main kilo reopen selection` へ戻る

## First Probe

- exact AOT probe: `bench_kilo_micro_array_string_store.hako`
  - `[perf/counter][store.array.str] total=800000 cache_hit=800000 cache_miss_handle=0 cache_miss_epoch=0 retarget_hit=800000 source_store=0 non_string_source=0`
  - first read: cache churn 仮説はこの exact micro では当たっていない
- exact AOT probe: `bench_kilo_micro_concat_const_suffix.hako`
  - current direct probe では counter が `0` のままだった
  - current AOT lowering が `concat_const_suffix_fallback(...)` を踏んでいない可能性があるので、consumer path は再確認が要る
