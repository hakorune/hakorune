# hako-alloc allocator comparison controlled benchmark execution diagnostics proof

Row: MIMAP-441A

This proof app fixes observer-only diagnostics for the MIMAP-440A controlled
benchmark execution inventory. It classifies missing shape inputs and open
closed-seam inputs while keeping benchmark execution, process allocator
replacement, hooks, backend matcher additions, global allocator installation,
and worker execution closed.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-441A --level L2
```
