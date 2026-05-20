# hako-alloc segment arena backing modeled allocation-ledger release/recycle pointer residence prerequisite proof

Row: MIMAP-336A

This proof records the model-only pointer residence prerequisite from an
accepted release/recycle lifecycle generation prerequisite report.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-336A
```

Stop lines: no real release/recycle execution, no real lifecycle generation,
no raw pointer residence, no pointer-derived lookup, no arena backing mutation,
no segment-map mutation, no atomic/OSVM/worker/provider/backend matcher
activation.
