# Hako Alloc Allocator Comparison Controlled Benchmark Execution Diagnostics SSOT

Status: accepted
Date: 2026-05-21
Owner: MIMAP-441A

## Decision: accepted

MIMAP-441A adds observer-only diagnostics for the MIMAP-440A controlled
benchmark execution inventory. It classifies missing execution-shape inputs and
open closed-seam inputs without executing a benchmark.

## Diagnostic Reasons

| Reason | Meaning |
| --- | --- |
| 0 | controlled execution shape ready |
| 1 | benchmark runner missing |
| 2 | workload source not ready |
| 3 | measurement source not ready |
| 4 | output contract missing |
| 5 | evidence storage missing |
| 6 | representative run missing |
| 7 | process allocator replacement open |
| 8 | hook installation open |
| 9 | backend matcher additions open |
| 10 | global allocator installation open |
| 11 | hidden env / implicit discovery open |
| 12 | closed seam attempted execution |

## Stop Lines

- No benchmark execution.
- No process allocator replacement.
- No hook installation.
- No backend matcher additions.
- No global allocator installation.
- No hidden env or implicit discovery of benchmark behavior.
- No worker/thread execution.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI.
- No runtime sum materialization.

## Validation

Validation profile: `scalar-mir`.

MIMAP-441A runs L2 daily evidence:

- VM proof app output contract
- MIR JSON emit
- route preflight
- typed object / record declaration checks
- `.inc` no-growth check for app / owner names

L3/L4 evidence is deferred to the controlled benchmark execution closeout.
