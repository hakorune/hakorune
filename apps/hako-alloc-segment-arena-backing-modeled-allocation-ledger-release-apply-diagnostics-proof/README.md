# hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-apply-diagnostics-proof

Row: MIMAP-289A

This proof app observes MIMAP-288A modeled allocation-ledger release-apply
inventory counters and last-report facts. It does not record new release-apply
rows and does not open real arena backing release, pointer lookup, segment-map,
atomic bitmap, OSVM, worker, provider, or backend matcher seams.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-289A
```
