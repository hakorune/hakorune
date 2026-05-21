# hako-alloc-allocator-comparison-c-mimalloc-result-first-conclusion-preflight-proof

Row: MIMAP-464A

Purpose: prove a guarded first performance / memory-use conclusion preflight over
the landed MIMAP-461A C-vs-Hako result reporting diagnostics.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-464A --level L2
```

Stop lines:

- no repeated benchmark pack
- no performance / memory-use conclusion
- no process allocator replacement
- no hook installation
- no backend matcher addition
- no `#[global_allocator]`
- no provider package / DLL generation
