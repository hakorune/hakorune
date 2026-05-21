# Hako Alloc Allocator Comparison C Mimalloc Result Reporting Inventory SSOT

Status: accepted
Decision: accepted
Date: 2026-05-21
Owner: MIMAP-460A

## Decision: accepted

MIMAP-460A adds a narrow reporting inventory over the MIMAP-458A C-vs-Hako result
summary diagnostics.

The reporting inventory prepares stable scalar fields for later reporting
diagnostics and presentation / decision rows. It preserves already-recorded
comparison availability, allocation/request-byte evidence, deltas, and closed
stop-line fields without rerunning benchmarks or deciding a performance or
memory-use winner.

## Reason Vocabulary

```text
0 = accepted summary diagnostic row
1 = missing summary diagnostic row
2 = blocked summary diagnostic row
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
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_reporting_inventory_guard.sh --level L2
```
