---
Status: Closed Inventory
Decision: accepted
Date: 2026-03-26
Scope: `phase-21_5` / `kilo` reopen blocker のうち `method_call_only` family だけを narrow inventory として固定し、next exact code bucket を 1 family に絞る。
Related:
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P8-PERF-REOPEN-JUDGMENT.md
  - docs/development/current/main/phases/phase-29ck/P10-SMALL-PERF-REENTRY-TASK-PACK.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/design/backend-recipe-route-profile-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - benchmarks/bench_method_call_only.hako
  - benchmarks/bench_box_create_destroy.hako
  - apps/tests/mir_shape_guard/method_call_only_small.prebuilt.mir.json
---

# P9: `method_call_only` Perf-Entry Inventory

## Purpose

- `perf/kilo` reopen blocker を 1 family に絞る。
- `method_call_only_small` prebuilt と `bench_method_call_only.hako` emit の差を、perf retune ではなく boundary acceptance inventory として読む。
- `box_create_destroy` は control として残し、同じ patch に混ぜない。

## Fixed Inputs

### Family under investigation

1. prebuilt keep/input
   - `apps/tests/mir_shape_guard/method_call_only_small.prebuilt.mir.json`
2. emitted bench input
   - `benchmarks/bench_method_call_only.hako`

### Control

1. emitted control bench
   - `benchmarks/bench_box_create_destroy.hako`

## Closed Evidence

1. `PERF_AOT=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_compare_c_vs_hako.sh method_call_only_small 1 1`
   - current result: `aot_status=ok`
2. `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_loop_integer_hotspot_contract_vm.sh`
   - green
3. `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_strlen_ir_contract_vm.sh`
   - green
4. boundary controls remain green
   - `phase29ck_boundary_pure_first_min.sh`
   - `phase29ck_boundary_compat_keep_min.sh`
   - `phase29ck_llvm_backend_box_capi_link_min.sh`

読み:
- blocker は perf tuning ではなく boundary-side acceptance coverage にあった。
- prebuilt / emitted full / emitted small / control の current `method_call_only` family は pure-first acceptance に収まった。

## Inventory Questions

この front で答える質問は 3 つだけだよ。

1. `method_call_only_small.prebuilt.mir.json` は current pure-first lane でどこまで通るか。
2. `bench_method_call_only.hako` から emit された MIR は、prebuilt small とどこが違うか。
3. `bench_box_create_destroy.hako` は control として pure-first lane でどこまで separate に保てるか。

## Expected Owners

- first owner:
  - `lang/src/shared/backend/backend_recipe_box.hako`
  - `lang/c-abi/shims/hako_llvmc_ffi.c`
  - `crates/nyash-llvm-compiler/src/boundary_driver_ffi.rs`
- keep-only:
  - `src/llvm_py/**`
  - `tools/llvmlite_harness.py`
  - `phase-21_5` perf retune docs / asm-led leaf edits

## Non-Goals

- `kilo` / `micro kilo` retune
- asm top revisit
- `substring_concat` / `array_getset` / allocator leaf edits
- `llvmlite` keep-lane widening

## Acceptance For This Inventory

- the repo docs can name the exact `method_call_only` inputs and the control
- prebuilt / emitted full / emitted small / control are all reproducible
- next exact code bucket moves to `P10` small perf re-entry

## Exit Condition

この inventory は close したよ。

- next exact code bucket:
  - `P10-SMALL-PERF-REENTRY-TASK-PACK.md`
- `box_create_destroy` は control のまま据え置く
- unrelated perf leaves は引き続き混ぜない
