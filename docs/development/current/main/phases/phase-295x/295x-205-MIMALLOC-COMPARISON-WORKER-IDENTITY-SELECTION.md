---
Status: Landed
Date: 2026-05-25
Scope: phase-295x worker identity seam selection on the comparison lane
Blocker: MIMALLOC-COMPARISON-WORKER-IDENTITY-SELECTION-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-204-MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-EVIDENCE.md
  - tools/checks/k2_wide_mimalloc_worker_identity_exe_guard.sh
---

# 295x-205 Worker Identity Selection

## Decision

Close:

```text
MIMALLOC-COMPARISON-WORKER-IDENTITY-SELECTION-295X-002
```

Closed the smaller-default load-set evidence row and selected the allocator-internal worker identity substrate as the next narrow concurrency seam.

## Selected Row

Select:

```text
MIMAP-WORKER-001 internal worker identity substrate
```

## Stop Line

This row does not broaden provider/DLL or host replacement seams, install
hooks, change default runtime behavior, compute winner claims, or make RSS
parity claims unless this card explicitly says so.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_worker_identity_exe_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
