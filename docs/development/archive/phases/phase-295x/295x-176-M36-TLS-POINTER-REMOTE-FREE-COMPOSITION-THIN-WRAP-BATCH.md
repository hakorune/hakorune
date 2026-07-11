---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the M36 TLS pointer remote-free composition guard root into an impl-backed wrapper and keep the memory README owner note in sync.
Related:
  - tools/checks/k2_wide_mimalloc_tls_ptr_remote_free_exe_guard.sh
---

# 295x-176 M36 TLS Pointer Remote-Free Composition Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the M36 TLS pointer remote-free composition guard root. The
validation semantics stay the same while the real shell body moves into
`tools/checks/impl/`.

Selected root:

- `k2_wide_mimalloc_tls_ptr_remote_free_exe_guard.sh`

## Cleanup

- Keep the root script as a thin wrapper that execs its impl body.
- Keep the remote-free owner note in the memory README aligned with the M36 seam.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The M36 TLS pointer remote-free composition guard is easier to scan at the root level.

## Stop Line

This batch does not open pointer `fetch_add`, production remote-free policy,
new MIR route rows, new NyRT exports, provider activation, or allocator
replacement work.

## Validation

```bash
bash tools/checks/k2_wide_mimalloc_tls_ptr_remote_free_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
