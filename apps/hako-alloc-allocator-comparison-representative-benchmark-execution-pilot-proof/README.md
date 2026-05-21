# hako-alloc allocator comparison representative benchmark execution pilot proof

Row: MIMAP-444A

This proof app opens the first bounded representative allocator comparison
benchmark execution seam. It runs a small `HakoAllocProductionFacade` workload
and records scalar metrics while keeping process allocator replacement, hooks,
backend matcher additions, global allocator installation, and worker execution
closed.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-444A --level L2
```
