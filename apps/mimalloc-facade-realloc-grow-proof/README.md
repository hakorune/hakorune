# mimalloc facade realloc grow proof

This proof belongs to `MIMAP-017B`.

It proves the facade-owned realloc grow/move route for one known live old block.
The route validates the old page/block, allocates a replacement block through
the existing facade small allocation path, and releases the old known block only
after replacement allocation succeeds. It does not copy bytes, use page-map
lookup, resolve arbitrary pointers, register/unregister ownership, use
OSVM/page-source behavior, activate provider hooks, or replace the host
allocator.
