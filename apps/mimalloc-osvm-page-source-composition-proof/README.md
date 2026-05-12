# mimalloc-osvm-page-source-composition-proof

M168 proof fixture for the `.hako` mimalloc port.

This app composes the M167 page queue + page-local page model with the existing
`HakoAllocPageSourcePolicy` reserve/commit/decommit seam. Fresh modeled pages
are backed by OSVM page-source rows, then decommitted through the same policy
surface.

Scope:

- OSVM reserve/commit for fresh modeled page creation.
- Fresh page creation still registers the page through the queue/page model.
- Decommit is used only as page-source cleanup evidence.

Non-goals:

- No new native OSVM leaf.
- No OSVM unreserve/release row.
- No local-free collection / retire policy.
- No TLS, atomic, remote-free, page-map, provider, hook, or process allocator
  replacement.
- No production `usize` field migration.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_osvm_page_source_composition_guard.sh
```
