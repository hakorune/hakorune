# mimalloc-facade-huge-unregister-proof

MIMAP-026A proof fixture for the facade huge-release page-map unregister route.

Scope:

- Allocate one huge request through the MIMAP-023A facade huge-page model route.
- Release that same live huge pointer through the existing M181
  `HakoAllocHugeReleaseSeam.releaseHugePtr(ptr)` seam.
- Prove the huge metadata live count transitions from live to released and the
  page-map entry transitions from live to unregistered.
- Forward non-huge requests through the existing MIMAP-021C small fallback.
- Expose scalar report fields for selected pointer identity, page id,
  requested/committed sizes, huge-model counters, page-map counters, M181 seam
  counters, and facade route counters.

Non-goals:

- No OS page release, unreserve, decommit, purge, or reclaim behavior.
- No small release/free, realloc, alignment, remote-free, TLS, atomic,
  provider hooks, host allocator replacement, or `#[global_allocator]`.
- No double-release / stale-pointer facade diagnostics beyond the existing M181
  success path.
- No backend matcher shortcut.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_facade_huge_unregister_exe_guard.sh
```
