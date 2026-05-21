# Hako Alloc Allocator Comparison C Mimalloc Result Summary Diagnostics SSOT

Status: accepted
Decision: accepted
Date: 2026-05-21
Owner: MIMAP-458A

## Decision: accepted

MIMAP-458A adds observer-only diagnostics over the MIMAP-457A C-vs-Hako result
summary inventory.

The diagnostics classify accepted / blocked summary rows and preserve scalar
evidence. They do not rerun benchmarks and do not decide a performance or
memory-use winner.

## Reason Vocabulary

```text
0 = accepted summary inventory row
1 = missing summary inventory row
2 = blocked summary inventory row
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
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_summary_diagnostics_guard.sh --level L2
```
