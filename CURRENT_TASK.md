# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-08
Scope: repo root から current lane / current front / restart read order に最短で戻るための薄い pointer。

## Purpose

- root から active lane/front を最短で読む
- landed history / rejected perf evidence は phase docs と investigations を正本にする
- `CURRENT_TASK.md` は pointer に徹し、ledger にしない

## Quick Restart Pointer

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-137x/README.md`
4. `git status -sb`
5. `tools/checks/dev_gate.sh quick`

## Restart Handoff

- current expected worktree on reopen:
  - clean after the latest keeper commit
- active lane/front:
  - lane: `phase-137x main kilo reopen selection`
  - front: `kilo_micro_substring_only`
  - rule: WSL は `3 runs + perf` でしか delta を採らない
- current exact baseline:
  - `kilo_micro_substring_only: C 3 ms / AOT 5 ms`
  - `instr: 61,072,620`
  - `cycles: 10,218,661`
  - `cache-miss: 9,409`
  - symbol order: `substring_hii 50.62%`, `len_h 39.89%`, `ny_main 4.35%`
- current whole-kilo health:
  - `tools/checks/dev_gate.sh quick` is green
  - `kilo_kernel_small_hk` strict latest accepted reread: `ny_aot_ms=701`
  - parity: `vm_result=1140576`, `aot_result=1140576`
- do not reopen:
  - `OwnedText` backing for this lane
  - live-source direct-read widening on `as_str_fast()`
  - the reverted standalone `len_h` cold-split helper shape
  - lifting substring runtime cache mechanics (`cache lookup` / `source liveness check` / `handle reissue`) into `.hako` or `MIR`
- current landed substring truth:
  - `str.substring.route` observe read shows `view_arc_cache_handle_hit=599,998 / total=600,000`
  - `view_arc_cache_reissue_hit=0`, `view_arc_cache_miss=2`, `fast_cache_hit=0`, `dispatch_hit=0`, `slow_plan=2`
  - current keeper removes redundant `view_enabled` state from `SubstringViewArcCache`; this cache only runs under the `view_enabled` route, so the flag compare/store was dead hot-path work
- rejected perf history:
  - exact evidence is centralized in
    `docs/development/current/main/investigations/phase137x-substring-rejected-optimizations-2026-04-08.md`
  - current rejected local cuts:
    1. broad `NyashBox` substring-source contract widening
    2. `substring_view_arc_cache_lookup` / `entry_hit` hot-path fusion
    3. birth-side second `with_handle(...)` removal via planner-local source metadata carry
    4. reissue-side slot carry / `refresh_handle` rematch removal
    5. concrete `Arc<StringViewBox>` cache carrier narrowing
- next active cut:
  - keep runtime cache mechanics unchanged
  - stay inside the dominant `view_arc_cache handle-hit` path only
  - do not widen planner / publication shape again unless counters show `miss` or `reissue` actually matter
  - revisit `len_h` only if post-substring `3 runs + perf` says it re-opened
- first files to reopen for the next slice:
  - `crates/nyash_kernel/src/exports/string_helpers.rs`
  - `crates/nyash_kernel/src/exports/string_helpers/cache.rs`
  - `crates/nyash_kernel/src/exports/string_helpers/materialize.rs`
  - `crates/nyash_kernel/src/exports/string_view.rs`
  - `crates/nyash_kernel/src/tests/string.rs`
- safe restart order:
  1. `git status -sb`
  2. `tools/checks/dev_gate.sh quick`
  3. `tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_substring_only 1 3`
  4. `tools/perf/run_kilo_hk_bench.sh strict 1 3`
  5. `tools/perf/bench_micro_aot_asm.sh kilo_micro_substring_only '' 20`
  6. `docs/development/current/main/investigations/phase137x-substring-rejected-optimizations-2026-04-08.md`
- documentation rule for failed perf cuts:
  1. keep a short current summary in the phase README
  2. keep exact rejected-cut evidence in one rolling doc per front/family/date
  3. do not create test-by-test folders unless that artifact family itself becomes an independent lane

## Order At A Glance

1. `phase-147x semantic optimization contract selection` (landed)
2. `phase-148x borrowed text and sink contract freeze` (landed)
3. `phase-149x concat const-suffix vertical slice` (landed)
4. `phase-150x array string-store vertical slice` (landed)
5. `phase-151x canonical lowering visibility lock` (landed)
6. `phase-155x perf canonical visibility tighten` (landed)
7. `phase-156x perf counter instrumentation` (landed)
8. `phase-157x observe feature split` (landed)
9. `phase-158x observe tls backend` (landed)
10. `phase-159x observe trace split` (landed)
11. `phase-160x capability-family inventory` (landed)
12. `phase-161x hot-path capability seam freeze` (landed)
13. `phase-137x main kilo reopen selection` (active)

## Current Front

- read [phase-137x/README.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phases/phase-137x/README.md) for current lane context
- read [phase137x-substring-rejected-optimizations-2026-04-08.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase137x-substring-rejected-optimizations-2026-04-08.md) before retrying any substring-local perf cut
