# hako-alloc-segment-map-released-span-local-free-candidate-bridge-proof

Row: MIMAP-172A

This proof composes the segment-map modeled consume-ledger released-span
observation route into the existing modeled local-free candidate ledger.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-172A
```

Stop lines:

- no real segment allocation/free execution
- no free-list mutation
- no raw pointer residence
- no real segment-map mutation
- no arena backing
- no atomic bitmap execution
- no OSVM/page-source execution
- no provider activation, hooks, host allocator replacement, or global allocator
