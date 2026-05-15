# mimalloc-facade-page-source-fresh-page-proof

MIMAP-021B proof fixture for attaching one page-source-backed modeled page to
the object-lifecycle facade.

Scope:

- Reserve and commit one page through `HakoAllocPageSourcePolicy`.
- Construct one `HakoAllocPageModel`.
- Attach that page through `HakoAllocObjectLifecycleFacade.objectLifecycleAddPage`.
- Report scalar proof fields for source calls and facade queue state.

Non-goals:

- No allocation-on-miss retry.
- No release, realloc, alignment, purge, reclaim, decommit, or recommit
  behavior changes.
- No provider hooks, host allocator replacement, or `#[global_allocator]`.
- No remote-free, TLS, atomic, page-map lookup, unreserve, OS release, or
  backend matcher shortcut.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_facade_page_source_fresh_page_exe_guard.sh
```
