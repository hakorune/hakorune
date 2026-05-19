# hako-alloc segment arena backing modeled allocation plan proof

Row: MIMAP-268A

This proof app validates a scalar/model allocation plan over accepted segment
arena backing source-accounting reports.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-268A
```

Stop lines: no real pointer residence, no pointer-derived lookup, no real arena
backing allocation, no real segment-map mutation, no atomic bitmap execution,
no OSVM/page-source calls, no worker/provider activation, and no backend
owner-name matcher.
