# hako-alloc-allocator-comparison-c-mimalloc-result-presentation-only-extension-pilot-proof

Row: MIMAP-560A

Purpose: prove the presentation-only extension pilot over the landed MIMAP-552A
comparison-ready report and the closed MIMAP-550A explicit C mimalloc
comparison plan seam.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-560A --level L2
```

Stop lines:

- no repeated benchmark pack
- no process allocator replacement
- no hook installation
- no backend matcher addition
- no `#[global_allocator]`
- no provider package / DLL generation
- no explicit C mimalloc runner execution
