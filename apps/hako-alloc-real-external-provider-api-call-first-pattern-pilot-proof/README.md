# hako-alloc-real-external-provider-api-call-first-pattern-pilot-proof

Row: MIMAP-415A

This proof app exercises the first-pattern real external provider API call
pilot. It consumes the MIMAP-410A preflight report, records explicit real-call
pilot evidence, and keeps host allocator replacement, hooks, backend matcher
additions, worker/thread execution, and global allocator install closed.

Run:

```text
bash tools/checks/run_proof_app.sh --only MIMAP-415A
```
