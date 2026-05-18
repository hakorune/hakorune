# hako-alloc segment arena backing modeled source bridge proof

Row: MIMAP-260A

This proof app validates the scalar/model backing source bridge recorded from
an accepted modeled arena-slot report.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-260A
```

Stop lines: no real pointer residence, no pointer-derived lookup, no arena
backing allocation, no real segment-map mutation, no atomic bitmap execution,
no OSVM/page-source calls, no worker/provider activation, and no backend
owner-name matcher.
