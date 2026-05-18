# hako-alloc-segment-map-local-free-reuse-ledger-release-apply-bridge-proof

Row: MIMAP-200A

This proof composes the segment-map local-free reuse ledger release bridge into
the existing MIMAP-138A source-ledger release apply route.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-200A
```

Stop lines: no real segment allocation/free, raw pointer residence, real
segment-map mutation, real free-list mutation, page-array mutation, arena
backing, atomics, OSVM/page-source, worker/concurrency, provider activation,
host allocator replacement, hooks, global allocator replacement, runtime sum
materialization, or backend app/owner matcher.
