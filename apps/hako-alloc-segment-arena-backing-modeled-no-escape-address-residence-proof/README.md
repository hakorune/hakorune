# hako-alloc-segment-arena-backing-modeled-no-escape-address-residence-proof

Row: MIMAP-248A

This proof records an accepted no-escape address capability as a scalar/model
residence row. It keeps the address carrier non-dereferenceable and proves that
pointer lookup, real arena backing, segment-map mutation, atomics, OSVM,
workers, providers, and backend matchers remain inactive.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-248A
```
