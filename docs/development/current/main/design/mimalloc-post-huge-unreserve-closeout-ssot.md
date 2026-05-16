---
Status: SSOT
Decision: accepted
Date: 2026-05-16
Scope: closeout after MIMAP-035A facade huge unreserve success/fail-fast lane.
Related:
  - docs/development/current/main/phases/phase-293x/293x-461-MIMAP-034A-FACADE-HUGE-UNRESERVE-ROUTE.md
  - docs/development/current/main/phases/phase-293x/293x-463-MIMAP-035A-FACADE-HUGE-UNRESERVE-FAILFAST.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
  - docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md
---

# Mimalloc Post-Huge-Unreserve Closeout SSOT

## Decision

The facade huge unreserve lane is closed through success and fail-fast
diagnostics:

```text
MIMAP-032A:
  OSVM unreserve substrate route
MIMAP-033A:
  page-source unreserve adapter
MIMAP-034A:
  facade huge unreserve-after-decommit success route
MIMAP-035A:
  duplicate/stale facade huge unreserve fail-fast diagnostics
```

This closeout does not imply provider activation, hook installation, process
allocator replacement, broad purge/recommit execution, or source-language
concurrency work.

## Completed Surface

| Row | Surface | Owner |
| --- | --- | --- |
| `MIMAP-032A` | `OsVmCoreBox.unreserve_bytes_i64` substrate route | runtime / OSVM substrate |
| `MIMAP-033A` | `HakoAllocPageSourcePolicy.unreservePage` and `HakoAllocPageSourceUnreserveAdapter` | page-source policy owner |
| `MIMAP-034A` | facade huge unreserve-after-decommit success | `HakoAllocObjectLifecycleFacadeHugeUnreserveRoute` |
| `MIMAP-035A` | duplicate/stale huge unreserve fail-fast diagnostics | `HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastRoute` |

## Still Inactive

These require explicit future rows:

```text
recommit after unreserve
purge scheduler widening
remote-free / TLS behavior changes
thread scheduling
reclaim execution
page ownership migration
provider activation
hooks
process allocator replacement
#[global_allocator]
backend .inc app/name matcher shortcuts
```

## Guard Contract

The closeout guard must verify:

```text
MIMAP-034A and MIMAP-035A cards are landed
MIMAP-034A and MIMAP-035A proof apps and guards exist
MIMAP-034A and MIMAP-035A owners are exported from hako_module.toml
README files name both owner boundaries
check-scripts index lists both focused guards plus this closeout guard
.inc contains no app/box matcher for the facade huge unreserve owners
provider/host replacement markers remain absent from active code
```

## Next Row

After this closeout, open a planning row:

```text
MIMAP-036B post-huge-unreserve-closeout row selection
```

That row must choose exactly one next action. It may select a later allocator
behavior row, a cleanup row, or a lane switch, but it must not implicitly open
provider activation or host allocator replacement.
