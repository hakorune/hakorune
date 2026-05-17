---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-071A scalar reclaim scheduler request ledger consume route.
Related:
  - docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-558-MIMAP-071A-RECLAIM-SCHEDULER-REQUEST-LEDGER-CONSUME-ROUTE.md
  - lang/src/hako_alloc/memory/reclaim_scheduler_request_ledger_box.hako
  - apps/hako-alloc-reclaim-scheduler-request-ledger-consume-proof/
---

# Hako Alloc Reclaim Scheduler Request Ledger Consume SSOT

## Decision

`MIMAP-071A` extends the scalar scheduler request ledger with a local consume
route for one pending modeled scheduler request.

The route clears the pending scalar request only when the requested page id
matches the pending page. It does not run a scheduler, spawn workers, expose
source-level concurrency semantics, call page-source APIs, release OSVM memory,
activate providers, or replace the host allocator.

## Owner

```text
lang/src/hako_alloc/memory/reclaim_scheduler_request_ledger_box.hako
```

New surface:

```text
HakoAllocReclaimSchedulerRequestLedgerConsumeReport
HakoAllocReclaimSchedulerRequestLedger.consumePendingRequest(page_id)
```

Reason vocabulary:

| Reason | Meaning |
| --- | --- |
| `0` | pending request consumed / cleared |
| `1` | no pending request |
| `2` | pending request page id mismatch |

## Stop Lines

No part of `MIMAP-071A` may add:

```text
real thread scheduling
worker spawning
source-level concurrency semantics
source-level worker_local
page-source call
OSVM unreserve / release
provider activation
hooks
host allocator replacement
backend app/name matcher
```
