---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-063A allocator-internal reclaim scheduler boundary inventory.
Related:
  - docs/development/current/main/phases/phase-293x/293x-550-MIMAP-063A-RECLAIM-SCHEDULER-BOUNDARY-INVENTORY.md
  - tools/checks/k2_wide_hako_alloc_reclaim_scheduler_boundary_inventory_guard.sh
  - docs/development/current/main/design/mimalloc-concurrency-substrate-boundary-ssot.md
---

# Hako Alloc Reclaim Scheduler Boundary Inventory SSOT

## Decision

`MIMAP-063A` fixes the boundary between scalar reclaim completion and any future
allocator-internal scheduling request.

This row is inventory-only. It does not add a `.hako` scheduler owner, execute
threads, expose source-level concurrency semantics, call page-source APIs,
release OSVM memory, activate providers, or replace the host allocator.

## Boundary Split

Allocator reclaim may eventually need an internal scheduler request, but that is
not the same as the Hakorune source concurrency surface.

```text
allocator-internal scheduler boundary:
  modeled request/suppress facts
  future runtime-owned worker handoff
  no source-visible task semantics

source concurrency surface:
  co / nowait / await
  Channel<T>
  sync box
  context
  source-level worker_local remains closed
```

`MIMAP-063A` only names the first boundary. It does not implement either
boundary.

## Existing Substrate Inputs

The inventory may reference already-landed substrate facts:

```text
worker/thread identity substrate: MIMAP-WORKER-001
TLS/atomic route proofs: existing mimalloc capability rows
scalar reclaim completion: MIMAP-060A
scalar reclaim lane closeout: MIMAP-061A
```

It must not add new substrate rows.

## Next Row

After this inventory, the next row may add a small `.hako` marker contract:

```text
MIMAP-064A reclaim scheduler request marker contract
```

That future row should classify whether a completed scalar reclaim would request
modeled scheduler handoff or remain local/suppressed. It still must not execute
real scheduling.

## Guard Surface

```text
tools/checks/k2_wide_hako_alloc_reclaim_scheduler_boundary_inventory_guard.sh
```

The guard verifies:

```text
current card and next card pointers
boundary split is documented
source-level concurrency features remain closed
provider / hook / replacement remain inactive
no scheduler app/owner matcher leaks into lang/c-abi/shims
```

## Stop Lines

No part of this row may add:

```text
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
