---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P12` close 後の small-entry lane を raw/net numbers でもう一度固定し、small-entry perf lane を monitor-only close できるかを判断する。
Related:
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P12-SMALL-ENTRY-GC-SECTIONS-CANDIDATE.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - benchmarks/baselines/method_call_only_small.latest.json
  - benchmarks/baselines/box_create_destroy_small.latest.json
  - tools/archive/legacy-selfhost/engineering/phase29ck_small_entry_startup_probe.sh
  - tools/perf/bench_compare_c_vs_hako.sh
---

# P13: Small-Entry Raw/Net Refresh

## Purpose

- `P12` で boundary mainline link trim は landed した。
- 次は small-entry lane を raw wallclock と startup-subtracted numbers で refresh し、current lane を monitor-only close できるかを判断する。
- runtime leaf / medium-full `kilo` widening / keep-lane editsへ逸れない。

## Fixed Inputs

1. target
   - `method_call_only_small`
2. control
   - `box_create_destroy_small`

## Current Truth

1. boundary mainline exe shape is trimmed.
   - `method_call_only_small` mainline exe:
     - file size `5,375,880`
     - relocation count `61`
2. startup-subtracted lane is already near floor.
   - `method_call_only_small`: `c_ms=2`, `ny_aot_ms=1`
   - `box_create_destroy_small`: `c_ms=2`, `ny_aot_ms=0`
3. raw short-run lane is still startup-dominated enough that boundary runtime leaf tuning is not justified yet.

## Landed Evidence

1. raw 1x1 evidence is refreshed.
   - `method_call_only_small`: `c_ms=3`, `ny_aot_ms=9`
   - `box_create_destroy_small`: `c_ms=3`, `ny_aot_ms=8`
2. startup-subtracted evidence is refreshed.
   - `method_call_only_small`: `c_ms=2`, `ny_aot_ms=1`
   - `box_create_destroy_small`: `c_ms=2`, `ny_aot_ms=0`
3. judgment:
   - small-entry lane still reads as startup-dominated
   - there is no fresh exact runtime leaf to cut in this lane
   - current baseline files do not need a new rewrite for this slice

## Follow-up

1. small-entry perf lane is now `none (monitor-only)`
2. phase-level next exact front returns to runtime proof blocker inventory
   - acceptance anchor:
     - `tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh`

## Acceptance

- `bash tools/archive/legacy-selfhost/engineering/phase29ck_small_entry_startup_probe.sh`
- `PERF_AOT=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_compare_c_vs_hako.sh method_call_only_small 1 1`
- `PERF_AOT=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_compare_c_vs_hako.sh box_create_destroy_small 1 1`
- `PERF_SUBTRACT_STARTUP=1 PERF_AOT=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_compare_c_vs_hako.sh method_call_only_small 2 5`
- `PERF_SUBTRACT_STARTUP=1 PERF_AOT=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_compare_c_vs_hako.sh box_create_destroy_small 2 5`
- `bash tools/checks/dev_gate.sh quick`

## Non-Goals

- medium/full `kilo` widening
- runtime string/box leaf edits
- `llvmlite` keep-lane edits
- `link_driver.rs` mirror parity work

## Exit Condition

- post-trim raw/net figures are refreshed
- small-entry perf lane is closed monitor-only
- next phase front is no longer a small-entry perf edit
