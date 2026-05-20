# hako-alloc segment arena backing modeled allocation-ledger release/recycle execution support requirement matrix diagnostics proof

Row: MIMAP-329A

This proof observes MIMAP-328A support requirement matrix facts and publishes
observer-only diagnostics.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-329A
```

Stop lines:

- no new requirement matrix row recording from the diagnostic owner
- no real release/recycle execution
- no lifecycle generation, pointer residence, segment-map mutation, atomic
  bitmap execution, OSVM calls, worker scheduling, provider activation, or
  backend matcher
