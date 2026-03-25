---
Status: Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P8/P9` close 後の最初の perf re-entry を small-entry lane だけに固定し、`kilo` 全体へ広げる前の exact front を narrow に保つ。
Related:
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P8-PERF-REOPEN-JUDGMENT.md
  - docs/development/current/main/phases/phase-29ck/P9-METHOD-CALL-ONLY-PERF-ENTRY-INVENTORY.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
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

## Green Entry Evidence

1. `PERF_AOT=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_compare_c_vs_hako.sh method_call_only_small 1 1`
   - current result: `aot_status=ok`
2. `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_loop_integer_hotspot_contract_vm.sh`
   - green
3. `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_strlen_ir_contract_vm.sh`
   - green
4. `bash tools/checks/dev_gate.sh quick`
   - green

## Next Exact Front

1. record/update small baseline only
   - `method_call_only_small`
   - `box_create_destroy_small`
2. identify the first hot leaf from the small lane
3. do not reopen `kilo` medium/full or unrelated micro packs in the same series

## Non-Goals

- immediate `kilo` medium/full reopen
- `substring_concat` / `array_getset` / allocator leaf edits
- `llvmlite` keep-lane widening
- boundary recipe widening beyond the retired `method_call_only` family

## Exit Condition

- small-entry baseline is current and reproducible
- first exact perf leaf is named
- the next code bucket can be cut without reopening boundary acceptance work
