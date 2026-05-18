# hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-proof

Row: MIMAP-228A

Purpose: prove the controlled source release-ledger lifecycle-key migration
pilot in scalar/model space.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-228A
```

This proof uses a new lifecycle-keyed source release ledger owner. It preserves
the old modeled-reuse-token keyed release owner as an unmigrated reference and
keeps real segment allocation/free, raw pointer residence, real segment-map
mutation, arena backing, atomic bitmap execution, OSVM/page-source execution,
worker scheduling, provider activation, hooks, `#[global_allocator]`, and
backend matchers inactive.
