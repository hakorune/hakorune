---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: closeout after MIMAP-042A / MIMAP-043A OSVM-backed fast-path route rows.
Related:
  - docs/development/current/main/phases/phase-293x/293x-516-MIMAP-042A-OSVM-FAST-PATH-BOUNDED-PURGE-ROUTE.md
  - docs/development/current/main/phases/phase-293x/293x-518-MIMAP-043A-OSVM-FAST-PATH-RECOMMIT-REUSE-ROUTE.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
  - docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md
---

# Mimalloc OSVM Fast-Path Route Closeout SSOT

## Decision

The OSVM-backed fast-path route is closed through bounded purge and
post-recommit allocation reuse:

```text
MIMAP-042A:
  HakoAllocOsVmFastPathPurgeRoute
  allocation -> release -> bounded purge

MIMAP-043A:
  HakoAllocOsVmFastPathReuseRoute
  bounded purge -> M205 recommit -> post-recommit allocation
```

This closeout does not imply provider activation, hook installation, process
allocator replacement, reclaim execution, remote-free execution, unreserve, OS
release, or source-language concurrency work.

## Completed Surface

| Row | Surface | Owner |
| --- | --- | --- |
| `MIMAP-042A` | OSVM-backed allocation/release plus bounded purge through M199/M212 | `HakoAllocOsVmFastPathPurgeRoute` |
| `MIMAP-043A` | M205 recommit plus post-recommit allocation through the same route | `HakoAllocOsVmFastPathReuseRoute` |

## Still Inactive

These require explicit future rows:

```text
unreserve from the fast-path route
OS release from the fast-path route
provider activation
hooks
process allocator replacement
#[global_allocator]
remote-free execution
TLS/atomic execution changes
thread scheduling
reclaim execution
page ownership migration
backend .inc app/name matcher shortcuts
```

## Guard Contract

The closeout guard must verify:

```text
MIMAP-042A and MIMAP-043A cards are landed
MIMAP-042A and MIMAP-043A proof apps and guards exist
MIMAP-042A and MIMAP-043A owners are exported from hako_module.toml
README files name both owner boundaries
check-scripts index lists both focused guards plus this closeout guard
.inc contains no app/box matcher for the OSVM fast-path route owners
provider/host replacement markers remain absent from active code
route owners do not directly call page-source/OSVM/unreserve/OS-release seams
```

## Next Row

After this closeout, open a planning row:

```text
MIMAP-044B post-fast-path-closeout row selection
```

That row must choose exactly one next action. It may select a later allocator
behavior row, a compiler/language sidecar, or a lane switch, but it must not
implicitly open provider activation or host allocator replacement.
