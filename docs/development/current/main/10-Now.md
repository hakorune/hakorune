---
Status: SSOT
Date: 2026-04-06
Scope: current lane / blocker / next pointer だけを置く薄い mirror。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
---

# Self Current Task — Now (main)

## Current

- lane: `phase-137x main kilo reopen selection`
- current front: `store.array.str` を first exact front に据えて executor overhead を削る
- blocker: exact micro と whole-kilo を同時に良化する patch だけを採る
- first landed slice:
  - `tools/selfhost/lib/selfhost_build_exe.sh` no longer forces harness on the daily EXE lane
  - provider/selfhost docs now read llvmlite as explicit keep only
  - `tools/build_llvm.sh` harness keep now routes through `ny-llvmc --driver harness`
  - `tools/llvm_smoke.sh` is explicit compat/probe keep
  - WSL EXE-first and selfhost pilot guides now treat llvmlite as keep-only
  - public env reference labels `NYASH_LLVM_USE_HARNESS=1` examples as explicit keep-lane
- perf reopen front:
  - `store.array.str` -> `array_string_store_handle_at(...)`
  - `const_suffix` / `thaw.str + lit.str + str.concat2 + freeze.str` -> `concat_const_suffix_fallback(...)`
- observe lane:
  - `--features perf-observe`
  - `NYASH_PERF_COUNTERS=1`
  - TLS exact counter backend
  - `--features perf-trace`
  - `NYASH_PERF_TRACE=1`
  - trace lane is now parked placeholder
  - contract identity:
    - `store.array.str`
    - `const_suffix`
- latest bundle anchor:
  - `target/trace_logs/kilo-string-trace-asm/20260406-024104/summary.txt`
  - `target/trace_logs/kilo-string-trace-asm/20260406-024104/asm/perf_report.txt`
- recent landed:
  - `phase-140x map owner pilot`
  - `phase-139x array owner pilot`
  - `phase-138x nyash_kernel semantic owner cutover`
  - `phase-134x nyash_kernel layer recut selection`
  - `phase-133x micro kilo reopen selection`

## Current Read

- `vm` cleanup is no longer current work
- fixed perf order stays:
  - `leaf-proof micro`
  - `micro kilo`
  - `main kilo`
- `phase-133x` is closed:
  - `kilo_micro_substring_concat`: parity locked
  - `kilo_micro_array_getset`: parity locked
  - `kilo_micro_indexof_line`: frozen faster than C
- `phase-134x` re-cut `nyash_kernel` into four buckets:
  - `keep`
  - `thin keep`
  - `compat glue`
  - `substrate candidate`
- landed source slices:
  - `crates/nyash_kernel/src/exports/string.rs` split
  - `crates/nyash_kernel/src/plugin/map_substrate.rs` thin-alias recut
- current architecture target is fixed:
  - `Rust host microkernel`
  - `.hako semantic kernel`
  - `native accelerators`
  - `ABI facade` as thin keep
  - `compat quarantine` as non-owner
- landed final string seam:
  - semantic owner: `runtime/kernel/string/**`
  - VM-facing wrapper: `string_core_box.hako`
  - thin facade: `string.rs`
  - lifetime/native substrate: `string_view.rs` / `string_helpers.rs` / `string_plan.rs`
  - quarantine: `module_string_dispatch/**`
- current architecture follow-up is implementation-first:
  - `phase-142x` = landed Array owner cutover implementation
  - `phase-143x` = landed Map owner cutover implementation
  - `phase-144x` = landed String semantic owner follow-up
- current cleanup lane:
  - `phase-145x` = landed compat quarantine shrink
  - `phase-146x` = landed string semantic boundary tighten
- current optimization authority lock:
  - `.hako` owns route / retained-form / boundary
  - MIR owns canonical substrate contract
  - Rust owns executor / accelerator only
- landed contract freeze:
  - `const_suffix -> thaw.str + lit.str + str.concat2 + freeze.str`
  - `ArrayStoreString -> store.array.str`
  - `MapStoreAny -> store.map.value`
- landed first consumer:
  - `const_suffix` current lowering now reads as executor detail under the canonical contract
- landed second consumer:
  - `ArrayStoreString` current lowering now reads as ABI/executor detail under canonical `store.array.str`
- landed visibility lock:
  - `const_suffix`, `ArrayStoreString`, `MapStoreAny` all read through owner -> canonical -> concrete lowering -> executor
- current stop-line:
  - observer stays compile-out by default and feature-on by choice
  - observer must not look like a fifth authority layer
  - exact counter backend must not keep shared atomic cost on the hot path
  - heavy trace must not piggyback on exact counter backend or sink
- perf lane is active again:
  - capability lock is landed:
    - `phase-160x capability-family inventory`
    - `phase-161x hot-path capability seam freeze`
  - current perf truth:
    - whole `kilo_kernel_small_hk = 741ms`
    - exact micro `kilo_micro_concat_const_suffix = 84ms`
    - exact micro `kilo_micro_array_string_store = 181ms`

## Root Anchors

- root anchor: `CURRENT_TASK.md`
- quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- one-screen map: `docs/development/current/main/15-Workstream-Map.md`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/design/semantic-optimization-authority-ssot.md`
3. `docs/development/current/main/phases/phase-137x/README.md`
4. `docs/development/current/main/phases/phase-137x/README.md`
5. `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
