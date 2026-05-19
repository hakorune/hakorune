# hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-second-release-diagnostic-proof

Row: MIMAP-296A

This proof app observes a MIMAP-292A modeled allocation-ledger release-applied
recycle row and proves that a second release attempt after that modeled recycle
is rejected in scalar/model diagnostics. It does not record new release-applied
recycle rows and does not open real arena backing release, pointer lookup,
segment-map, atomic bitmap, OSVM, worker, provider, or backend matcher seams.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-296A
```
