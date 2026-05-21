# hako-alloc allocator comparison C mimalloc execution inventory proof

Row: MIMAP-448A

This proof app inventories explicit C mimalloc comparison execution inputs. It
does not execute C mimalloc and keeps process allocator replacement, hooks,
backend matcher additions, global allocator installation, and worker execution
closed.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-448A --level L2
```
