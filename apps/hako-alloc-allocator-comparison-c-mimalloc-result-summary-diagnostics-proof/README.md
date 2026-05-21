# hako-alloc-allocator-comparison-c-mimalloc-result-summary-diagnostics-proof

Row: MIMAP-458A

Purpose: prove observer-only diagnostics over the MIMAP-457A C-vs-Hako result
summary inventory.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-458A --level L2
```

Stop lines:

- no repeated benchmark pack
- no performance / memory-use conclusion
- no process allocator replacement
- no hook installation
- no backend matcher addition
- no `#[global_allocator]`
- no provider package / DLL generation
