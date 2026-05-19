# hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-diagnostics-proof

Row: MIMAP-293A

This proof app observes MIMAP-292A modeled allocation-ledger release-applied recycle
inventory counters and last-report facts. It does not record new release-applied recycle
rows and does not open real arena backing release, pointer lookup, segment-map,
atomic bitmap, OSVM, worker, provider, or backend matcher seams.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-293A
```
