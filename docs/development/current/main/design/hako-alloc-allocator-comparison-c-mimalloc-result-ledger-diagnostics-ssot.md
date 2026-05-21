# Hako Alloc Allocator Comparison C Mimalloc Result Ledger Diagnostics SSOT

Status: accepted
Decision: accepted
Date: 2026-05-21
Owner: MIMAP-455A

## Decision: accepted

MIMAP-455A adds observer-only diagnostics over the MIMAP-454A C-vs-Hako result
ledger report.

The diagnostics classify accepted / blocked ledger rows and preserve scalar
evidence. They do not rerun benchmarks and do not decide a performance or
memory-use winner.

## Reason Vocabulary

```text
0 = accepted comparison ledger row
1 = missing Hako representative diagnostic
2 = blocked Hako representative diagnostic
3 = missing C mimalloc evidence diagnostic
4 = blocked C mimalloc evidence diagnostic
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

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_ledger_diagnostics_guard.sh --level L2
```
