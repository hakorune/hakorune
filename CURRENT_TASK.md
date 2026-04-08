# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-09
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
- runtime-wide pattern anchor:
  - `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
- current string corridor design anchor:
  - `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
- active lane/front:
  - lane: `phase-137x main kilo reopen selection`
  - accept gate front: `kilo_micro_substring_only`
  - split exact fronts:
    - `kilo_micro_substring_views_only`
    - `kilo_micro_len_substring_views`
  - active local cut front: `kilo_micro_substring_views_only`
  - pure Rust reference compare lane:
    - `benchmarks/rust/bench_kilo_micro_substring_views_only.rs`
    - `tools/perf/bench_rust_vs_hako_stat.sh kilo_micro_substring_views_only 1 3`
    - latest pure Rust reference: `instr=5,667,104 / cycles=1,572,750 / cache-miss=5,254 / ms=3`
    - latest C-like Rust reference: `instr=12,566,914 / cycles=3,404,383 / cache-miss=5,256 / ms=3`
  - rule: WSL は `3 runs + perf` でしか delta を採らない
- current exact baseline:
  - `kilo_micro_substring_only: C 3 ms / AOT 5 ms`
  - `instr: 49,372,458`
  - `cycles: 8,539,682`
  - `cache-miss: 9,294`
  - split exact reread:
    - `kilo_micro_substring_views_only: instr=34,373,156 / cycles=6,421,062 / cache-miss=9,550 / AOT 5 ms`
    - `kilo_micro_len_substring_views: instr=16,073,034 / cycles=4,347,479 / cache-miss=8,958 / AOT 4 ms`
  - reading: latest keeper came from `len_h`, and the split pair now says `substring_hii` is first target again
- target band for the next keeper:
  - mixed accept gate: `instr <= 49.1M`
  - local split `kilo_micro_substring_views_only`: `instr <= 34.2M`
  - control split `kilo_micro_len_substring_views`: roughly flat is acceptable
  - whole strict: hold `<= 709 ms`; ideal band is `690-705 ms`
- ideal `len_h` steady-state asm shape:
  - direct `STRING_DISPATCH_FN` load once; do not carry the `STRING_DISPATCH_STATE` state machine in `nyash.string.len_h`
  - direct `host_handles::DROP_EPOCH` load once
  - primary/secondary handle compare only
  - `JIT_TRACE_LEN_ENABLED_CACHE` load once with cold init off the hot return path
  - trace-off fast hit returns directly without carrying extra cold work inline
- current whole-kilo health:
  - `tools/checks/dev_gate.sh quick` is green
  - `kilo_kernel_small_hk` strict latest accepted reread: `ny_aot_ms=709`
  - parity: `vm_result=1140576`, `aot_result=1140576`
- do not reopen:
  - `OwnedText` backing for this lane
  - live-source direct-read widening on `as_str_fast()`
  - global `dispatch` / `trace` false-state fast probes outside `string_len_export_impl()`
  - lifting substring runtime cache mechanics (`cache lookup` / `source liveness check` / `handle reissue`) into `.hako` or `MIR`
  - widening `@rune` beyond declaration-local metadata for this lane
  - generic scalar/cache/route frameworks before a second keeper lane proves the same invariant
