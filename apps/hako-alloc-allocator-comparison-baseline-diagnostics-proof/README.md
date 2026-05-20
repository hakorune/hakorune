# hako-alloc allocator comparison baseline diagnostics proof

Row: MIMAP-428A

This proof app exercises allocator comparison baseline diagnostics. It consumes
MIMAP-427A inventory reports, classifies missing comparison inputs, and keeps
benchmark execution, hook installation, backend matcher additions, process
allocator replacement, worker/thread execution, and global allocator install
closed.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-428A --level L2
```
