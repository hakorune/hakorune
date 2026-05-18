# hako-alloc segment arena backing modeled residence arena-binding proof

Row: MIMAP-252A

This proof app validates the scalar/model binding between an accepted modeled
no-escape address residence report and an accepted scalar requirement matrix
for the same segment and arena.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-252A
```

Stop lines: no real pointer residence, no pointer-derived lookup, no arena
backing allocation, no real segment-map mutation, no atomic bitmap execution,
no OSVM/page-source calls, no worker/provider activation, and no backend
owner-name matcher.
