---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-065A reclaim scheduler marker closeout guard.
Related:
  - docs/development/current/main/design/hako-alloc-reclaim-scheduler-boundary-inventory-ssot.md
  - docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-marker-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-552-MIMAP-065A-RECLAIM-SCHEDULER-MARKER-CLOSEOUT-GUARD.md
---

# Hako Alloc Reclaim Scheduler Marker Closeout SSOT

## Decision

`MIMAP-065A` closes the scheduler boundary / request-marker slice.

This row is a closeout guard only. It adds no `.hako` behavior and does not
execute scheduling, expose source-level concurrency features, call page-source
APIs, release OSVM memory, activate providers, or replace the host allocator.

## Closed Row Set

| Row | Role |
| --- | --- |
| `MIMAP-063A` | allocator-internal scheduler boundary inventory |
| `MIMAP-064A` | scalar scheduler request marker contract |

## Guard Surface

```text
tools/checks/k2_wide_hako_alloc_reclaim_scheduler_marker_closeout_guard.sh
```

The guard verifies:

```text
MIMAP-063A and MIMAP-064A are landed
MIMAP-064A proof app is in tools/checks/proof_apps.toml
MIMAP-063A / MIMAP-064A / MIMAP-065A guards are indexed
MIMAP-064A owner remains exported and documented
no scheduler app/owner matcher leaks into lang/c-abi/shims
provider / hook / replacement remain inactive
```

## Next Row

After this closeout, the current row should be:

```text
MIMAP-066A post-scheduler-marker row selection
```

That row decides whether to continue allocator reclaim behavior, open a
compiler/language sidecar, or switch to a broader Hakorune language feature
lane. The decision must remain one narrow row.

## Stop Lines

No part of this row may add:

```text
new .hako allocator behavior
real thread scheduling
source-level concurrency semantics
source-level worker_local
page-source call
OSVM unreserve / release
provider activation
hooks
host allocator replacement
backend app/name matcher
```
