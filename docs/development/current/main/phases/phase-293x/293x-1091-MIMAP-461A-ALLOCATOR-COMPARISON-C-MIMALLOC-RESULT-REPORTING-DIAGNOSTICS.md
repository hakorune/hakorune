# 293x-1091 MIMAP-461A Allocator Comparison C Mimalloc Result Reporting Diagnostics

Status: landed
Date: 2026-05-21

## Purpose

Add observer-only diagnostics over the MIMAP-460A C-vs-Hako result reporting
inventory.

This row should classify whether the reporting inventory is accepted or blocked
before a later reporting closeout / presentation row. It must not rerun
benchmarks and must not turn scalar evidence into a performance or memory-use
conclusion.

## Scope

- Consume the MIMAP-460A reporting inventory report.
- Classify accepted / blocked reporting inventory rows.
- Preserve the existing availability, evidence, delta, and closed stop-line
  fields.
- Publish scalar diagnostic counters for missing reporting inventory and blocked
  reporting inventory.
- Keep this as diagnostics, not a final report or decision row.

## Stop Lines

- No repeated or heavy benchmark pack.
- No performance conclusion.
- No memory-use conclusion.
- No process allocator replacement.
- No hook installation.
- No backend matcher additions.
- No `#[global_allocator]`.
- No provider package / DLL generation.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No worker/thread execution.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Validation profile: `scalar-mir`.

Validated:

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_reporting_diagnostics_guard.sh --level L2
```

## Task Order

1. Add the reporting diagnostics owner and proof app.
2. Add a focused L2 guard consuming the MIMAP-460A reporting inventory report.
3. Keep benchmark execution and final reporting / presentation conclusions
   closed.
4. Select a reporting closeout / presentation-preflight row only after the
   diagnostics guard is green.

## Completed

- Added `HakoAllocAllocatorComparisonCMimallocResultReportingDiagnostic` as an
  observer-only diagnostic owner over the MIMAP-460A reporting inventory report.
- Added a manifest-backed proof app and focused L2 guard.
- Preserved the closed stop lines for benchmark reruns, performance/memory-use
  conclusions, allocator replacement, hooks, backend matcher additions, global
  allocator installation, provider package generation, hidden discovery,
  worker/thread execution, and cross-function `Result` direct ABI.
- Selected MIMAP-462A as the reporting closeout row.

## Next

MIMAP-462A should close the C mimalloc result reporting inventory / diagnostics
pack at L2 before opening the presentation / decision row selection.
