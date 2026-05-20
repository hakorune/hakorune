# hako-alloc atomic bitmap pilot proof

Row: MIMAP-348A

This proof records a bounded atomic bitmap fact from an accepted segment-map
mutation report. It does not use real atomic primitives, dereference, arena
release/recycle, OSVM, worker/TLS, provider activation, or backend matchers.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-348A
```
