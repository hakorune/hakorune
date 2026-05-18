# hako-alloc-segment-map-modeled-consume-ledger-release-proof

MIMAP-161A proof app for the segment-map modeled consume ledger release route.

It proves that a live token produced by the segment-map accepted-readiness
consume ledger can be released through the same owner boundary while reusing
the existing modeled ledger release substrate.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-161A
```

Stop lines:

- no real segment free execution
- no raw pointer residence
- no real segment-map mutation
- no arena backing allocation
- no atomic bitmap execution
- no OSVM/page-source execution
- no provider activation, hooks, or host allocator replacement
- no backend app/owner matcher
