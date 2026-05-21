# hako-alloc-allocator-comparison-c-mimalloc-result-reporting-diagnostics-proof

Row: MIMAP-461A

Purpose: prove observer-only diagnostics over the MIMAP-460A C-vs-Hako result
reporting inventory.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-461A --level L2
```

Stop lines:

- no repeated benchmark pack
- no performance / memory-use conclusion
- no process allocator replacement
- no hook installation
- no backend matcher addition
- no `#[global_allocator]`
- no provider package / DLL generation
