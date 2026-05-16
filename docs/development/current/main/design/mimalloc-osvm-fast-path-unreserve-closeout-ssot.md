---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: closeout after MIMAP-045A / MIMAP-046A OSVM-backed fast-path unreserve rows.
Related:
  - docs/development/current/main/phases/phase-293x/293x-522-MIMAP-045A-OSVM-FAST-PATH-UNRESERVE-ROUTE.md
  - docs/development/current/main/phases/phase-293x/293x-524-MIMAP-046A-OSVM-FAST-PATH-UNRESERVE-FAILFAST.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
  - docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md
---

# Mimalloc OSVM Fast-Path Unreserve Closeout SSOT

## Decision

The OSVM-backed fast-path unreserve surface is closed through success and
fail-fast diagnostics:

```text
MIMAP-045A:
  HakoAllocOsVmFastPathUnreserveRoute
  allocate -> release -> bounded purge/decommit -> page-source unreserve adapter

MIMAP-046A:
  HakoAllocOsVmFastPathUnreserveFailFastRoute
  duplicate / unknown / not-decommitted fast-path unreserve diagnostics
```

This closeout does not imply OS release, post-unreserve reuse, reclaim
execution, provider activation, hook installation, process allocator
replacement, remote-free execution, TLS/atomic execution changes, thread
scheduling, or source-language concurrency work.

## Completed Surface

| Row | Surface | Owner |
| --- | --- | --- |
| `MIMAP-045A` | OSVM-backed fast-path unreserve through MIMAP-033A adapter | `HakoAllocOsVmFastPathUnreserveRoute` |
| `MIMAP-046A` | duplicate / unknown / not-decommitted unreserve diagnostics | `HakoAllocOsVmFastPathUnreserveFailFastRoute` |

## Still Inactive

These require explicit future rows:

```text
OS release from the fast-path route
post-unreserve reuse behavior
reclaim execution
provider activation
hooks
process allocator replacement
#[global_allocator]
remote-free execution
TLS/atomic execution changes
thread scheduling
page ownership migration
backend .inc app/name matcher shortcuts
```

## Guard Contract

The closeout guard must verify:

```text
MIMAP-045A and MIMAP-046A cards are landed
MIMAP-045A and MIMAP-046A proof apps and guards exist
MIMAP-045A and MIMAP-046A owners are exported from hako_module.toml
README files name both owner boundaries
check-scripts index lists both focused guards plus this closeout guard
.inc contains no app/box matcher for the fast-path unreserve owners
provider/host replacement markers remain absent from active code
route owners do not directly call page-source/OSVM/OS-release seams
```

## Next Row

After this closeout, open a planning row:

```text
MIMAP-047B post-fast-path-unreserve-closeout row selection
```

That row must choose exactly one next action. It may select a later allocator
behavior row, a compiler/language sidecar, or a lane switch, but it must not
implicitly open OS release, provider activation, or host allocator replacement.
