---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the M167 mimalloc alloc fast path guard root into an impl-backed wrapper and keep the memory README owner note in sync.
Related:
  - tools/checks/k2_wide_mimalloc_alloc_fast_path_guard.sh
---

# 295x-178 M167 Mimalloc Alloc Fast Path Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the M167 mimalloc alloc fast path guard root. The validation
semantics stay the same while the real shell body moves into `tools/checks/impl/`.

Selected root:

- `k2_wide_mimalloc_alloc_fast_path_guard.sh`

## Cleanup

- Keep the root script as a thin wrapper that execs its impl body.
- Keep the alloc fast path owner note in the memory README aligned with the M167 seam.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The M167 alloc fast path guard is easier to scan at the root level.

## Stop Line

This batch does not open OSVM page sourcing, local-free collection/retire,
remote-free atomics, page-map lookup, provider activation, hook install,
process allocator replacement, `.inc` name matching, or production `usize`
field migration work.

## Validation

```bash
bash tools/checks/k2_wide_mimalloc_alloc_fast_path_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
