# hako-alloc segment arena backing modeled allocation apply proof

Row: MIMAP-272A

This proof app validates a scalar/model allocation apply over accepted segment
arena backing modeled allocation-plan reports.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-272A
```

Stop lines: no real pointer residence, no pointer-derived lookup, no real arena
backing allocation, no real segment-map mutation, no atomic bitmap execution,
no OSVM/page-source calls, no worker/provider activation, and no backend
owner-name matcher.
