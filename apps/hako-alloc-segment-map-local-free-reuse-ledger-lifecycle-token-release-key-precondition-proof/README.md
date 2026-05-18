# hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-proof

Row: MIMAP-220A

Purpose: prove the lifecycle-token release-key precondition observer after the
lifecycle-token observer diagnostic closeout.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-220A
```

This proof may report that a modeled reuse token is a future release-key
migration candidate. It keeps the actual release ledger keyed by modeled reuse
token and keeps release-ledger key migration, real lifecycle semantics, real
segment allocation/free, raw pointer residence, real segment-map mutation,
arena backing, atomic bitmap, OSVM/page-source execution, worker scheduling,
provider activation, hooks, `#[global_allocator]`, and backend matchers
inactive.
