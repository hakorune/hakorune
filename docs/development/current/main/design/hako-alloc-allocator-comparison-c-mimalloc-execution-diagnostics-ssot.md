# Hako Alloc Allocator Comparison C Mimalloc Execution Diagnostics SSOT

Status: accepted
Decision: accepted
Date: 2026-05-21
Owner: MIMAP-449A

## Decision: accepted

MIMAP-449A adds observer-only diagnostics for the MIMAP-448A C mimalloc
execution inventory. It classifies missing comparison inputs without executing
C mimalloc.

## Diagnostic Reasons

| Reason | Meaning |
| --- | --- |
| 0 | C mimalloc execution inventory ready |
| 1 | C mimalloc runner missing |
| 2 | representative workload contract missing |
| 3 | Hako representative metrics missing |
| 4 | output contract missing |
| 5 | memory-usage contract missing |
| 6 | evidence storage missing |
| 7 | run count missing |
| 8 | invalid run count |

## Stop Lines

- No C mimalloc execution.
- No process allocator replacement.
- No hook installation.
- No backend matcher additions.
- No global allocator installation.
- No hidden env or implicit discovery of C mimalloc behavior.
- No worker/thread execution.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI.
- No runtime sum materialization.

## Validation

Validation profile: `scalar-mir`.

MIMAP-449A runs L2 daily evidence:

- VM proof app output contract
- MIR JSON emit
- route preflight
- typed object / record declaration checks
- `.inc` no-growth check for app / owner names

L3/L4 evidence is deferred to a later C mimalloc execution closeout.
