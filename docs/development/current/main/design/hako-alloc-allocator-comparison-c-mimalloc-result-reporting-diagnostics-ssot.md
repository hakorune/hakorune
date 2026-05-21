# Hako Alloc Allocator Comparison C Mimalloc Result Reporting Diagnostics SSOT

Status: accepted
Decision: accepted
Date: 2026-05-21
Owner: MIMAP-461A

## Decision: accepted

MIMAP-461A adds observer-only diagnostics over the MIMAP-460A C-vs-Hako result
reporting inventory.

The diagnostics classify accepted / blocked reporting rows and preserve stable
scalar evidence. They do not rerun benchmarks and do not decide a performance or
memory-use winner.

## Reason Vocabulary

```text
0 = accepted reporting inventory row
1 = missing reporting inventory row
2 = blocked reporting inventory row
```

## Stop Lines

- No repeated benchmark execution.
- No performance conclusion.
- No memory-use conclusion.
- No process allocator replacement.
- No hooks.
- No backend matcher additions.
- No `#[global_allocator]`.
- No provider package / DLL generation.
- No hidden discovery or process-global activation.
- No worker/thread execution.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_reporting_diagnostics_guard.sh --level L2
```
