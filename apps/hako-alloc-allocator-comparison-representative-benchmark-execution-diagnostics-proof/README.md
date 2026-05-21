# hako-alloc allocator comparison representative benchmark execution diagnostics proof

Row: MIMAP-445A

This proof app diagnoses the MIMAP-444A representative benchmark execution
pilot report. It classifies accepted execution evidence and blocked execution
reports while keeping process allocator replacement, hooks, backend matcher
additions, global allocator installation, C mimalloc execution, and worker
execution closed.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-445A --level L2
```
