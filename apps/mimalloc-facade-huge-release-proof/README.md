# mimalloc-facade-huge-release-proof

MIMAP-024A proof fixture for the facade huge-release metadata route.

Scope:

- Allocate one huge request through the MIMAP-023A facade huge-page model route.
- Release that same live huge pointer through
  `HakoAllocHugePageModel.markReleased(ptr)`.
- Forward non-huge requests through the existing MIMAP-021C small fallback.
- Expose scalar report fields for the selected pointer, page id,
  requested/committed sizes, live-count transition, model release counters, and
  facade route counters.

Non-goals:

- No adoption of the wider M181 `HakoAllocHugeReleaseSeam` facade route.
- No page-map lookup or unregister.
- No OSVM release, unreserve, decommit, purge, or reclaim behavior.
- No small release/free, realloc, alignment, remote-free, TLS, atomic,
  provider hooks, host allocator replacement, or `#[global_allocator]`.
- No double-release / stale-pointer facade fail-fast route.
- No backend matcher shortcut.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_facade_huge_release_exe_guard.sh
```
