# hako-alloc segment arena backing pointer-derived lookup execution pilot proof

Row: MIMAP-346A

This proof derives a bounded pointer lookup fact from the accepted arena backing
handle pilot. It keeps dereference, arena release/recycle, segment-map mutation,
atomic bitmap, OSVM, worker/TLS, provider activation, and backend matchers
closed.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-346A
```
