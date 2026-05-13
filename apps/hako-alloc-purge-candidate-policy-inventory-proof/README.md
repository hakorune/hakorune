# hako-alloc-purge-candidate-policy-inventory-proof

Purpose: M211 proof for the `hako_alloc` purge candidate policy inventory.

The app builds M207 lifecycle reports manually through
`HakoAllocPageLifecycleInvariantObserver.report(...)`, then classifies those
reports through `HakoAllocPurgeCandidatePolicyInventory`.

It intentionally avoids:

- heap queue scans
- `observeHeapPage(...)` calls from the M211 owner
- decommit/recommit execution
- scheduler behavior
- page-source calls
- heap/page/marker mutation
- OSVM unreserve/release
- provider activation, hooks, or process allocator replacement

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_purge_candidate_policy_inventory_guard.sh
```

