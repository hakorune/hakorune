---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-068A scalar reclaim scheduler request ledger route.
Related:
  - docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-marker-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-555-MIMAP-068A-RECLAIM-SCHEDULER-REQUEST-LEDGER-ROUTE.md
  - lang/src/hako_alloc/memory/reclaim_scheduler_request_ledger_box.hako
  - apps/hako-alloc-reclaim-scheduler-request-ledger-proof/
---

# Hako Alloc Reclaim Scheduler Request Ledger SSOT

## Decision

`MIMAP-068A` adds a scalar allocator-owned request ledger after the reclaim
scheduler request marker.

The route records that a completed scalar reclaim would request modeled
scheduler handoff. It does not execute scheduling, spawn workers, expose
source-level concurrency semantics, call page-source APIs, release OSVM memory,
activate providers, or replace the host allocator.

## Owner

```text
lang/src/hako_alloc/memory/reclaim_scheduler_request_ledger_box.hako
```

Responsibilities:

```text
compose HakoAllocReclaimSchedulerRequestMarker
record at most one pending modeled scheduler request
preserve the page id, marker reason, and reclaim completion facts that drove it
suppress marker-blocked, scheduler-disabled, and already-pending rows
report inactive production-facing scheduler/provider/source-concurrency flags
```

Non-responsibilities:

```text
real thread scheduling
worker spawn / wake / run queue execution
source-level co / nowait / await / Channel / sync box / context / worker_local
page-source or OSVM release/unreserve
provider activation / hooks / host allocator replacement
backend app/name matcher
```

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | pending scheduler request recorded |
| `1` | marker suppressed because reclaim completion was blocked |
| `2` | marker suppressed because scheduler request was disabled |
| `3` | ledger already has a pending request |

## Proof Surface

```text
apps/hako-alloc-reclaim-scheduler-request-ledger-proof/
tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_guard.sh
```

Required inactive facts:

```text
would_schedule_thread = 0
would_spawn_worker = 0
would_touch_source_concurrency = 0
would_call_page_source = 0
would_unreserve = 0
would_release_osvm = 0
would_activate_provider = 0
would_host_allocator_swap = 0
```

## Stop Lines

No part of `MIMAP-068A` may add:

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
