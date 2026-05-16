# hako-alloc-reclaim-completion-marker-proof

Purpose: MIMAP-060A proof for the scalar reclaim completion marker route.

The app composes `HakoAllocReclaimCompletionMarker` and proves that completion is
marked only after the post-drain owner-transfer integration route succeeds. It
also fixes blocked rows for pending remote-free work and failed owner transfer,
while page-source calls, OSVM release, scheduling, provider activation, and host
allocator replacement stay inactive.

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_reclaim_completion_marker_guard.sh
```
