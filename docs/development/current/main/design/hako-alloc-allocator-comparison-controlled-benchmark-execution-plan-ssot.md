# Hako Alloc Allocator Comparison Controlled Benchmark Execution Plan

Status: accepted
Decision: accepted
Scope: MIMAP-439A allocator comparison controlled benchmark execution plan.

## Purpose

MIMAP-439A selects the first benchmark execution seam after the preflight pack
is closed out.

The selected path is intentionally narrow:

```text
controlled benchmark execution inventory
  -> diagnostics
  -> closeout
  -> representative benchmark execution row
```

The next behavior row is MIMAP-440A, which should inventory the execution shape
without replacing the process allocator or installing hooks.

## First Controlled Shape

```text
benchmark runner: explicit
workload source: MIMAP-430A workload matrix
measurement source: MIMAP-433A measurement plan
execution target: hako_alloc-owned proof app / tool entry
output contract: explicit line contract, no hidden env discovery
allocator replacement: closed
#[global_allocator]: closed
backend matcher additions: closed
hook installation: closed
```

## Validation Profile

```text
MIMAP-440A inventory:
  L2 scalar-mir

MIMAP-441A diagnostics:
  L2 scalar-mir

closeout:
  L2 pack, L3 only if benchmark execution is actually opened
```

## Still Closed

```text
process allocator replacement
#[global_allocator]
hook installation
backend matcher additions
worker/TLS or thread execution
hidden env / implicit discovery / process-global activation config
```

## Notes

This plan does not claim performance parity with C mimalloc. It only selects
the first explicit benchmark execution seam so later rows can collect
throughput and memory-usage evidence without changing the host allocator.
