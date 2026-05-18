# hako-alloc-segment-map-local-free-integration-bridge-proof

Row: MIMAP-184A

This proof composes the segment-map released-span chain into the existing
modeled local-free integration owner.

Validation profile: `scalar-mir`.

Representative EXE evidence is deferred to the future
`segment-map-local-free-integration-bridge` closeout pack.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-184A
```

Stop lines:

- no real segment allocation/free execution
- no real free-list mutation
- no raw pointer residence
- no real segment-map mutation
- no arena backing
- no atomic bitmap execution
- no OSVM/page-source execution
- no source-level concurrency, TLS, or worker-local behavior
- no provider activation, hooks, host allocator replacement, or global allocator
- no backend app/name matcher
