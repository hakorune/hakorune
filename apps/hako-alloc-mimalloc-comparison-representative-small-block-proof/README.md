# hako-alloc-mimalloc-comparison-representative-small-block-proof

Row: MIMALLOC-COMPARISON-SAME-WORKLOAD-PACK-001

This proof app mirrors the explicit C mimalloc runner's
`representative-small-block-v0` request shape in `.hako` using the existing
`HakoAllocPageModel`.

Run:

```bash
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_same_workload_memory_report_guard.sh
```

Stop line: this app does not replace the process allocator, install hooks,
activate providers, open TLS/worker behavior, or claim a winner.
