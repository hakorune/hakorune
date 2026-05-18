# hako-alloc-segment-map-local-free-apply-plan-bridge-proof

Row: MIMAP-176A

This proof composes the segment-map local-free candidate bridge into the
existing modeled local-free apply-plan ledger.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-176A
```

Stop lines:

- no real segment allocation/free execution
- no free-list mutation
- no page-state mutation
- no raw pointer residence
- no real segment-map mutation
- no arena backing
- no atomic bitmap execution
- no OSVM/page-source execution
- no provider activation, hooks, host allocator replacement, or global allocator
