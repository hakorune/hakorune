# hako-alloc allocator comparison workload matrix diagnostics proof

Row: MIMAP-431A

This proof app exercises allocator comparison workload matrix diagnostics. It
consumes MIMAP-430A inventory reports, classifies missing workload families,
and keeps benchmark execution, hook installation, backend matcher additions,
process allocator replacement, worker/thread execution, and global allocator
install closed.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-431A --level L2
```
