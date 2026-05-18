# hako-alloc segment-map lookup guarded readiness composition proof

This proof app fixes MIMAP-153A.

It composes the MIMAP-151A explicit-ID scalar lookup boundary with the existing
segment/page membership and allocation-readiness scalar owners. It proves one
accepted lookup -> membership -> readiness path plus stable lookup,
membership, readiness, and raw-pointer request rejection paths.

Stop line: this app must not open raw pointer residence, real segment-map
execution, arena backing, atomic bitmap execution, OSVM/page-source execution,
worker scheduling, provider activation, host allocator replacement, hooks, or
backend matchers.
