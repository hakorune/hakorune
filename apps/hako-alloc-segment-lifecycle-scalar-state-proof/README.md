# hako-alloc-segment-lifecycle-scalar-state-proof

Proof app for `MIMAP-082A`.

It exercises the scalar segment lifecycle state contract and confirms that raw
pointer residence, atomic bitmap execution, OSVM execution, real thread
scheduling, provider activation, process allocator replacement, and backend
matchers stay inactive.

Run:

```bash
bash apps/hako-alloc-segment-lifecycle-scalar-state-proof/test.sh
```

