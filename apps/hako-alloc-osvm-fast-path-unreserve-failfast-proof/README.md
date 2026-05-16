# hako-alloc-osvm-fast-path-unreserve-failfast-proof

Proof app for `MIMAP-046A`.

It demonstrates that `HakoAllocOsVmFastPathUnreserveFailFastRoute` keeps the
MIMAP-045A success route narrow while adding scalar diagnostics for invalid
fast-path unreserve requests:

- first unreserve succeeds through the existing MIMAP-045A route;
- duplicate unreserve is rejected before a second adapter call;
- unknown page id is rejected before adapter execution;
- known but not-decommitted page is rejected before adapter execution.

This app is not a provider activation, host allocator replacement, hook, global
allocator, post-unreserve reuse, OS release, remote-free, TLS, atomic, worker
scheduling, reclaim execution, or user-facing concurrency proof.
