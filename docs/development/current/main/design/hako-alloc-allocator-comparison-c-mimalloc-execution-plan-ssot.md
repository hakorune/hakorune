# Hako Alloc Allocator Comparison C Mimalloc Execution Plan SSOT

Status: accepted
Decision: accepted
Date: 2026-05-21
Scope: MIMAP-447A allocator comparison C mimalloc execution plan.

## Purpose

MIMAP-447A selects the first C mimalloc comparison execution path after the
Hako representative benchmark execution pack is closed.

This row is planning-only. It defines the C-side comparison boundary and selects
the next inventory row. It does not execute C mimalloc, replace the process
allocator, install hooks, add backend matchers, or install a global allocator.

## Selected Path

```text
C mimalloc execution inventory
  -> diagnostics
  -> closeout
  -> comparison result ledger / report row
```

The next behavior row is:

```text
MIMAP-448A Allocator Comparison C Mimalloc Execution Inventory
```

## First C Mimalloc Shape

```text
C mimalloc runner:
  explicit runner/tool input, no implicit discovery

workload:
  same representative workload as MIMAP-444A
  allocate(8), allocate(48), allocate(80 reject-equivalent if unsupported),
  release(first), allocate(16), release(rejected-equivalent if unsupported)

output contract:
  stable line/record contract for throughput and memory-use evidence

evidence storage:
  explicit report/ledger row, no hidden env side channel

host process:
  no process allocator replacement
  no hook installation
  no backend matcher additions
  no #[global_allocator]
```

## Validation Profile

```text
MIMAP-448A inventory:
  scalar-mir if it adds a .hako proof owner

MIMAP-449A diagnostics:
  scalar-mir

closeout:
  pack validation; native/C execution only when explicitly opened by a later row
```

## Still Closed

```text
C mimalloc execution
process allocator replacement
#[global_allocator]
hook installation
backend matcher additions
worker/TLS or thread execution
hidden env / implicit discovery / process-global activation config
```

## Notes

The C mimalloc comparison is completion evidence for throughput and memory-use
comparison. It is not a request to replace the current runtime allocator. Any
process-global allocator replacement remains parked behind a separate optional
ladder.
