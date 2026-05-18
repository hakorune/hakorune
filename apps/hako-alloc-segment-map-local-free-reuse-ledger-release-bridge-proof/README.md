# hako-alloc-segment-map-local-free-reuse-ledger-release-bridge-proof

Row: MIMAP-196A

This proof composes the segment-map local-free reuse ledger bridge into the
existing MIMAP-134A reuse-ledger release owner.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-196A
```

Stop lines: no real segment allocation/free, raw pointer residence, real
segment-map mutation, real free-list mutation, page-array mutation, arena
backing, atomics, OSVM/page-source, worker/concurrency, provider activation,
host allocator replacement, hooks, global allocator replacement, runtime sum
materialization, or backend app/owner matcher.
