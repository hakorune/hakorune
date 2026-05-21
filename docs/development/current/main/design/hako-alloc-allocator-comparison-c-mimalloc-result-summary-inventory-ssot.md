# Hako Alloc Allocator Comparison C Mimalloc Result Summary Inventory SSOT

Status: accepted
Decision: accepted
Date: 2026-05-21
Owner: MIMAP-457A

## Decision: accepted

MIMAP-457A adds a narrow summary inventory over the MIMAP-454A C-vs-Hako result
ledger and the MIMAP-455A result-ledger diagnostics.

The summary inventory is a scalar report owner. It prepares a later reporting row
by collecting result availability, diagnostic readiness, already-recorded
allocation/request-byte deltas, and closed stop-line fields. It does not rerun
benchmarks and does not decide a performance or memory-use winner.

## Reason Vocabulary

```text
0 = accepted summary inventory
1 = missing result ledger row
2 = missing result-ledger diagnostic row
3 = blocked result-ledger diagnostic row
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
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_summary_inventory_guard.sh --level L2
```
