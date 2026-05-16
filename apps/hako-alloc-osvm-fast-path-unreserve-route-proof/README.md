# hako-alloc-osvm-fast-path-unreserve-route-proof

Proof app for `MIMAP-045A`.

It demonstrates that `HakoAllocOsVmFastPathUnreserveRoute` composes the existing
OSVM-backed fast-path route and the MIMAP-033A page-source unreserve adapter:

- allocate one small block through the OSVM-backed fast-path route;
- release the handle so the page becomes a purge candidate;
- run one bounded purge candidate through the M199/M212 seam;
- observe the decommit marker and backing range;
- unreserve that backing range through `HakoAllocPageSourceUnreserveAdapter`.

This app is not a provider activation, host allocator replacement, hook, global
allocator, post-unreserve reuse, remote-free, TLS, atomic, worker scheduling, or
user-facing concurrency proof.
