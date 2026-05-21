# Hako Alloc Allocator Comparison Representative Benchmark Execution Pilot SSOT

Status: accepted
Decision: accepted
Date: 2026-05-21
Owner: MIMAP-444A

## Decision: accepted

MIMAP-444A opens the first bounded representative benchmark execution seam for
allocator comparison evidence.

This row may execute a controlled `HakoAllocProductionFacade` workload and
record scalar metrics. It still does not replace the process allocator, install
hooks, add backend matchers, install a global allocator, or run worker/thread
behavior.

## Representative Workload

The pilot executes one bounded workload:

```text
allocate(8)
allocate(48)
allocate(80)       # expected reject in current facade
release(first)
allocate(16)
release(rejected)  # expected reject
```

Expected accepted metrics:

```text
allocation_count = 3
release_count = 1
reject_count = 2
requested_bytes = 72
outstanding_blocks = 2
small_free_count = 7
medium_free_count = 3
```

## Reject Reasons

| Reason | Meaning |
| --- | --- |
| 0 | accepted |
| 1 | controlled execution diagnostic not ready |
| 2 | invalid run count |
| 3 | missing output contract |
| 4 | missing evidence storage |
| 5 | closed seam attempted execution |

## Stop Lines

- No process allocator replacement.
- No hook installation.
- No backend matcher additions.
- No global allocator installation.
- No C mimalloc execution.
- No hidden env or implicit discovery of benchmark behavior.
- No worker/thread execution.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI.
- No runtime sum materialization.

## Validation

Validation profile: `scalar-mir`.

MIMAP-444A runs L2 daily evidence:

- VM proof app output contract
- MIR JSON emit
- route preflight
- typed object / record declaration checks
- `.inc` no-growth check for app / owner names

L3 native benchmark comparison evidence belongs to a later explicit row.
