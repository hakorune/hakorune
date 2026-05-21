# hako-alloc-allocator-comparison-c-mimalloc-result-ledger-diagnostics-proof

Row: MIMAP-455A

Purpose: prove observer-only diagnostics over the MIMAP-454A C-vs-Hako result
ledger.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-455A --level L2
```

Stop lines:

- no repeated benchmark pack
- no performance / memory-use conclusion
- no process allocator replacement
- no hook installation
- no backend matcher addition
- no `#[global_allocator]`
- no provider package / DLL generation
