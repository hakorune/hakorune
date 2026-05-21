# hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-extension-follow-on-extension-pilot-proof

Row: MIMAP-522A

Purpose: prove the presentation extension follow-on extension follow-on
extension follow-on extension pilot over the landed MIMAP-516A presentation
extension follow-on extension follow-on extension follow-on pilot report.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-522A --level L2
```

Stop lines:

- no repeated benchmark pack
- no process allocator replacement
- no hook installation
- no backend matcher addition
- no `#[global_allocator]`
- no provider package / DLL generation
