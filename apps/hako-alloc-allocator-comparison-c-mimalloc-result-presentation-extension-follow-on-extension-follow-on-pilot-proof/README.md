# hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-pilot-proof

Row: MIMAP-504A

Purpose: prove the presentation extension follow-on extension follow-on pilot
over the landed MIMAP-498A presentation extension follow-on extension pilot
report.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-504A --level L2
```

Stop lines:

- no repeated benchmark pack
- no process allocator replacement
- no hook installation
- no backend matcher addition
- no `#[global_allocator]`
- no provider package / DLL generation
