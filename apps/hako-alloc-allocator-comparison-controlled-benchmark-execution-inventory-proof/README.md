# hako-alloc allocator comparison controlled benchmark execution inventory proof

Row: MIMAP-440A

This proof app fixes the first controlled benchmark execution shape for the
allocator comparison lane. It records the explicit benchmark runner, workload
source, measurement source, output contract, evidence storage, and
representative-run selection while keeping process allocator replacement,
hooks, backend matcher additions, global allocator installation, and worker
execution closed.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-440A --level L2
```
