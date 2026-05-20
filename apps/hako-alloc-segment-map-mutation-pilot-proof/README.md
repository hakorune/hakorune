# hako-alloc segment-map mutation pilot proof

Row: MIMAP-347A

This proof records a bounded segment-map mutation fact from an accepted
pointer-derived lookup execution report. It keeps dereference, arena
release/recycle, atomic bitmap, OSVM, worker/TLS, provider activation, and
backend matchers closed.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-347A
```
