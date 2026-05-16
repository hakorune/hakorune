# mimalloc-facade-huge-page-source-proof

MIMAP-028A proof fixture for the facade huge page-source backing route.

Scope:

- Classify one huge request through the existing huge threshold.
- Reserve and commit one page-source backing range.
- Route the same huge request into the existing MIMAP-023A huge page model
  route.
- Expose scalar report fields for backing identity, huge metadata, page-map
  registration, and inactive release/unregister/decommit counters.

Non-goals:

- No huge release, unregister, unreserve, decommit, or OS release behavior.
- No small release/free, realloc, alignment, purge, or reclaim behavior.
- No provider hooks, host allocator replacement, or `#[global_allocator]`.
- No remote-free, TLS, atomic, or backend matcher shortcut.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_facade_huge_page_source_exe_guard.sh
```
