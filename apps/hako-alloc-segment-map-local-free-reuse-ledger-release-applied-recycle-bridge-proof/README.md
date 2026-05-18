# hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-bridge-proof

Row: MIMAP-204A

This proof composes the segment-map local-free reuse ledger release apply bridge
into the existing source-ledger release-applied recycle route.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-204A
```

Stop lines: no real segment allocation/free, raw pointer residence, real
segment-map mutation, real free-list mutation, page-array mutation, arena
backing, atomics, OSVM/page-source, worker/concurrency, provider activation,
host allocator replacement, hooks, global allocator replacement, runtime sum
materialization, or backend app/owner matcher.
