# mimalloc-facade-huge-page-model-proof

MIMAP-023A proof fixture for the facade-owned huge-page model route.

Scope:

- Classify request size through the existing MIMAP-022B threshold.
- Route huge requests into the existing M180 `HakoAllocHugePageModel`.
- Forward non-huge requests through the existing MIMAP-022B / MIMAP-021C path.
- Expose scalar report fields for huge allocation, small forwarding, final
  result, and route counters.

Non-goals:

- No new huge page model owner.
- No huge release, unregister, unreserve, decommit, or OS release behavior.
- No page-map lookup route.
- No release, realloc, alignment, purge, reclaim, decommit, or recommit
  behavior changes.
- No provider hooks, host allocator replacement, or `#[global_allocator]`.
- No remote-free, TLS, atomic, or backend matcher shortcut.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_facade_huge_page_model_exe_guard.sh
```
