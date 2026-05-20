# hako-alloc-provider-inactive-boundary-inventory-proof

Row: MIMAP-352A
Owner: `HakoAllocProviderInactiveBoundaryInventory`
Profile: `scalar-mir`

This proof records that provider activation, host allocator replacement,
hooks, `#[global_allocator]`, and backend owner-name matchers remain inactive
after the worker/TLS pilot.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-352A --level L2
```

Stop lines:

- no provider activation
- no host allocator replacement
- no hooks or `#[global_allocator]`
- no backend `.inc` matcher by app, box, owner, or row name
- no source-level worker-local or concurrency surface
- no cross-function `Result` direct ABI or runtime sum materialization
