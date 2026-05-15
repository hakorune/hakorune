# mimalloc facade aligned allocation proof

This proof belongs to `MIMAP-016B`.

It proves the facade-owned aligned small-allocation route over the existing
object lifecycle queue. Supported alignment metadata is normalized before the
request reuses the existing small allocation path. Unsupported alignment fails
fast at the facade and does not call page-map lookup, pointer arithmetic,
realloc behavior, OSVM/page-source behavior, provider hooks, or host allocator
replacement.
