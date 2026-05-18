# hako-alloc-segment-map-local-free-page-apply-bridge-proof

Row: MIMAP-180A

This proof composes the segment-map local-free apply-plan bridge into the
existing modeled local-free page-apply route.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-180A
```

Stop lines:

- no real segment allocation/free execution
- no real free-list mutation
- no raw pointer residence
- no real segment-map mutation
- no arena backing
- no atomic bitmap execution
- no OSVM/page-source execution
- no provider activation, hooks, host allocator replacement, or global allocator
