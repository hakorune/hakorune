# 293x-561 MIMAP-074A Reclaim Scheduler Request Ledger Roundtrip Route

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-074A` is the allocator behavior row selected by `MIMAP-073A`.

The request ledger record route and consume route are now separately guarded.
This row should add one allocator-owned scalar roundtrip that records a modeled
reclaim scheduler request and consumes the same pending request, proving the
local lifecycle without opening real scheduler substrate.

## Scope

- Add a small `.hako` owner that composes
  `HakoAllocReclaimSchedulerRequestLedger`.
- Prove the success path:

```text
record scheduler request
  -> pending request exists
  -> consume matching page id
  -> pending request cleared
```

- Preserve scalar suppressed rows for scheduler-disabled and completion-blocked
  record attempts.
- Keep record/consume ownership local to the allocator request ledger.
- Add a proof app, manifest row, focused guard, and docs index entry.

## Stop Lines

- No real thread scheduling.
- No worker spawning.
- No source-level concurrency feature change.
- No page-source call.
- No OSVM unreserve / release.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.
- No cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `074A.1` | Add roundtrip SSOT and owner. | owner composes record + consume only. | no scheduler substrate |
| `074A.2` | Add proof app and manifest entry. | VM proof shows success and suppressed rows. | no backend matcher |
| `074A.3` | Add focused guard and docs index entry. | guard checks stop lines and proof output. | no broad cleanup |
| `074A.4` | Update current pointers. | current pointer guard passes. | no provider activation |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_roundtrip_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

## Implementation Result

`MIMAP-074A` added:

```text
HakoAllocReclaimSchedulerRequestLedgerRoundtripReport
HakoAllocReclaimSchedulerRequestLedgerRoundtrip.recordAndConsumeSchedulerRequest(...)
apps/hako-alloc-reclaim-scheduler-request-ledger-roundtrip-proof/
tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_roundtrip_guard.sh
docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-roundtrip-ssot.md
```

The route composes the existing scheduler request ledger. A success row records
one modeled scheduler request and consumes the same pending page id, leaving
the pending request cleared. Scheduler-disabled and completion-blocked rows
remain scalar suppressions.

Proof output shape:

```text
success=1,1,0,1,0,300,1,1,1,1,2
disabled=0,0,2,2,0,0
blocked=0,0,1,1,0,0
inactive=0,0,0,0,0,0,0,0
route_counts=3,1,2,302,1
ledger_counts=3,1,2,1,0,-1
summary=ok
```

Next row:

```text
MIMAP-075A reclaim scheduler request ledger roundtrip closeout guard
```
