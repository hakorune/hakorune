# hako-alloc-thread-heap-owner-inventory-proof

Purpose: M215 proof for read-only thread heap owner-token inventory.

The app classifies scalar owner-token facts through
`HakoAllocThreadHeapOwnerInventory` and proves that scheduling, atomic claim,
remote-free drain, owner mutation, reclaim execution, page-source calls,
unreserve, and OSVM release remain inactive.

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_thread_heap_owner_inventory_guard.sh
```
