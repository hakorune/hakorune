# hako-alloc allocator comparison benchmark execution preflight diagnostics proof

Row: MIMAP-437A

This proof app consumes the MIMAP-436A benchmark execution preflight inventory
report and publishes observer-only diagnostics for missing preflight inputs or
open execution seams. It does not run benchmarks or replace the process
allocator.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-437A --level L2
```
