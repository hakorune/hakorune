# 293x-558 MIMAP-071A Reclaim Scheduler Request Ledger Consume Route

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-071A` is the allocator behavior row selected by `MIMAP-070A`.

The row extends the scalar scheduler request ledger lifecycle with a local
consume/clear route for one pending modeled scheduler request. It must not run
a scheduler or spawn a worker.

## Scope

- Extend `HakoAllocReclaimSchedulerRequestLedger` with one local consume method.
- Consume only when a pending request exists and the page id matches the
  pending request.
- Preserve scalar fail-fast reasons for no-pending and page-mismatch rows.
- Add one proof app and one focused guard, or extend the existing proof app only
  if that keeps MIMAP-068A and MIMAP-071A evidence clearly separated.

## Proposed Owner

```text
lang/src/hako_alloc/memory/reclaim_scheduler_request_ledger_box.hako
```

Allowed:

```text
consume/clear one pending modeled scheduler request
report did_consume, reason, consumed_page_id, pending_after, counters
keep request recording semantics unchanged
```

Forbidden:

```text
real thread scheduling
worker spawning or scheduler progress
source-level co / nowait / await / Channel / sync box / context / worker_local
page-source or OSVM release/unreserve
provider activation / hooks / host allocator replacement
backend app/name matcher
```

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `071A.1` | Extend ledger owner with consume route. | no-pending, mismatch, and success reasons are scalar. | no scheduler |
| `071A.2` | Add proof app/guard. | VM/MIR/EXE proof locks consume lifecycle. | no page-source / OSVM |
| `071A.3` | Update docs/manifests/current pointers. | current pointer guard passes. | no cleanup bundle |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_consume_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

## Implementation Result

`MIMAP-071A` added:

```text
HakoAllocReclaimSchedulerRequestLedgerConsumeReport
HakoAllocReclaimSchedulerRequestLedger.consumePendingRequest(page_id)
apps/hako-alloc-reclaim-scheduler-request-ledger-consume-proof/
tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_consume_guard.sh
docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-consume-ssot.md
```

The route clears one pending modeled scheduler request only when the requested
page id matches. No-pending and page-mismatch rows stay scalar suppressions.

Proof output shape:

```text
no_pending=0,1,0,0,-1,-1
recorded=1,0,1,200
mismatch=0,2,1,1,200,200
consumed=1,0,1,0,200,-1
after=0,1,0,0,-1,-1
inactive=0,0,0,0,0,0,0,0
record_counts=1,1,0
consume_counts=1,3,0,-1,200,1
summary=ok
```

Next row:

```text
MIMAP-072A reclaim scheduler ledger consume closeout guard
```
