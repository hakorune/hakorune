# hako-alloc segment arena backing modeled source accounting proof

Row: MIMAP-264A

This proof app validates scalar/model accounting over accepted modeled source
bridge reports.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-264A
```

Stop lines: no real pointer residence, no pointer-derived lookup, no arena
backing allocation, no real segment-map mutation, no atomic bitmap execution,
no OSVM/page-source calls, no worker/provider activation, and no backend
owner-name matcher.
