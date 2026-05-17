# hako-alloc-segment-page-membership-scalar-proof

Proof app for `MIMAP-085A`.

It exercises the scalar segment/page membership contract and confirms that raw
pointer residence, segment-map pointer lookup, arena backing allocation, atomic
bitmap execution, OSVM execution, real thread scheduling, provider activation,
process allocator replacement, and backend matchers stay inactive.

Run:

```bash
bash apps/hako-alloc-segment-page-membership-scalar-proof/test.sh
```

