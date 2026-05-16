---
Status: SSOT
Decision: accepted
Date: 2026-05-16
Scope: MIMAP-037A facade huge backing-set helper cleanup.
Related:
  - docs/development/current/main/phases/phase-293x/293x-467-MIMAP-037A-FACADE-HUGE-BACKING-SET-HELPER.md
  - docs/development/current/main/phases/phase-293x/293x-463-MIMAP-035A-FACADE-HUGE-UNRESERVE-FAILFAST.md
  - docs/development/current/main/design/mimalloc-post-huge-unreserve-closeout-ssot.md
---

# Mimalloc Facade Huge Backing-Set Helper SSOT

## Decision

`HakoAllocObjectLifecycleFacadeHugeBackingSet` owns the small backing-range
set used by facade huge unreserve fail-fast diagnostics.

It exists to keep duplicate/stale backing-range tracking out of the route
owner. The route decides behavior; this helper only records and checks
`base + bytes` pairs.

## Owner

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_huge_backing_set_box.hako
```

## Contract

```text
length():
  returns the number of marked backing ranges

find(base, bytes):
  returns index for an exact pair, or -1

has(base, bytes):
  returns 1 when find(base, bytes) is non-negative, else 0

mark(base, bytes):
  returns 0 on success
  returns 1 when base is zero
  returns 2 when bytes is not positive
  returns 5 when the exact pair already exists
```

The numeric return values intentionally preserve the MIMAP-035A fail-fast
vocabulary. This row does not add new diagnostics.

## Consumers

```text
HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastRoute:
  uses the helper for duplicate/stale unreserve backing checks
```

## Stop Lines

This helper must not:

```text
call page-source APIs
call OSVM APIs
call unreserve/decommit/recommit adapters
own allocator lifecycle behavior
open provider activation or host allocator replacement
add backend .inc matchers
```

## Guard

```text
tools/checks/k2_wide_mimalloc_facade_huge_backing_set_helper_guard.sh
```

The guard pins the helper export, README/index entries, replacement of the
parallel `unreserved_bases` / `unreserved_bytes` route fields, and absence of
provider / host allocator replacement.
