# hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-proof

Row: MIMAP-212A

Purpose: prove the scalar lifecycle-token pilot after segment-map local-free
reuse ledger release-applied recycle second-release diagnostics.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-212A
```

This proof keeps release-ledger key migration, real segment allocation/free,
raw pointer residence, real segment-map mutation, arena backing, atomic bitmap,
OSVM/page-source execution, worker scheduling, provider activation, hooks,
`#[global_allocator]`, and backend matchers inactive.
