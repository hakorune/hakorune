# hako-alloc-reclaim-scheduler-request-ledger-proof

Purpose: MIMAP-068A proof for the scalar reclaim scheduler request ledger.

The app composes `HakoAllocReclaimSchedulerRequestLedger` and proves that a
single pending modeled scheduler request is recorded only after the reclaim
scheduler request marker succeeds. Marker-blocked, scheduler-disabled, and
already-pending rows remain scalar suppressions.

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_guard.sh
```
