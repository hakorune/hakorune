---
Status: Landed
Date: 2026-05-25
Scope: compute baseline-subtracted RSS gap evidence.
Blocker: MIMALLOC-COMPARISON-MEMORY-GAP-INCREMENTAL-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-34-MIMALLOC-COMPARISON-MEMORY-GAP-BASELINE.md
  - tools/allocator/mimalloc_memory_gap_incremental.py
  - tools/checks/k2_wide_phase295x_memory_gap_incremental_guard.sh
---

# 295x-35 Mimalloc Comparison Memory Gap Incremental

## Decision

Close:

```text
MIMALLOC-COMPARISON-MEMORY-GAP-INCREMENTAL-295X-001
```

Add presentation-only baseline-subtracted RSS evidence:

```text
output_contract=mimalloc-comparison-memory-gap-incremental-v0
baseline_workload=representative-empty-v0
fixed_process_runtime_baseline_delta_bytes = hako_empty_median - c_empty_median
workload_incremental_delta_bytes = hako_incremental - c_incremental
```

The decomposition is intentionally arithmetic and non-judgmental:

```text
total_delta = fixed_process_runtime_baseline_delta + incremental_delta
unattributed_residual = 0
winner_claim = 0
```

This makes it visible whether the measured RSS gap is mostly fixed process
runtime cost or workload-incremental allocator behavior before any winner
claim.

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-MEMORY-GAP-CLOSEOUT-295X-001
```

The follow-on should close this attribution pack, summarize the observed fixed
baseline vs incremental deltas, and select whether the next comparison seam is
measurement policy refinement, workload expansion, or .hako runtime baseline
reduction.

## Stop Line

This row does not reduce `.hako` runtime baseline RSS, compute performance or
memory winners, require RSS parity, enable provider/DLL/replacement seams,
install hooks, or open worker/TLS, atomics, remote-free stress, abandoned heap
stress, OSVM page-source parity, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_memory_gap_incremental_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
