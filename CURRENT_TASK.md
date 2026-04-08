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
  - accept gate front: `kilo_micro_substring_only`
  - split exact fronts:
    - `kilo_micro_substring_views_only`
    - `kilo_micro_len_substring_views`
  - active local cut front: `kilo_micro_len_substring_views`
  - rule: WSL は `3 runs + perf` でしか delta を採らない
- current exact baseline:
  - `kilo_micro_substring_only: C 3 ms / AOT 5 ms`
  - `instr: 58,672,982`
  - `cycles: 9,979,794`
  - `cache-miss: 9,939`
  - split exact reread:
    - `kilo_micro_substring_views_only: instr=37,073,017 / cycles=6,804,272 / cache-miss=9,648 / AOT 4 ms`
    - `kilo_micro_len_substring_views: instr=22,672,209 / cycles=3,991,125 / cache-miss=8,789 / AOT 4 ms`
  - reading: mixed front の win は `substring_hii` ではなく `len_h` fast-hit 側から来ている
- target band for the next keeper:
  - mixed accept gate: `instr <= 58.5M`
  - local split `kilo_micro_len_substring_views`: `instr <= 22.4M`
  - control split `kilo_micro_substring_views_only`: roughly flat is acceptable
  - whole strict: hold `<= 755 ms`; ideal band is `730-745 ms`
- ideal `len_h` steady-state asm shape:
  - `STRING_DISPATCH_STATE` load once
  - `handles::drop_epoch()` load once
  - primary/secondary handle compare only
  - `JIT_TRACE_LEN_ENABLED_CACHE` load once
  - trace-off fast hit returns directly without carrying extra cold work inline
- current whole-kilo health:
  - `tools/checks/dev_gate.sh quick` is green
  - `kilo_kernel_small_hk` strict latest accepted reread: `ny_aot_ms=755`
  - parity: `vm_result=1140576`, `aot_result=1140576`
- do not reopen:
  - `OwnedText` backing for this lane
  - live-source direct-read widening on `as_str_fast()`
  - global `dispatch` / `trace` false-state fast probes outside `string_len_export_impl()`
  - lifting substring runtime cache mechanics (`cache lookup` / `source liveness check` / `handle reissue`) into `.hako` or `MIR`
- current landed substring truth:
  - `str.substring.route` observe read shows `view_arc_cache_handle_hit=599,998 / total=600,000`
  - `view_arc_cache_reissue_hit=0`, `view_arc_cache_miss=2`, `fast_cache_hit=0`, `dispatch_hit=0`, `slow_plan=2`
  - current keeper removes redundant `view_enabled` state from `SubstringViewArcCache`; this cache only runs under the `view_enabled` route, so the flag compare/store was dead hot-path work
  - split exact fronts show `substring_hii` steady-state retained-view path is roughly unchanged at `37.07M instr`
  - current keeper is on `len_h`: `string_len_fast_cache_lookup()` now hoists one `handles::drop_epoch()` read and reuses it across primary/secondary slot checks
  - current keeper also keeps the `len_h` fast-hit return thin: `string_len_export_impl()` now tail-calls a tiny helper so trace-off steady state returns `cached` without carrying `trace_len_fast_hit(...)` inline
- rejected perf history:
  - exact evidence is centralized in
    `docs/development/current/main/investigations/phase137x-substring-rejected-optimizations-2026-04-08.md`
  - current rejected local cuts:
    1. broad `NyashBox` substring-source contract widening
    2. `substring_view_arc_cache_lookup` / `entry_hit` hot-path fusion
    3. birth-side second `with_handle(...)` removal via planner-local source metadata carry
    4. reissue-side slot carry / `refresh_handle` rematch removal
    5. concrete `Arc<StringViewBox>` cache carrier narrowing
    6. `len_h` cache-first reorder
    7. `drop_epoch_if_ready()` fast accessor probe
    8. global `dispatch` / `trace` false-state fast probes
    9. `len_h` dispatch-hit cold split
    10. `trace_len_state()` helper / trace cache single-load probe
    11. `len_h` two-slot pre-match + single epoch-guard probe
    12. local `dispatch_known_absent_fast` + cold dispatch probe combo
    13. `drop_epoch_after_cache_hit()` ready-after-hit probe
    14. `len_h` dispatch single-probe + raw trace-state split
    15. `len_h` 1-probe hash-slot cache shape
    16. registry-pointer epoch read on len cache hits
- next active cut:
  - keep `kilo_micro_substring_only` as accept gate
  - use `kilo_micro_len_substring_views` for local `len_h` cuts
  - keep `substring_hii` runtime cache mechanics unchanged unless split fronts move again
  - helper/state rewrites and cache-shape rewrites did not change emitted `len_h` hot asm enough
  - next likely touch points are `len_h` prologue/`REG` ready probe cost and any cut that can prove an asm-visible hot-block change before retrying
- first files to reopen for the next slice:
  - `crates/nyash_kernel/src/exports/string_helpers/cache.rs`
  - `crates/nyash_kernel/src/exports/string_helpers.rs`
  - `crates/nyash_kernel/src/exports/string_helpers/materialize.rs`
  - `crates/nyash_kernel/src/exports/string_debug.rs`
  - `crates/nyash_kernel/src/hako_forward_bridge.rs`
  - `crates/nyash_kernel/src/tests/string.rs`
- safe restart order:
  1. `git status -sb`
  2. `tools/checks/dev_gate.sh quick`
  3. `tools/perf/run_kilo_string_split_pack.sh 1 3`
  4. `tools/perf/bench_micro_aot_asm.sh kilo_micro_len_substring_views 'nyash.string.len_h' 20`
  5. `docs/development/current/main/investigations/phase137x-substring-rejected-optimizations-2026-04-08.md`
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
