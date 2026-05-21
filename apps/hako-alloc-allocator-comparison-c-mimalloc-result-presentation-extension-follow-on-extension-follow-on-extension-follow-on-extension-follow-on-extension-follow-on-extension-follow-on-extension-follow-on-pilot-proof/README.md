# hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-extension-follow-on-pilot-proof

Row: MIMAP-552A

Purpose: prove the comparison-ready presentation extension follow-on extension
follow-on extension follow-on extension follow-on extension follow-on
extension follow-on extension follow-on pilot over the landed MIMAP-546A
deeper-extension-ready report and the fixed MIMAP-550A explicit C mimalloc
contract.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-552A --level L2
```

Stop lines:

- no repeated benchmark pack
- no process allocator replacement
- no hook installation
- no backend matcher addition
- no `#[global_allocator]`
- no provider package / DLL generation
