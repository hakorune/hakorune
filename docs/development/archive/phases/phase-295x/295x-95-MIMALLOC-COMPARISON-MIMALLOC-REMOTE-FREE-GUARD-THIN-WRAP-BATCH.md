---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the remaining mimalloc remote-free guard roots into impl-backed wrappers.
Related:
  - tools/checks/k2_wide_mimalloc_remote_free_page_integration_guard.sh
  - tools/checks/k2_wide_mimalloc_remote_free_policy_exe_guard.sh
---

# 295x-95 Mimalloc Comparison Mimalloc Remote-Free Guard Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-REMOTE-FREE-GUARD-THIN-WRAP-BATCH-295X-001
```

## Decision

Thin-wrap the remaining mimalloc remote-free guard roots first. The selected
batch keeps the same guard paths and same check semantics, but moves the real
shell bodies into `tools/checks/impl/` so the root entrypoints stay stable and
small.

Selected roots:

- `k2_wide_mimalloc_remote_free_page_integration_guard.sh`
- `k2_wide_mimalloc_remote_free_policy_exe_guard.sh`

## Cleanup

- Keep each selected root as a thin wrapper that execs its impl body.
- Preserve all check semantics, artifact paths, and stop lines.
- Leave the current mimalloc comparison blocker unchanged.
- Keep the allocator-family inventories aligned with the root guard names.

## Result

The selected thick guards are now easier to scan at the root level, while the
real validation logic lives under `tools/checks/impl/`.

## Stop Line

This batch does not open provider activation, provider/DLL packaging, process
allocator replacement, hooks, `#[global_allocator]`, worker/TLS, atomics,
remote-free stress, abandoned-heap stress, or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_mimalloc_remote_free_page_integration_guard.sh
bash tools/checks/k2_wide_mimalloc_remote_free_policy_exe_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
