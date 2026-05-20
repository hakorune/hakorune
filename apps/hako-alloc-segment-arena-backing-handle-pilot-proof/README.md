# hako-alloc segment arena backing handle pilot proof

Row: MIMAP-345A

This proof records a bounded arena backing handle token from the accepted
no-escape pointer residence pilot. It keeps pointer-derived lookup, dereference,
arena release/recycle, segment-map mutation, atomic bitmap, OSVM, worker/TLS,
provider activation, and backend matchers closed.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-345A
```
