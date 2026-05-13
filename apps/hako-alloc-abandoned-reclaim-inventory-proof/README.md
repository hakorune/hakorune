# hako-alloc-abandoned-reclaim-inventory-proof

Purpose: M213 proof for abandoned/reclaim inventory vocabulary.

The app classifies scalar owner/page facts through
`HakoAllocAbandonedReclaimInventory`.

It intentionally avoids:

- thread scheduling
- atomics expansion
- reclaim execution
- page-source calls
- decommit/recommit
- unreserve or OSVM release
- provider activation, hooks, or process allocator replacement

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_abandoned_reclaim_inventory_guard.sh
```

