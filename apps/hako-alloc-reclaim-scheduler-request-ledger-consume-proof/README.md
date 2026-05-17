# hako-alloc-reclaim-scheduler-request-ledger-consume-proof

Purpose: MIMAP-071A proof for the scalar reclaim scheduler request ledger
consume route.

The app records one pending modeled scheduler request, rejects no-pending and
page-mismatch consumes, then consumes the matching pending request without
running a scheduler.

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_consume_guard.sh
```
