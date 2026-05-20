# hako-alloc-allocator-comparison-baseline-inventory-proof

Row: MIMAP-427A

This proof app validates the allocator comparison baseline inventory row. It
records the explicit inputs needed before comparing `.hako` / `hako_alloc`
against C mimalloc for throughput and memory usage.

Run:

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_baseline_inventory_guard.sh --level L2
```

Stop lines:

- no hook installation
- no backend matcher additions
- no process allocator replacement
- no `#[global_allocator]`
- no benchmark execution
