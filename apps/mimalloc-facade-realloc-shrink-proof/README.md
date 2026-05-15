# mimalloc facade realloc shrink proof

This proof belongs to `MIMAP-017A`.

It proves the facade-owned same-page realloc shrink/no-move route for one known
live `(page id, block id)` over the existing object lifecycle queue. It does
not grow, move, copy bytes, register or unregister page-map ownership, perform
arbitrary pointer lookup, use OSVM/page-source behavior, activate provider
hooks, or replace the host allocator.
