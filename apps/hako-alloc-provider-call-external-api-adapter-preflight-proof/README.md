# hako-alloc-provider-call-external-api-adapter-preflight-proof

Row: MIMAP-402A

This proof app exercises the provider-call external API adapter preflight. It
records preflight readiness after the adapter inventory while external provider
API calls, host allocator replacement, hooks, backend matcher additions,
worker/thread execution, and global allocator install remain closed.

Run:

```text
bash tools/checks/run_proof_app.sh --only MIMAP-402A
```
