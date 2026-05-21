# hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-pilot-proof

Row: MIMAP-486A

Purpose: prove the presentation extension pilot over the landed MIMAP-480A
presentation follow-on pilot report.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-486A --level L2
```

Stop lines:

- no repeated benchmark pack
- no process allocator replacement
- no hook installation
- no backend matcher addition
- no `#[global_allocator]`
- no provider package / DLL generation
