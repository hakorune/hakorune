# hako-alloc segment-map scalar lookup boundary inventory proof

This proof app fixes MIMAP-151A.

It proves one explicit-ID segment/page/slice lookup row and stable rejection
for unknown segment, wrong page, stale generation, out-of-range slice, and
raw-pointer lookup requests.

Stop line: this app must not open raw pointer residence, real segment-map
execution, arena backing, atomic bitmap execution, OSVM/page-source execution,
worker scheduling, provider activation, host allocator replacement, hooks, or
backend matchers.
