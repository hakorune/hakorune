# hako-alloc-allocator-comparison-c-mimalloc-result-explicit-runner-planning-pilot-proof

Row: MIMAP-566A

Purpose: prove the terminal explicit-runner planning pilot contract while
keeping benchmark and execution seams closed.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-566A --level L2
```

Stop lines:

- no repeated benchmark pack
- no process allocator replacement
- no hook installation
- no backend matcher addition
- no `#[global_allocator]`
- no provider package / DLL generation
- no explicit C mimalloc runner execution
