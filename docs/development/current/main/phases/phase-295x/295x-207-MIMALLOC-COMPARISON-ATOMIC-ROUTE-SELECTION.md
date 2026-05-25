---
Status: Current
Date: 2026-05-25
Scope: phase-295x allocator atomic seam selection on the comparison lane
Blocker: MIMALLOC-COMPARISON-ATOMIC-ROUTE-SELECTION-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-206-MIMALLOC-COMPARISON-WORKER-TLS-SELECTION.md
  - tools/checks/k2_wide_mimalloc_allocator_atomic_route_guard.sh
---

# 295x-207 Atomic Route Selection

## Decision

Close:

```text
MIMALLOC-COMPARISON-ATOMIC-ROUTE-SELECTION-295X-002
```

Closed the worker TLS cache-slot selection row and selected the allocator-facing atomic route set as the next narrow concurrency seam.

## Selected Row

Select:

```text
MIMAP-ATOMIC-001 allocator-facing atomic load/store/CAS/fetch_add route guard
```

## Stop Line

This row does not broaden provider/DLL or host replacement seams, install
hooks, change default runtime behavior, compute winner claims, or make RSS
parity claims unless this card explicitly says so.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_allocator_atomic_route_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
