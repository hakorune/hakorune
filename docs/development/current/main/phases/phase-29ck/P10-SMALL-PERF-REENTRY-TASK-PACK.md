---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P8/P9` close 後の最初の perf re-entry を small-entry lane だけに固定し、`kilo` 全体へ広げる前の exact front を narrow に保つ。
Related:
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P8-PERF-REOPEN-JUDGMENT.md
  - docs/development/current/main/phases/phase-29ck/P9-METHOD-CALL-ONLY-PERF-ENTRY-INVENTORY.md
  - docs/development/current/main/phases/phase-29ck/P11-SMALL-ENTRY-STARTUP-INVENTORY.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - tools/dev/phase29ck_small_entry_startup_probe.sh
---

# P10: Small Perf Re-entry

## Purpose

- pre-perf blocker が retired したあと、perf lane をいきなり `kilo` 全体へ widen しない。
- first reopen front は `small` entry だけに固定する。
- current exact question は「どの hot leaf が最初の owner か」を small baseline から切ること。

## Fixed Inputs

1. target
   - `method_call_only_small`
2. control
   - `box_create_destroy_small`

## Closed Entry Evidence

1. `PERF_AOT=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_compare_c_vs_hako.sh method_call_only_small 1 1`
   - current result: `aot_status=ok`
2. `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_loop_integer_hotspot_contract_vm.sh`
   - green
3. `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_strlen_ir_contract_vm.sh`
   - green
4. `bash tools/checks/dev_gate.sh quick`
   - green

## Closed Baseline Refresh

1. `benchmarks/baselines/method_call_only_small.latest.json`
   - `c_ms=3`
   - `py_ms=12`
   - `ny_vm_ms=9`
   - `ny_aot_ms=8`
2. `benchmarks/baselines/box_create_destroy_small.latest.json`
   - `c_ms=3`
   - `py_ms=12`
   - `ny_vm_ms=10`
   - `ny_aot_ms=8`

## Current Truth

1. `method_call_only_small` dumped AOT IR is now a pure loop.
   - body shape is `add i64 %acc.cur, 5`
   - no `nyash.string.len_h`
   - no `nyrt_string_length`
   - no `nyash.any.length_h`
2. `box_create_destroy_small` dumped AOT IR is also a pure loop.
   - body shape is `add i64 %acc.cur, 1`
   - no `nyash.box.from_i8_string`
   - no `nyash.string.len_h`
   - no `nyrt_string_length`
3. current small-entry AOT executables are dynamically linked and not stripped.
4. current short microasm is loader/startup dominated, not runtime-leaf dominated.

読み:
- current small-entry lane no longer exposes a live runtime string/box leaf.
- the next adjacent front is startup/loader inventory, not `string.len` or `newbox` retune.

## Non-Goals

- immediate `kilo` medium/full reopen
- `substring_concat` / `array_getset` / allocator leaf edits
- `llvmlite` keep-lane widening
- boundary recipe widening beyond the retired `method_call_only` family

## Exit Condition

この task pack は close したよ。

- small-entry baseline is current and reproducible
- current small-entry lane is named as pure-loop + startup-dominated
- next exact code bucket moved to `P11-SMALL-ENTRY-STARTUP-INVENTORY.md`
