# hako-alloc segment arena backing modeled arena slot proof

Row: MIMAP-256A

This proof app validates the scalar/model arena-slot inventory recorded from an
accepted modeled residence arena-binding report.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-256A
```

Stop lines: no real pointer residence, no pointer-derived lookup, no arena
backing allocation, no real segment-map mutation, no atomic bitmap execution,
no OSVM/page-source calls, no worker/provider activation, and no backend
owner-name matcher.
