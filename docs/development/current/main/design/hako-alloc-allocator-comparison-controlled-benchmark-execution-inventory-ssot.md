# Hako Alloc Allocator Comparison Controlled Benchmark Execution Inventory SSOT

Status: accepted
Date: 2026-05-21
Owner: MIMAP-440A

## Decision: accepted

MIMAP-440A records the first controlled allocator comparison benchmark
execution shape as scalar/model inventory. It does not execute a benchmark.

The row makes these inputs explicit:

- benchmark runner selected
- workload source ready
- measurement source ready
- output contract present
- evidence storage present
- representative run selected
- process allocator replacement closed
- hook installation closed
- backend matcher additions closed
- global allocator installation closed
- hidden env / implicit discovery closed

## Contract

Accepted inventory requires every explicit input to be `1`.

Reject reasons:

| Reason | Meaning |
| --- | --- |
| 0 | accepted |
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

MIMAP-440A runs L2 daily evidence:

- VM proof app output contract
- MIR JSON emit
- route preflight
- typed object / record declaration checks
- `.inc` no-growth check for app / owner names

L3/L4 evidence is deferred to the controlled benchmark execution closeout.
