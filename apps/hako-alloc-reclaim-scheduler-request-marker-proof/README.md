# hako-alloc-reclaim-scheduler-request-marker-proof

Purpose: MIMAP-064A proof for the scalar reclaim scheduler request marker.

The app composes `HakoAllocReclaimSchedulerRequestMarker` and proves that a
scheduler request marker is set only after reclaim completion succeeds and the
modeled scheduler request is enabled. Completion-blocked and scheduler-disabled
rows remain scalar suppressions.

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_marker_guard.sh
```
