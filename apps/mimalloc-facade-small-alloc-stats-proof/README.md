# mimalloc facade small allocation stats proof

This proof belongs to `MIMAP-014C`.

It proves facade-local read-only counters for the small allocation fast-path:
attempts, successes, failures, reusable-page successes, and active-page
successes.
