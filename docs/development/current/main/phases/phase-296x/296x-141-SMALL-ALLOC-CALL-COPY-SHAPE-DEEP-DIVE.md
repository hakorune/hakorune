---
Status: Current
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
