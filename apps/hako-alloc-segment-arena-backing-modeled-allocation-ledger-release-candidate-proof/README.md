# hako-alloc segment arena backing modeled allocation-ledger release candidate proof

Row: MIMAP-280A

This proof app validates a scalar/model release-candidate inventory over
accepted segment arena backing modeled allocation-ledger reports.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-280A
```

Stop lines: no real pointer residence, no pointer-derived lookup, no real arena
backing allocation/release, no real segment-map mutation, no atomic bitmap
execution, no OSVM/page-source calls, no worker/provider activation, and no
backend owner-name matcher.
