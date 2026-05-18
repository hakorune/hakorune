# hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-proof

Row: MIMAP-216A

Purpose: prove the lifecycle-token observer diagnostic after the lifecycle-token
pilot closeout.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-216A
```

This proof keeps release-ledger key migration, real lifecycle semantics, real
segment allocation/free, raw pointer residence, real segment-map mutation,
arena backing, atomic bitmap, OSVM/page-source execution, worker scheduling,
provider activation, hooks, `#[global_allocator]`, and backend matchers
inactive.
