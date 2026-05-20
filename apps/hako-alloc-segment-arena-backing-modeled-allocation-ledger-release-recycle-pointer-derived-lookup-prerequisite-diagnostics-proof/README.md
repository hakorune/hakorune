# hako-alloc segment arena backing modeled allocation-ledger release/recycle pointer-derived lookup prerequisite diagnostics proof

Row: MIMAP-341A

This proof observes the model-only pointer-derived lookup prerequisite facts and
publishes observer-only scalar diagnostics. The row also closes the model-only
pointer-derived lookup prerequisite pack.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-341A
```

Stop lines: no new prerequisite rows, no real release/recycle execution, no
raw pointer residence, no pointer-derived lookup, no arena backing mutation, no
segment-map mutation, no atomic/OSVM/worker/provider/backend matcher activation.
