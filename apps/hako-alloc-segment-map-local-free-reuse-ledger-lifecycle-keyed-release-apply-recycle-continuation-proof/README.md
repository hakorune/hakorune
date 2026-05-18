# hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-apply-recycle-continuation-proof

Row: MIMAP-232A

Purpose: prove lifecycle-keyed source release apply/recycle continuation in
scalar/model space.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-232A
```

This proof applies a lifecycle-keyed source release row to the modeled reuse
ledger through the explicit `modeled_reuse_token` backref, then recycles the
local-free reuse row. It does not use the old modeled-reuse-token keyed release
owner as the continuation owner; the old owner appears only as isolated fixture
setup/precondition input. It does not open real segment allocation/free, raw
pointer residence, real segment-map mutation, arena backing, atomic bitmap
execution, OSVM/page-source execution, worker scheduling, provider activation,
hooks, `#[global_allocator]`, or backend matchers.
