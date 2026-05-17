---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-074A scalar reclaim scheduler request ledger roundtrip route.
Related:
  - docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-ssot.md
  - docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-consume-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-561-MIMAP-074A-RECLAIM-SCHEDULER-REQUEST-LEDGER-ROUNDTRIP-ROUTE.md
  - lang/src/hako_alloc/memory/reclaim_scheduler_request_ledger_roundtrip_box.hako
  - apps/hako-alloc-reclaim-scheduler-request-ledger-roundtrip-proof/
---

# Hako Alloc Reclaim Scheduler Request Ledger Roundtrip SSOT

## Decision

`MIMAP-074A` adds a scalar allocator-owned roundtrip route that composes the
existing scheduler request ledger record and consume routes.

The route proves a local lifecycle only:

```text
record scheduler request
  -> pending request exists
  -> consume matching page id
  -> pending request cleared
```

It does not execute scheduling, spawn workers, expose source concurrency, call
page-source APIs, release OSVM memory, activate providers, or replace the host
allocator.

## Owner

```text
lang/src/hako_alloc/memory/reclaim_scheduler_request_ledger_roundtrip_box.hako
```

Responsibilities:

```text
compose HakoAllocReclaimSchedulerRequestLedger
record one modeled scheduler request
consume the same pending page id immediately
report the record and consume reasons in one scalar report
report inactive production-facing scheduler/provider/source-concurrency flags
```

Non-responsibilities:

```text
real thread scheduling
worker spawn / wake / run queue execution
source concurrency features
page-source or OSVM release/unreserve
provider activation / hooks / host allocator replacement
backend app/name matcher
```

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | record and consume succeeded |
| `1` | record suppressed because reclaim completion was blocked |
| `2` | record suppressed because scheduler request was disabled |
| `3` | record suppressed because ledger already had a pending request |
| `4` | record succeeded but consume failed |

## Proof Surface

```text
apps/hako-alloc-reclaim-scheduler-request-ledger-roundtrip-proof/
tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_roundtrip_guard.sh
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

No part of `MIMAP-074A` may add:

```text
real thread scheduling
worker spawning
source concurrency semantics
page-source call
OSVM unreserve / release
provider activation
hooks
host allocator replacement
backend app/name matcher
```
