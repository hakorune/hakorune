# hako-alloc-reclaim-remote-free-drain-contract-proof

Purpose: MIMAP-056A proof for the no-execution reclaim remote-free drain
contract.

The app classifies clear, pending, over-budget, invalid, and inconsistent
remote-free facts through `HakoAllocReclaimRemoteFreeDrainContract`. It proves
that pending remote-free work blocks broader reclaim while drain execution,
thread scheduling, page-source/OSVM seams, provider activation, and production
owner mutation remain inactive.

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_reclaim_remote_free_drain_contract_guard.sh
```
