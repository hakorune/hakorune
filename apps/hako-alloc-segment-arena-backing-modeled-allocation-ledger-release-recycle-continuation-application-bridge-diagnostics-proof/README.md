# hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-recycle-continuation-application-bridge-diagnostics-proof

Row: MIMAP-305A

This proof app observes a scalar/model continuation application bridge from
MIMAP-304A. It does not record another application row or open real lifecycle
generation, real arena backing release/recycle, pointer lookup, segment-map,
atomic bitmap, OSVM, worker, provider, or backend matcher seams.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-305A
```
