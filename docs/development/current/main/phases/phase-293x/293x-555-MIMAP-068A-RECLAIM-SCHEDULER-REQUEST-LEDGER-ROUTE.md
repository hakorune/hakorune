# 293x-555 MIMAP-068A Reclaim Scheduler Request Ledger Route

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-068A` is the allocator behavior row selected by `MIMAP-067A`.

The row adds a scalar allocator-owned request ledger after the MIMAP-064A
reclaim scheduler request marker. It records that a completed reclaim would
request modeled scheduler handoff, but it must not execute scheduling.

## Scope

- Add one narrow `.hako` owner for the request ledger.
- Compose `HakoAllocReclaimSchedulerRequestMarker`.
- Record at most one pending modeled scheduler request with scalar page/reason
  facts and counters.
- Add one proof app and one focused guard.
- Update the hako_alloc module manifest / memory README / proof manifest /
  check-script index as needed.

## Proposed Owner

```text
lang/src/hako_alloc/memory/reclaim_scheduler_request_ledger_box.hako
```

Responsibilities:

```text
compose HakoAllocReclaimSchedulerRequestMarker
record one pending scheduler request only when marker should_request_schedule=1
report suppress reasons from the marker when no request is recorded
expose scalar counters for attempts, recorded requests, suppressed requests
preserve inactive production-facing scheduler/provider/source-concurrency flags
```

Non-responsibilities:

```text
real thread scheduling
worker spawning or wake/sleep behavior
source-level co / nowait / await / Channel / sync box / context / worker_local
page-source or OSVM release/unreserve
provider activation / hooks / host allocator replacement
backend app/name matcher
```

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `068A.1` | Add the request ledger owner. | scalar report records pending request only for successful marker rows. | no real scheduler |
| `068A.2` | Add proof app. | VM proof prints recorded, suppressed, disabled, and inactive rows. | no page-source / OSVM |
| `068A.3` | Add focused guard/index wiring. | guard proves no source-concurrency/provider/backend matcher leak. | no cleanup bundle |
| `068A.4` | Update current pointers. | current pointer guard passes. | no extra row |

## Required Evidence

```text
NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 target/debug/hakorune --backend vm apps/hako-alloc-reclaim-scheduler-request-ledger-proof/main.hako
bash tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```
