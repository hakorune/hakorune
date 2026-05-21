# hako-alloc-allocator-comparison-c-mimalloc-result-summary-inventory-proof

Row: MIMAP-457A

Purpose: prove a scalar summary inventory over the MIMAP-454A C-vs-Hako result
ledger and the MIMAP-455A diagnostics.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-457A --level L2
```

Stop lines:

- no repeated benchmark pack
- no performance / memory-use conclusion
- no process allocator replacement
- no hook installation
- no backend matcher addition
- no `#[global_allocator]`
- no provider package / DLL generation
