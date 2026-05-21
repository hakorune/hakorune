# Hako Alloc Allocator Comparison Representative Benchmark Execution Diagnostics SSOT

Status: accepted
Decision: accepted
Date: 2026-05-21
Owner: MIMAP-445A

## Decision: accepted

MIMAP-445A adds observer-only diagnostics for the MIMAP-444A representative
benchmark execution pilot. It consumes the pilot report and classifies ready
execution evidence and blocked execution reports.

This row does not add a new benchmark workload. It does not run C mimalloc, does
not replace the process allocator, does not install hooks, does not add backend
matchers, does not install a global allocator, and does not open worker/thread
execution.

## Diagnostic Reasons

| Reason | Meaning |
| --- | --- |
| 0 | representative benchmark execution accepted |
| 1 | representative benchmark execution not ready |
| 2 | invalid run count |
| 3 | output contract missing |
| 4 | evidence storage missing |
| 5 | closed seam attempted execution |

## Stop Lines

- No new benchmark workload.
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

MIMAP-445A runs L2 daily evidence:

- VM proof app output contract
- MIR JSON emit
- route preflight
- typed object / record declaration checks
- `.inc` no-growth check for app / owner names

L3/L4 evidence is deferred to the representative benchmark execution closeout.
