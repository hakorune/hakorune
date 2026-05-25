---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the M42 and M43 remote-free policy and retry-loop guard roots into impl-backed wrappers.
Related:
  - tools/checks/k2_wide_mimalloc_remote_free_list_policy_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_remote_free_retry_loop_exe_guard.sh
---

# 295x-143 M42 M43 Remote-Free Policy Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the M42 and M43 remote-free policy and retry-loop guard roots. The
batch keeps the same validation semantics and moves the real shell bodies into
`tools/checks/impl/`.

Selected roots:

- `k2_wide_mimalloc_remote_free_list_policy_exe_guard.sh`
- `k2_wide_mimalloc_remote_free_retry_loop_exe_guard.sh`

## Cleanup

- Keep the root scripts as thin wrappers that exec their impl bodies.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The two remote-free guard roots are now easier to scan at the root level.

## Stop Line

This batch does not open provider activation, provider/DLL packaging, process
allocator replacement, hooks, `#[global_allocator]`, worker/TLS, atomics,
remote-free stress, abandoned heap stress, or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_mimalloc_remote_free_list_policy_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_remote_free_retry_loop_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
