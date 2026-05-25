---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the M208 heap reuse priority policy guard root into an impl-backed wrapper.
Related:
  - tools/checks/k2_wide_hako_alloc_heap_reuse_priority_policy_guard.sh
---

# 295x-147 M208 Heap Reuse Priority Policy Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the M208 heap reuse priority policy guard root. The batch keeps the
same validation semantics and moves the real shell body into
`tools/checks/impl/`.

Selected root:

- `k2_wide_hako_alloc_heap_reuse_priority_policy_guard.sh`

## Cleanup

- Keep the root script as a thin wrapper that execs its impl body.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The M208 heap reuse priority policy guard root is now easier to scan at the
root level.

## Stop Line

This batch does not open provider activation, provider/DLL packaging, process
allocator replacement, hooks, `#[global_allocator]`, worker/TLS wider substrate
work, atomics, remote-free stress, abandoned heap stress, or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_heap_reuse_priority_policy_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
