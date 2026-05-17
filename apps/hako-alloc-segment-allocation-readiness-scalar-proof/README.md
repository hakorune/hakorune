# hako-alloc-segment-allocation-readiness-scalar-proof

Proof app for `MIMAP-088A`.

It exercises the scalar segment allocation-readiness contract and confirms that
segment allocation/free execution, raw pointer residence, segment-map lookup,
arena backing allocation, atomic bitmap execution, OSVM execution, real thread
scheduling, provider activation, process allocator replacement, and backend
matchers stay inactive.

Run:

```bash
bash apps/hako-alloc-segment-allocation-readiness-scalar-proof/test.sh
```
