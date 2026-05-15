# mimalloc facade small allocation fallback proof

This proof belongs to `MIMAP-014B`.

It proves that the object lifecycle facade keeps one small allocation route that
prefers a reusable page, falls back to an active page after reusable candidates
are unavailable, and exposes a scalar miss reason when no page can satisfy the
request.
