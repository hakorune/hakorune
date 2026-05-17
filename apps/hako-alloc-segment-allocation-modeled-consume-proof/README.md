# hako-alloc-segment-allocation-modeled-consume-proof

Proof app for `MIMAP-091A`.

It exercises the modeled scalar segment allocation consume route and confirms
that real segment allocation/free execution, raw pointer residence, segment-map
lookup, arena backing allocation, atomic bitmap execution, OSVM execution, real
thread scheduling, provider activation, process allocator replacement, and
backend matchers stay inactive.

Run:

```bash
bash apps/hako-alloc-segment-allocation-modeled-consume-proof/test.sh
```
