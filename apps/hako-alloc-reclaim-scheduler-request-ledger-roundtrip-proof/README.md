# hako-alloc-reclaim-scheduler-request-ledger-roundtrip-proof

Purpose: MIMAP-074A proof for the scalar reclaim scheduler request ledger
roundtrip route.

The app records one modeled scheduler request and consumes the matching pending
request through an allocator-owned wrapper. It also proves scheduler-disabled
and completion-blocked rows stay scalar suppressions.

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_roundtrip_guard.sh
```
