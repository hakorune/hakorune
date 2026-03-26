---
Status: Task Pack
Decision: accepted
Date: 2026-03-26
Scope: perf/mainline owner を `Stage1 = ny-llvmc(boundary pure-first)` に固定し、`llvmlite` compat replay 混入を fail-fast にする。
Related:
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P13-SMALL-ENTRY-RAW-NET-REFRESH.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/design/ai-handoff-and-debug-contract.md
  - docs/reference/environment-variables.md
  - crates/nyash-llvm-compiler/src/boundary_driver_ffi.rs
  - lang/c-abi/shims/hako_llvmc_ffi.c
  - tools/ny_mir_builder.sh
  - tools/perf/lib/aot_helpers.sh
  - tools/perf/bench_compare_c_py_vs_hako.sh
---

# P14: Pure-First No-Replay Cutover

## Purpose

- `kilo` 最適化を続ける前に、owner を固定する。
- `llvmlite` は `Stage0` explicit keep lane に留める。
- `ny-llvmc(boundary pure-first)` を `Stage1` daily/mainline/perf owner に固定する。
- perf success が internal harness replay を隠さないようにする。

## Current Truth

1. route trace is now visible.
   - `NYASH_LLVM_ROUTE_TRACE=1` enables
     - `[llvm-route/select] owner=boundary recipe=<...> compat_replay=<...> symbol=<...>`
     - `[llvm-route/replay] lane=<none|harness> reason=<...>`
2. perf AOT lane is now contract-locked.
   - `tools/perf/lib/aot_helpers.sh` pins
     - `HAKO_BACKEND_COMPILE_RECIPE=pure-first`
     - `HAKO_BACKEND_COMPAT_REPLAY=none`
   - missing route trace, replay hit, or non-`none` compat policy are fail-fast.
3. wrapper trace passthrough is now preserved.
   - `tools/ny_mir_builder.sh` no longer suppresses backend diagnostics when `NYASH_LLVM_ROUTE_TRACE=1`.
4. current `kilo` blocker is now explicit.
   - replay-disabled `kilo_kernel_small_hk` still fails with `unsupported pure shape for current backend recipe`
   - the active owner is `lang/c-abi/shims/hako_llvmc_ffi.c`, not `src/llvm_py/**`

## Exact Blocker Family

1. generic pure-first coverage still lacks the first `kilo` family
   - `copy`
   - `newbox ArrayBox`
   - string-loop `RuntimeDataBox.length/get/set/substring/indexOf`
   - string `binop +`
2. until that family is accepted in pure-first, asm-guided perf edits stay blocked

## Acceptance

- `cargo test -p nyash-llvm-compiler boundary_driver_ -- --nocapture`
- `bash tools/checks/dev_gate.sh quick`
- `HAKO_BACKEND_COMPILE_RECIPE=pure-first HAKO_BACKEND_COMPAT_REPLAY=none NYASH_LLVM_ROUTE_TRACE=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/ny_mir_builder.sh --in /tmp/kilo_kernel_small.mir.json --emit exe -o /tmp/kilo_kernel_small.exe --quiet`
- `PERF_VM_FORCE_NO_FALLBACK=1 NYASH_LLVM_SKIP_BUILD=0 bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small_hk 1 1`

## Non-Goals

- `src/llvm_py/**` perf leaf optimization
- broad wrapper deletion
- `llvmlite` keep lane removal
- pure-first widening beyond the first exact unsupported `kilo` shape family

## Exit Condition

- perf/mainline owner is visible and replay cannot hide inside a green run
- next exact front is pure-first generic coverage in `hako_llvmc_ffi.c`
