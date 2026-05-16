# mimalloc-facade-huge-unregister-failfast-proof

MIMAP-027A proof fixture for the facade huge-unregister fail-fast diagnostics
route.

Scope:

- Allocate and unregister one huge pointer through the MIMAP-026A route.
- Reject a second release of the same unregistered pointer through the existing
  M181 `HakoAllocHugeReleaseSeam`.
- Reject one stale/unknown huge pointer through the same M181 seam.
- Expose scalar report fields for page-map live-count stability, lookup-miss
  counters, M181 reject counters, and final route status.

Non-goals:

- No OS page release, unreserve, decommit, purge, or reclaim behavior.
- No small release/free, realloc, alignment, remote-free, TLS, atomic,
  provider hooks, host allocator replacement, or `#[global_allocator]`.
- No direct facade-side page-map lookup/unregister or direct huge-model
  metadata release calls.
- No backend matcher shortcut.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_facade_huge_unregister_failfast_exe_guard.sh
```
