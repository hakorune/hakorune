# mimalloc page queue lifecycle selection proof

Decision: accepted for `MIMAP-010`.

This app proves the scalar lifecycle-aware page queue selection policy. It keeps decommitted pages skipped, selects reusable retired pages, then falls through to ordinary active pages.

The proof is queue-local and does not call OSVM, segment ownership, provider
activation, allocator hooks, or host allocator replacement.
