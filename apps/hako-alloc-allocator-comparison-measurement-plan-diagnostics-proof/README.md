# hako-alloc allocator comparison measurement plan diagnostics proof

Row: MIMAP-434A

This proof app consumes the MIMAP-433A measurement-plan inventory report and
publishes observer-only diagnostics for missing or invalid measurement inputs.
It does not run benchmarks or replace the process allocator.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-434A --level L2
```
