# hako-alloc allocator comparison C mimalloc execution diagnostics proof

Row: MIMAP-449A

This proof app diagnoses the MIMAP-448A C mimalloc execution inventory report.
It does not execute C mimalloc and keeps process allocator replacement, hooks,
backend matcher additions, global allocator installation, and worker execution
closed.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-449A --level L2
```
