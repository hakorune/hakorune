---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the MIMAP-018A facade stats snapshot guard root into an impl-backed wrapper.
Related:
  - tools/checks/k2_wide_mimalloc_facade_stats_snapshot_exe_guard.sh
---

# 295x-150 MIMAP-018A Facade Stats Snapshot Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the MIMAP-018A facade stats snapshot guard root. The batch keeps
the same validation semantics and moves the real shell body into
`tools/checks/impl/`.

Selected root:

- `k2_wide_mimalloc_facade_stats_snapshot_exe_guard.sh`

## Cleanup

- Keep the root script as a thin wrapper that execs its impl body.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The MIMAP-018A facade stats snapshot guard root is now easier to scan at the
root level.

## Stop Line

This batch does not open policy mutation, provider activation, provider/DLL
packaging, process allocator replacement, hooks, `#[global_allocator]`,
worker/TLS wider substrate work, atomics, remote-free stress, abandoned heap
stress, or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_mimalloc_facade_stats_snapshot_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
