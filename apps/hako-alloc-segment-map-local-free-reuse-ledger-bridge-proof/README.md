# hako-alloc-segment-map-local-free-reuse-ledger-bridge-proof

Row: MIMAP-192A
Profile: scalar-mir

This proof app connects the segment-map local-free reuse bridge to the existing
modeled local-free reuse ledger owner. It keeps real segment execution, raw
pointer residence, arena backing, atomics, OSVM/page-source calls, provider
activation, and backend shortcuts closed.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-192A
```
