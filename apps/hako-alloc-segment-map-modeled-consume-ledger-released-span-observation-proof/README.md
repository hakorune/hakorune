# hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-proof

MIMAP-168A proof app for segment-map consume-ledger released-span observation.

It proves that a successful release report from the segment-map modeled
consume-ledger owner boundary exposes enough scalar span facts to be recorded by
the existing released-span ledger.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-168A
```

Stop lines:

- no real segment allocation/free execution
- no raw pointer residence
- no real segment-map mutation
- no arena backing allocation
- no atomic bitmap execution
- no OSVM/page-source execution
- no provider activation, hooks, or host allocator replacement
- no backend app/owner matcher
