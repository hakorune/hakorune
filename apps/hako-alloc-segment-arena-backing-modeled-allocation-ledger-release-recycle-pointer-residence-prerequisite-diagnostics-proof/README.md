# hako-alloc segment arena backing modeled allocation-ledger release/recycle pointer residence prerequisite diagnostics proof

Row: MIMAP-337A

This proof observes the model-only pointer residence prerequisite facts and
publishes observer-only scalar diagnostics.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-337A
```

Stop lines: no new prerequisite rows, no real release/recycle execution, no raw
pointer residence, no pointer-derived lookup, no arena backing mutation, no
segment-map mutation, no atomic/OSVM/worker/provider/backend matcher activation.
