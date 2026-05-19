# hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-diagnostics-proof

Row: MIMAP-281A

This proof app observes the modeled allocation-ledger release-candidate
inventory and publishes scalar diagnostic summary facts without recording new
release-candidate rows or opening real allocator execution.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-281A
```
