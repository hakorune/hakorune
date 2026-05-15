# mimalloc facade alignment metadata proof

This proof belongs to `MIMAP-016A`.

It proves facade-local alignment request metadata observers over the existing
object lifecycle facade. The proof normalizes one supported request and records
one unsupported request as scalar metadata only; it does not execute aligned
allocation placement, page-map lookup, realloc behavior, OSVM/page-source
behavior, provider hooks, or host allocator replacement.
