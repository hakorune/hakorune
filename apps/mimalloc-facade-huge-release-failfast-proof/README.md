# mimalloc-facade-huge-release-failfast-proof

MIMAP-025A proof fixture for the facade huge-release fail-fast diagnostics
route.

Scope:

- Allocate and release one huge pointer through the MIMAP-024A route.
- Reject a second release of the same huge pointer.
- Reject one stale/unknown huge pointer.
- Expose scalar report fields for live-count stability, release-reject
  counters, page-map counters, and final route status.

Non-goals:

- No adoption of the wider M181 `HakoAllocHugeReleaseSeam`.
- No page-map lookup or unregister.
- No OS page release, unreserve, decommit, purge, or reclaim behavior.
- No small release/free, realloc, alignment, remote-free, TLS, atomic,
  provider hooks, host allocator replacement, or `#[global_allocator]`.
- No backend matcher shortcut.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_facade_huge_release_failfast_exe_guard.sh
```
