# mimalloc-facade-huge-failfast-proof

MIMAP-022B proof fixture for the facade-owned huge-request fail-fast boundary
in front of the MIMAP-021C allocation-miss page-source fallback.

Scope:

- Classify request size through `SizeClassBox`.
- Reject huge requests before page-source attach/retry is invoked.
- Forward non-huge requests to the existing MIMAP-021C alloc-miss fallback.
- Expose scalar report fields for huge rejection, small forwarding, fallback
  attempt, final result, and route counters.

Non-goals:

- No huge page model or page-map lookup.
- No release, realloc, alignment, purge, reclaim, decommit, or recommit
  behavior changes.
- No provider hooks, host allocator replacement, or `#[global_allocator]`.
- No remote-free, TLS, atomic, or backend matcher shortcut.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_facade_huge_failfast_exe_guard.sh
```
