# hako-alloc segment arena backing modeled allocation-ledger release intent diagnostics proof

Row: MIMAP-285A

This proof app validates observer-only diagnostics over the MIMAP-284A modeled
release-intent inventory.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-285A
```

Stop lines: no new release-intent rows, no real pointer residence, no
pointer-derived lookup, no real arena backing allocation/release, no real
segment-map mutation, no atomic bitmap execution, no OSVM/page-source calls, no
worker/provider activation, and no backend owner-name matcher.