- current landed substring truth:
  - `str.substring.route` observe read shows `view_arc_cache_handle_hit=599,998 / total=600,000`
  - `view_arc_cache_reissue_hit=0`, `view_arc_cache_miss=2`, `fast_cache_hit=0`, `dispatch_hit=0`, `slow_plan=2`
  - current keeper removes redundant `view_enabled` state from `SubstringViewArcCache`; this cache only runs under the `view_enabled` route, so the flag compare/store was dead hot-path work
  - split exact fronts now put `substring_hii` retained-view path at `34.37M instr`
  - `2026-04-09` perf reread on `kilo_micro_substring_views_only`:
    - exact: `instr=34,372,749 / cycles=6,415,829 / cache-miss=8,601 / AOT 4 ms`
    - top: `nyash.string.substring_hii 85.99%`, `ny_main 7.30%`
    - annotate reading:
      1. first hot cluster is `SUBSTRING_ROUTE_POLICY_CACHE` load/decode
      2. second hot cluster is `substring` provider state read + `SUBSTRING_VIEW_ARC_CACHE` TLS entry/state check
      3. only then `SubstringViewArcCache` steady-state compare path
      4. slow plan / materialize is not the dominant cost on this front
  - latest baseline asm reread says the next visible tax is still before the view-arc cache compare block:
    1. `SUBSTRING_ROUTE_POLICY_CACHE` decode
    2. `substring_view_enabled` / fallback provider state reads
    3. only then `SubstringViewArcCache` steady-state compare path
  - current keeper is on `len_h`: `string_len_fast_cache_lookup()` now hoists one `handles::drop_epoch()` read and reuses it across primary/secondary slot checks
  - current keeper also keeps the `len_h` fast-hit return thin: `string_len_export_impl()` now tail-calls a tiny helper so trace-off steady state returns `cached` without carrying `trace_len_fast_hit(...)` inline
  - current keeper removes the `STRING_DISPATCH_STATE` state machine from emitted `nyash.string.len_h`; the hot entry now probes `STRING_DISPATCH_FN` directly once
  - current keeper also splits trace state into `jit_trace_len_state_raw()` and cold `jit_trace_len_state_init()`, so the hot cache-hit path sees one trace-state load and returns directly when trace is off
  - current keeper also lands the `drop_epoch()` global mirror: emitted `nyash.string.len_h` now reads `host_handles::DROP_EPOCH` directly and no longer carries the `host_handles::REG` ready probe / `OnceCell` path
  - latest split exact reread moves first priority back to `substring_hii`; `len_h` now reads as the secondary control split
  - pure Rust reference is the current lower bound for this front; current AOT is about `6.06x instr / 4.10x cycles` over it
  - C-like Rust reference is the current contract-aligned comparison point; current AOT is about `2.73x instr / 1.91x cycles` over it
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
    17. `len_h` `ReadOnlyScalarLane` separation-only slice
    18. `len_h` combined `ReadOnlyScalarLane` + entry snapshot slice
    19. `len_h`-specific 4-box slice (`façade + control snapshot + pure cache probe + cold path`)
    20. `SubstringViewArcCache` global compare reorder (`start/end` before `source_handle`)
    21. `SubstringViewArcCache` `same_source_pair` specialization
    22. `substring_hii` common-case body duplication via `route_raw == 0b111`
    23. `substring` provider `raw read + cold init` adoption (`substring_view_enabled` / fallback policy / route policy)
    24. `substring_route_policy()` cold init split while keeping the active caller shape unchanged
    25. `substring_hii` route/provider snapshot + eager `DROP_EPOCH` snapshot
    26. `SubstringViewArcCache::entry_hit` reissue/clear cold split
