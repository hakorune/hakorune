# hako-alloc allocator comparison C mimalloc explicit runner evidence diagnostics proof

Row: MIMAP-452A

This proof app diagnoses MIMAP-451A explicit runner evidence reports. It does
not rerun the C mimalloc runner and keeps process allocator replacement, hooks,
backend matcher additions, global allocator installation, provider package
generation, hidden discovery, and worker execution closed.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-452A --level L2
```
