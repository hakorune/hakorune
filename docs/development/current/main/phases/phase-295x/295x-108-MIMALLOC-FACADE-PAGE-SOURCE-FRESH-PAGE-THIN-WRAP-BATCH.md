---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the MIMAP-021B facade page-source fresh-page guard root into an impl-backed wrapper and keep the memory README owner note in sync.
Related:
  - tools/checks/k2_wide_mimalloc_facade_page_source_fresh_page_exe_guard.sh
  - tools/checks/impl/k2_wide_mimalloc_facade_page_source_fresh_page_exe_guard.sh
  - lang/src/hako_alloc/memory/README.md
---

# 295x-108 Mimalloc Facade Page-Source Fresh-Page Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the MIMAP-021B facade page-source fresh-page EXE guard. The batch
keeps the same validation semantics, but moves the real shell body into
`tools/checks/impl/` and documents the owner note in the memory-layer README.

Selected root:

- `k2_wide_mimalloc_facade_page_source_fresh_page_exe_guard.sh`

## Cleanup

- Keep the root script as a thin wrapper that execs its impl body.
- Keep the MIMAP-021B owner note visible in `lang/src/hako_alloc/memory/README.md`.
- Keep the fresh-page attach guard narrow.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The MIMAP-021B facade page-source fresh-page guard is now easier to scan at
the root level, and the memory README explicitly documents the owner note that
the guard expects.

## Stop Line

This batch does not open provider activation, provider/DLL packaging, process
allocator replacement, hooks, `#[global_allocator]`, worker/TLS, atomics,
remote-free stress, abandoned-heap stress, or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_mimalloc_facade_page_source_fresh_page_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
