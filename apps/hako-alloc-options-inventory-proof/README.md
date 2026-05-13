# hako-alloc-options-inventory-proof

Purpose: M214 proof for read-only allocator options/defaults inventory.

The app classifies static option/default ids through
`HakoAllocOptionsInventory` and proves that mutable options, environment
toggles, allocator policy changes, provider activation, hooks, process
allocator replacement, and reclaim execution remain inactive.

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_options_inventory_guard.sh
```
