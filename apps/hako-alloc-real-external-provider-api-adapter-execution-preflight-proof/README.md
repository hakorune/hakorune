# hako-alloc-real-external-provider-api-adapter-execution-preflight-proof

Row: MIMAP-410A

This proof app exercises the real external provider API adapter execution
preflight. It records readiness after the model-space external API call stub
execution closeout while actual external provider API calls, host allocator
replacement, hooks, backend matcher additions, worker/thread execution, and
global allocator install remain closed.

Run:

```text
bash tools/checks/run_proof_app.sh --only MIMAP-410A
```
