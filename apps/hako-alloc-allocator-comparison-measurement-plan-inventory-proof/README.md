# hako-alloc allocator comparison measurement plan inventory proof

Row: MIMAP-433A

This proof app inventories explicit measurement plan inputs required before
`.hako` / `hako_alloc` can be compared against C mimalloc for throughput and
memory usage. It does not run benchmarks or replace the process allocator.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-433A --level L2
```
