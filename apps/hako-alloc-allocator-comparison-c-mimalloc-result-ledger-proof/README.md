# hako-alloc-allocator-comparison-c-mimalloc-result-ledger-proof

Row: MIMAP-454A

Purpose: prove the narrow allocator comparison result ledger over existing Hako
representative benchmark diagnostics and explicit C mimalloc runner evidence
diagnostics.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-454A --level L2
```

Stop lines:

- no repeated benchmark pack
- no process allocator replacement
- no hook installation
- no backend matcher addition
- no `#[global_allocator]`
- no provider package / DLL generation
- no worker/thread execution
