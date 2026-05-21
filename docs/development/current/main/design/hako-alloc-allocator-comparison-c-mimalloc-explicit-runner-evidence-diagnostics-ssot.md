# Hako Alloc Allocator Comparison C Mimalloc Explicit Runner Evidence Diagnostics SSOT

Status: accepted
Decision: accepted
Date: 2026-05-21
Owner: MIMAP-452A

## Decision: accepted

MIMAP-452A adds observer diagnostics for the MIMAP-451A explicit C mimalloc
runner execution pilot report. It classifies missing or failed evidence without
rerunning the C runner and without opening repeated benchmark execution.

## Diagnostic Reasons

| Reason | Meaning |
| --- | --- |
| 0 | explicit runner evidence ready |
| 1 | MIMAP-451A evidence diagnostic/report missing |
| 2 | MIMAP-451A diagnostic was rejected/not ready |
| 3 | explicit runner invocation missing |
| 4 | runner output missing |
| 5 | memory-use evidence missing |
| 6 | stable output contract missing |
| 7 | runner result was non-zero |
| 8 | invalid run count |

## Stop Lines

- No repeated or heavy benchmark pack.
- No C runner execution from the diagnostic owner.
- No process allocator replacement.
- No hook installation.
- No backend matcher additions.
- No `#[global_allocator]`.
- No provider package / DLL generation.
- No hidden env or runtime discovery of mimalloc behavior.
- No source-level worker-local or concurrency surface.
- No worker/thread execution.
- No cross-function `Result` direct ABI.
- No runtime sum materialization.

## Validation

Validation profile: `scalar-mir`.

MIMAP-452A runs L2 daily evidence:

- VM proof app output contract;
- MIR JSON emit;
- route preflight;
- typed object / record declaration checks;
- `.inc` no-growth check for app / owner names.

The MIMAP-451A explicit C runner remains covered by the MIMAP-451A guard. This
row observes evidence shape only.
