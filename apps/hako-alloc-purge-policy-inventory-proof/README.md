# hako-alloc-purge-policy-inventory-proof

Purpose: M192 proof for the `hako_alloc` purge/decommit policy inventory.

The app calls `HakoAllocPurgePolicyInventory.classifyLocalPage(...)` over the
small decision matrix needed before any future purge/decommit execution row.

It intentionally avoids:

- `HakoAllocPageSourcePolicy` calls
- OSVM decommit/unreserve/release execution
- heap/page mutation
- provider activation, hooks, or process allocator replacement

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_purge_policy_inventory_guard.sh
```
