# mimalloc-facade-page-source-alloc-miss-proof

MIMAP-021C proof fixture for the facade-owned allocation-miss page-source
fallback.

Scope:

- Attempt one facade small allocation.
- If it fails with `small_no_page`, attach exactly one fresh page through the
  MIMAP-021B page-source adapter.
- Retry one small allocation and expose scalar proof fields for source and
  retry outcomes.

Non-goals:

- No release, realloc, alignment, purge, reclaim, decommit, or recommit
  behavior changes.
- No provider hooks, host allocator replacement, or `#[global_allocator]`.
- No remote-free, TLS, atomic, page-map lookup, unreserve, OS release, or
  backend matcher shortcut.
- No loop over multiple fresh pages.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_facade_page_source_alloc_miss_exe_guard.sh
```