- next active cut:
  - keep `kilo_micro_substring_only` as accept gate
  - use `kilo_micro_substring_views_only` for local `substring_hii` cuts
  - keep `len_h` runtime mechanics stable unless split fronts move again
  - latest keeper eliminated the remaining `len_h` control-plane hot loads; do not reopen `len_h` local cuts until `substring` is re-read
  - active design rule:
    - stop treating the next move as another `substring_hii` leaf/provider/cache split
    - current upstream design is:
      - `.hako policy -> canonical MIR facts -> placement/effect pass -> Rust microkernel -> LLVM`
    - do not add a permanent second public MIR dialect
    - do not widen current `@rune` surface for boundary/cache/provider mechanics
  - task order is fixed:
    1. docs-first: treat `string-canonical-mir-corridor-and-placement-pass-ssot.md` as the active design owner for this wave
    2. landed: MIR-side inventory for `str.slice` / `str.len` / `freeze.str` now lives in `src/mir/string_corridor.rs`; refresh is behavior-preserving
    3. landed: canonical MIR fact carrier is `FunctionMetadata.string_corridor_facts`; verbose dumps expose the facts
    4. landed: placement/effect scaffold is now `src/mir/string_corridor_placement.rs`; it reads `FunctionMetadata.string_corridor_facts`, emits no-op candidate decisions into `FunctionMetadata.string_corridor_candidates`, and leaves runtime lowering unchanged
    5. next slice is the first real borrowed-corridor sinking pilot; prefer the narrowest internal `str.slice -> str.len` style corridor
    6. fifth slice is AOT-internal direct kernel entry selection; ABI/FFI keeps the facade
    7. only after the upstream corridor slices land and move exact/asm, reopen new `substring_hii` runtime leaf cuts
    8. keep the cross-lane scope-control table in `string-canonical-mir-corridor-and-placement-pass-ssot.md` truthful; do not let the `string` pilot silently redefine `array/map` or ABI structure
    9. do not retry `len_lane` separation by itself; both separation-only and combined snapshot retries failed keeper gates
    10. the earlier `drop_epoch()` global mirror rejection was invalidated by stale release artifacts; the hypothesis is now landed, and future perf reads must rebuild release artifacts first
    11. do not retry the same `len_h`-specific 4-box slice as-is; it lost before the control-plane fixes landed
    12. `len_h` の箱が当たるまで generic framework にはしない; reusable abstraction は後回し
    13. do not genericize implementation from `string` alone; first collect keeper patterns in the runtime-hot-lane pattern SSOT
    14. hot caller での `substring` provider swap は 1 本では keep しない:
       `substring_view_enabled` / fallback policy / route policy を同時に `raw read + cold init` へ切り替える slice は local front を落とした
    15. shape cleanup では hot body duplication をしない; `route_raw == common-case` の全文複製は reopen しない
    16. next shape cleanup must stay below the active caller or pair with an asm-visible win; provider foundation only is allowed, but hot caller adoption needs proof
    17. `substring_route_policy()` cold split alone is also blocked; even without caller adoption it lost the local split
    18. if a future slice reopens `len_h`, it must beat the new `DROP_EPOCH`-based asm and preserve direct dispatch / single trace-state loads
    19. do not retry the same `substring_hii` route/provider snapshot with eager `DROP_EPOCH` capture; it regressed both exact fronts and whole strict before any cache-entry win appeared
    20. do not cold-split `SubstringViewArcCache::entry_hit` reissue/clear path in isolation; the call boundary/code layout regressed all split fronts badly
- first files to reopen for the next slice:
  - `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
  - `src/mir/string_corridor.rs`
  - `src/mir/string_corridor_placement.rs`
  - `crates/hakorune_mir_core/src/effect.rs`
  - `crates/hakorune_mir_defs/src/call_unified.rs`
  - `src/mir/**`
  - `crates/nyash_kernel/src/exports/string_helpers.rs`
  - `crates/nyash_kernel/src/exports/string_helpers/cache.rs`
  - `crates/nyash_kernel/src/exports/string_debug.rs`
  - `crates/nyash_kernel/src/hako_forward_bridge.rs`
  - `crates/nyash_kernel/src/exports/string_helpers/materialize.rs`
  - `crates/nyash_kernel/src/exports/string_view.rs`
  - `crates/nyash_kernel/src/tests/string.rs`
- safe restart order:
  1. `git status -sb`
  2. `tools/checks/dev_gate.sh quick`
  3. `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
  4. `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
  5. after any `nyash_kernel` / `hakorune` runtime source edit, rerun `bash tools/perf/build_perf_release.sh` before exact micro / asm probes
  6. `tools/perf/run_kilo_string_split_pack.sh 1 3`
  7. `tools/perf/bench_micro_aot_asm.sh kilo_micro_substring_views_only 'nyash.string.substring_hii' 200`
  8. `docs/development/current/main/investigations/phase137x-substring-rejected-optimizations-2026-04-08.md`
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
