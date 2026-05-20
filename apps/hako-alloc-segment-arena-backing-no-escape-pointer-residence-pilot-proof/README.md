# hako-alloc segment arena backing no-escape pointer residence pilot proof

Row: MIMAP-344A

This proof records a private proof-scope no-escape pointer residence token from
the accepted remaining execution prerequisite ledger. It keeps pointer-derived
lookup, dereference, arena release/recycle, segment-map mutation, atomic bitmap,
OSVM, worker/TLS, provider activation, and backend matchers closed.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-344A
```

Stop lines: no pointer-derived lookup, no dereference, no arena backing
release/recycle, no segment-map mutation, no atomic/OSVM/worker/provider/backend
matcher activation.
