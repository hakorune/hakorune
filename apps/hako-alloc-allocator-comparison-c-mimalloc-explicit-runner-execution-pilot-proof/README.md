# hako-alloc allocator comparison C mimalloc explicit runner execution pilot proof

Row: MIMAP-451A

This proof app records explicit C mimalloc runner execution evidence after the
MIMAP-448A / MIMAP-449A readiness package. It keeps process allocator
replacement, hooks, backend matcher additions, global allocator installation,
provider package generation, hidden discovery, and worker execution closed.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-451A --level L2
```
