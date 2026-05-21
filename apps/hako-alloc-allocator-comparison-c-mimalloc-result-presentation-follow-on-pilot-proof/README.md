# hako-alloc-allocator-comparison-c-mimalloc-result-presentation-follow-on-pilot-proof

Row: MIMAP-480A

Purpose: prove the presentation follow-on pilot over the landed MIMAP-474A
presentation-only conclusion pilot report.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-480A --level L2
```

Stop lines:

- no repeated benchmark pack
- no process allocator replacement
- no hook installation
- no backend matcher addition
- no `#[global_allocator]`
- no provider package / DLL generation
