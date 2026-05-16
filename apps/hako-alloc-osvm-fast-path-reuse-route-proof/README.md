# hako-alloc-osvm-fast-path-reuse-route-proof

Proof app for `MIMAP-043A`.

It demonstrates that `HakoAllocOsVmFastPathReuseRoute` composes the existing
OSVM-backed fast-path purge route and M205 recommit heap integration:

- allocate one small block through the OSVM-backed fast-path route;
- release the handle so the page becomes a purge candidate;
- run one bounded purge candidate through the M199/M212 seam;
- show allocation is rejected before recommit;
- recommit the selected page through M205;
- allocate from the same page after recommit.

This app is not a provider activation, host allocator replacement, hook, global
allocator, remote-free, TLS, atomic, worker scheduling, or user-facing
concurrency proof.
