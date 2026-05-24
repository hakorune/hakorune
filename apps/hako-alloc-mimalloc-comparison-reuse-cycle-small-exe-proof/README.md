# hako-alloc-mimalloc-comparison-reuse-cycle-small-exe-proof

Row: MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-WORKLOAD-IMPLEMENTATION-295X-001

This proof app mirrors the explicit C mimalloc runner's
`representative-reuse-cycle-small-v0` request shape in `.hako` using the
existing `HakoAllocPageModel`.

Run:

```bash
bash tools/checks/k2_wide_phase295x_reuse_cycle_small_workload_implementation_guard.sh
```

Stop line: this app does not replace the process allocator, install hooks,
activate providers, open TLS/worker behavior, or claim a winner.
