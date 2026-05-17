# hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-proof

This proof app fixes `MIMAP-130A`.

It proves that a successful modeled local-free reuse report can be recorded as
a dedicated scalar live reuse allocation ledger row without widening the
bump-shaped modeled allocation ledger contract.

Run:

```bash
tools/checks/run_proof_app.sh --only MIMAP-130A
```
