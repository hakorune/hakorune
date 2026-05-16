# hako-alloc-reclaim-remote-free-drain-execution-proof

Purpose: MIMAP-057A proof for the first modeled reclaim remote-free drain
execution route.

The app composes `HakoAllocReclaimRemoteFreeDrainExecution`, proves that one
pending remote-free entry decrements only the executor-local modeled pending
count, and proves that no-work, invalid/inconsistent contract facts, and
insufficient budget stay blocked without broader reclaim behavior.

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_reclaim_remote_free_drain_execution_guard.sh
```
