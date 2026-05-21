# hako-alloc-allocator-comparison-c-mimalloc-result-presentation-only-conclusion-pilot-proof

Row: MIMAP-474A

Purpose: prove the presentation-only conclusion pilot over the landed MIMAP-468A
first conclusion pilot report.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-474A --level L2
```

Stop lines:

- no repeated benchmark pack
- no process allocator replacement
- no hook installation
- no backend matcher addition
- no `#[global_allocator]`
- no provider package / DLL generation
