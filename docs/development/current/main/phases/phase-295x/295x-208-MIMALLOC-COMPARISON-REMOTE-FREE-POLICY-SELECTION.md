---
Status: Landed
Date: 2026-05-25
Scope: phase-295x allocator remote-free policy seam selection on the comparison lane
Blocker: MIMALLOC-COMPARISON-REMOTE-FREE-POLICY-SELECTION-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-207-MIMALLOC-COMPARISON-ATOMIC-ROUTE-SELECTION.md
  - tools/checks/k2_wide_mimalloc_remote_free_policy_exe_guard.sh
---

# 295x-208 Remote-Free Policy Selection

## Decision

Close:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-POLICY-SELECTION-295X-002
```

Closed the allocator-facing atomic route selection row and selected the production-facade remote-free policy integration as the next narrow concurrency seam.

## Selected Row

Select:

```text
MIMAP-REMOTE-001 production-facade remote-free policy integration over existing atomic/TLS proofs
```

## Stop Line

This row does not broaden provider/DLL or host replacement seams, install
hooks, change default runtime behavior, compute winner claims, or make RSS
parity claims unless this card explicitly says so.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_remote_free_policy_exe_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
