# hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-execution-intent-marker-diagnostics-proof

Row: MIMAP-317A

This proof app observes model-only release/recycle execution intent marker
facts and publishes scalar diagnostics. It does not record new intent marker
rows or execute real release/recycle behavior.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-317A
```
