---
Status: Landed
Date: 2026-05-25
Scope: phase-295x allocator TLS seam selection on the comparison lane
Blocker: MIMALLOC-COMPARISON-WORKER-TLS-SELECTION-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-205-MIMALLOC-COMPARISON-WORKER-IDENTITY-SELECTION.md
  - tools/checks/k2_wide_mimalloc_worker_tls_cache_exe_guard.sh
---

# 295x-206 Worker TLS Cache Selection

## Decision

Close:

```text
MIMALLOC-COMPARISON-WORKER-TLS-SELECTION-295X-002
```

Closed the worker identity selection row and selected the allocator worker TLS cache-slot substrate as the next narrow concurrency seam.

## Selected Row

Select:

```text
MIMAP-TLS-001 allocator-local TLS / worker-local cache-slot substrate
```

## Stop Line

This row does not broaden provider/DLL or host replacement seams, install
hooks, change default runtime behavior, compute winner claims, or make RSS
parity claims unless this card explicitly says so.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_worker_tls_cache_exe_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
