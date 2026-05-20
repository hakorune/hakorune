# hako-alloc-provider-call-external-api-adapter-inventory-proof

Row: MIMAP-400A

This proof app exercises the provider-call external API adapter inventory. It
records adapter presence/readiness after the stub execution seam while external
provider API calls, host allocator replacement, hooks, backend matcher
additions, worker/thread execution, and global allocator install remain closed.

Run:

```text
bash tools/checks/run_proof_app.sh --only MIMAP-400A
```
