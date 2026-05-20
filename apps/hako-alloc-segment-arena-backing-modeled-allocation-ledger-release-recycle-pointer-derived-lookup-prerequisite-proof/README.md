# hako-alloc segment arena backing modeled allocation-ledger release/recycle pointer-derived lookup prerequisite proof

Row: MIMAP-340A

This proof records the model-only pointer-derived lookup prerequisite from an
accepted pointer residence prerequisite report.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-340A
```

Stop lines: no real release/recycle execution, no raw pointer residence, no
pointer-derived lookup, no arena backing mutation, no segment-map mutation, no
atomic/OSVM/worker/provider/backend matcher activation.
