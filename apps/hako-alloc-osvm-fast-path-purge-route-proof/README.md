# hako-alloc-osvm-fast-path-purge-route-proof

Proof app for `MIMAP-042A`.

It demonstrates that `HakoAllocOsVmFastPathPurgeRoute` composes the existing
OSVM-backed fast-path heap, state-aware purge guard, and bounded scheduler:

- allocate one small block through the OSVM-backed fast-path heap;
- release the handle so the page becomes a purge candidate;
- run one bounded purge candidate through the M199/M212 seam;
- run a duplicate purge without executing source decommit again.

This app is not a provider activation, host allocator replacement, hook, global
allocator, remote-free, TLS, atomic, or user-facing concurrency proof.
