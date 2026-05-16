# hako-alloc-reclaim-owner-transfer-execution-proof

Purpose: MIMAP-055A proof for the first guarded reclaim owner-transfer
execution route.

The app composes `HakoAllocReclaimOwnerTransferExecution`, proves that one
ready page can update only the executor-local modeled owner token, and proves
that contract-blocked and claim-blocked requests leave that modeled owner
unchanged.

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_reclaim_owner_transfer_execution_guard.sh
```
