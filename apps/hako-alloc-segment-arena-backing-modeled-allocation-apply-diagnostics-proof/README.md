# hako-alloc segment arena backing modeled allocation apply diagnostics proof

Row: MIMAP-273A

This proof app validates observer-only diagnostics over the MIMAP-272A modeled
allocation-apply inventory.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-273A
```

Stop lines: no new allocation-apply inventory rows, no real pointer residence,
no pointer-derived lookup, no real arena backing allocation, no real
segment-map mutation, no atomic bitmap execution, no OSVM/page-source calls,
no worker/provider activation, and no backend owner-name matcher.
