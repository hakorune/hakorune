---
Status: Landed
Date: 2026-05-28
Scope: deep-dive objectLifecycleSmallAlloc call/copy shape after static-scalar lowering.
Blocker: SMALL-ALLOC-CALL-COPY-SHAPE-DEEP-DIVE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-139-POST-STATIC-SCALAR-CALL-LOWERING-MEASUREMENT.md
  - docs/development/current/main/phases/phase-296x/296x-140-POST-STATIC-SCALAR-SOURCE-MIR-REFRESH.md
---

# 296x-141 Small Alloc Call/Copy Shape Deep Dive

## Purpose

Classify the remaining objectLifecycleSmallAlloc shape after static-scalar
reason calls are gone. The likely surface is compiler-lowering call/copy
materialization around facade result helpers and page hot-path calls.

This row must also keep the source-side page-local ArrayBox finding visible:
`HakoAllocPageModel.acquire/releaseLocal/resetToFresh` may dominate dynamically
even when the immediate caller surface points at `objectLifecycleSmallAlloc`.
Do not choose a keeper until call/copy materialization and dynamic page-array
weight are separated.

## Required Output

```text
output_contract=small-alloc-call-copy-shape-deep-dive-v0
input_contract=post-static-scalar-source-mir-refresh-v0
selected_owner
call_family
copy_family
page_array_dynamic_weight
selected_next
summary=ok
```

## Acceptance Notes

```text
The row should decide one of:
  - compiler_lowering: receiver/arg/return copy materialization is primary
  - allocator_page_array_surface: page-local ArrayBox get/set/reset is primary
  - benchmark_harness: reset/setup dominates the measured body

Cold fallback page scans and post-loop observers stay parked unless counters
show they execute in the exact proof workload.
```

## Partial Evidence

Helper copy-family probe:

```text
output_contract=hako-mimalloc-small-alloc-helper-copy-family-probe-v0
helper_call_count=16
helper_copy_count=62
receiver_copy_count=38
arg_copy_count=15
result_copy_count=9
local_ssa_copy_count=44
dominant_copy_family=helper_result_local_ssa
dominant_callee_family=facade_result_helpers
selected_next=same_module_helper_call_lowering_seam
summary=ok
```

Interpretation:

```text
The remaining small-alloc surface is not a broad acceptance gap. The current
evidence points to same-module facade result/state helper calls producing a
receiver + local-SSA copy chain. A source-level facade wrapper inline trial
reduced helper calls but increased MIR copy/field surface, so that path is a
non-keeper. The next compiler-side seam is same-module helper call lowering.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_small_alloc_call_copy_shape_deep_dive_guard.sh
```
