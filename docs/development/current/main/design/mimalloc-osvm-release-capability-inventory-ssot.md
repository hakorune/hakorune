---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-048A OSVM release capability inventory after OSVM-backed fast-path unreserve closeout.
Related:
  - docs/development/current/main/design/mimalloc-osvm-fast-path-unreserve-closeout-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-528-MIMAP-048A-OSVM-RELEASE-CAPABILITY-INVENTORY.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
---

# Mimalloc OSVM Release Capability Inventory SSOT

## Decision

`MIMAP-048A` keeps OS release inactive.

The landed allocator surface currently owns OSVM-backed unreserve, not OS
release:

```text
unreserve:
  hako_osvm_unreserve_bytes_i64
  OsVmCoreBox.unreserve_bytes_i64 / unreserve_bytes_usize
  HakoAllocPageSourcePolicy.unreservePage
  HakoAllocPageSourceUnreserveAdapter.unreservePage
  HakoAllocOsVmFastPathUnreserveRoute.unreservePurgedPage

release:
  no hako_osvm_release*
  no release_bytes*
  no releasePage
  no fast-path release route
```

Future OS release work must be a separate row. It must define whether release is
a platform capability distinct from unreserve, a stronger lifetime transition,
or rejected vocabulary. Until then, source owners must not use "release" as a
synonym for unreserve in allocator route responsibilities.

## Boundary

| Surface | State | Owner |
| --- | --- | --- |
| OSVM reserve/commit/decommit/unreserve | active | `OsVmCoreBox` |
| Page-source unreserve adapter | active | `HakoAllocPageSourceUnreserveAdapter` |
| Fast-path unreserve route | active | `HakoAllocOsVmFastPathUnreserveRoute` |
| OSVM release | inactive | future explicit row only |
| Fast-path release behavior | inactive | future explicit row only |

## Guard Contract

The inventory guard must prove:

```text
MIMAP-048A card is landed
this SSOT is accepted
MIMAP-047A closeout remains accepted
unreserve owners and guards remain present
release-specific symbols are absent from runtime/backend/allocator active code
unreserve owner comments do not describe themselves as release rows
check-scripts index lists the release inventory guard
```

Release-specific forbidden vocabulary:

```text
hako_osvm_release
hako_osvm_release_bytes_i64
release_bytes_i64
release_bytes_usize
releasePage
HakoOsvmRelease
```

This vocabulary may appear in docs and guards that explicitly describe the
inactive release boundary. It must not appear in active runtime/backend route
code until a future release row opens it.

## Stop Lines

`MIMAP-048A` must not add:

```text
hako_osvm_release* externs
release_bytes_* source methods
releasePage page-source methods
fast-path release behavior
provider activation
hooks
host allocator replacement
#[global_allocator]
reclaim execution
page ownership migration
backend .inc app/name matchers
```

## Next Row

After this inventory, open:

```text
MIMAP-048B post-release-inventory row selection
```

That row must choose one follow-up task. It must not treat this inventory as
implicit permission to implement release behavior.
