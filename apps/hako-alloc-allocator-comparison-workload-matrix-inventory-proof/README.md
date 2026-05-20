# hako-alloc allocator comparison workload matrix inventory proof

Row: MIMAP-430A

This proof app inventories the workload families required before comparing
`.hako` / `hako_alloc` against C mimalloc for throughput and memory usage. It
does not run benchmarks or replace the process allocator.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-430A --level L2
```
