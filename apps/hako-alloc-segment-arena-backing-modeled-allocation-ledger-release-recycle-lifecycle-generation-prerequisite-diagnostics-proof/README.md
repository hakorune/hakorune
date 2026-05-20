# hako-alloc segment arena backing modeled allocation-ledger release/recycle lifecycle generation prerequisite diagnostics proof

Row: MIMAP-333A

This proof observes MIMAP-332A model-only lifecycle generation prerequisite
facts and keeps real lifecycle generation closed.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-333A
```

Stop lines: no new prerequisite rows from the diagnostic owner, no real
lifecycle generation, no release/recycle execution, no pointer residence, no
arena backing mutation, no segment-map mutation, no atomic/OSVM/worker/provider
or backend matcher activation.
