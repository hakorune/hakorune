# hako-alloc-allocator-comparison-c-mimalloc-result-reporting-inventory-proof

Row: MIMAP-460A

Purpose: prove a stable scalar reporting inventory over the MIMAP-458A C-vs-Hako
result summary diagnostics.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-460A --level L2
```

Stop lines:

- no repeated benchmark pack
- no performance / memory-use conclusion
- no process allocator replacement
- no hook installation
- no backend matcher addition
- no `#[global_allocator]`
- no provider package / DLL generation
