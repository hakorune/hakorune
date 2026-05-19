# hako-alloc segment arena backing modeled allocation-ledger release apply proof

Row: MIMAP-288A

This proof app validates a scalar/model release-apply inventory over accepted
segment arena backing modeled allocation-ledger release-intent reports.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-288A
```

Stop lines: no real pointer residence, no pointer-derived lookup, no real arena
backing allocation/release, no real segment-map mutation, no atomic bitmap
execution, no OSVM/page-source calls, no worker/provider activation, and no
backend owner-name matcher.
