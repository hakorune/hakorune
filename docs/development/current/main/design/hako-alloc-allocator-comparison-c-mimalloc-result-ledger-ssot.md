# Hako Alloc Allocator Comparison C Mimalloc Result Ledger SSOT

Status: accepted
Decision: accepted
Date: 2026-05-21
Owner: MIMAP-454A

## Decision: accepted

MIMAP-454A opens a narrow scalar result ledger over:

```text
MIMAP-445A Hako representative benchmark execution diagnostics
MIMAP-452A explicit C mimalloc runner evidence diagnostics
```

The ledger records whether both sides are available and copies the scalar
evidence required by later diagnostics / closeout rows. It does not make a
performance or memory-use conclusion.

## Ledger Contract

The owner is:

```text
HakoAllocAllocatorComparisonCMimallocResultLedger
```

It may:

- consume one Hako representative diagnostic report;
- consume one explicit C mimalloc evidence diagnostic report;
- record Hako scalar metrics;
- record C mimalloc scalar metrics and memory evidence;
- compute simple scalar deltas;
- classify missing / blocked Hako or C evidence.

It must not:

- rerun Hako or C benchmarks;
- replace the process allocator;
- install hooks;
- add backend matchers;
- install `#[global_allocator]`;
- generate provider packages / DLLs;
- use hidden discovery or process-global activation config;
- open worker/TLS, source concurrency, cross-function `Result` ABI, or runtime
  sum materialization.

## Reason Vocabulary

```text
0 = accepted comparison ledger row
1 = missing Hako representative diagnostic
2 = blocked Hako representative diagnostic
3 = missing C mimalloc evidence diagnostic
4 = blocked C mimalloc evidence diagnostic
```

## Evidence Fields

The accepted row records:

```text
Hako:
  allocation_count
  release_count
  reject_count
  requested_bytes
  outstanding_blocks
  small_free_count
  medium_free_count

C mimalloc:
  allocation_count
  free_count
  requested_bytes
  peak_rss_bytes
  run_count

derived:
  allocation_count_delta
  requested_bytes_delta
  memory_evidence_present
```

The derived fields are ledger evidence only. They are not a performance
ranking, memory-use winner, or replacement decision.

## Validation

MIMAP-454A uses `scalar-mir` validation:

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_ledger_guard.sh --level L2
```
