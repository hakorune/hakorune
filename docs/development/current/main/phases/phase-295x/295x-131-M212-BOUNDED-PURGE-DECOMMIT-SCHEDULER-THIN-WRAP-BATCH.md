---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the M212 bounded purge/decommit scheduler guard root into an impl-backed wrapper and keep the memory README owner note in sync.
Related:
  - tools/checks/k2_wide_hako_alloc_bounded_purge_decommit_scheduler_guard.sh
  - tools/checks/impl/k2_wide_hako_alloc_bounded_purge_decommit_scheduler_guard.sh
  - lang/src/hako_alloc/memory/README.md
---

# 295x-131 M212 Bounded Purge/Decommit Scheduler Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the M212 bounded purge/decommit scheduler guard root. The batch
keeps the same validation semantics, moves the real shell body into
`tools/checks/impl/`, and keeps the memory owner note in sync.

Selected root:

- `k2_wide_hako_alloc_bounded_purge_decommit_scheduler_guard.sh`

## Cleanup

- Keep the root script as a thin wrapper that execs its impl body.
- Keep the M212 bounded purge/decommit scheduler owner visible in
  `lang/src/hako_alloc/memory/README.md`.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The M212 bounded purge/decommit scheduler guard is now easier to scan at the
root level, and the memory README explicitly documents the owner note that the
guard expects.

## Stop Line

This batch does not open provider activation, provider/DLL packaging, process
allocator replacement, hooks, `#[global_allocator]`, worker/TLS, atomics,
remote-free stress, abandoned heap stress, or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_bounded_purge_decommit_scheduler_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
