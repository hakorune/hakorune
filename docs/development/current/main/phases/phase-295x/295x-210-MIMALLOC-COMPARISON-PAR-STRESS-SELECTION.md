---
Status: Current
Date: 2026-05-25
Scope: phase-295x native multi-worker substrate stress selection on the comparison lane
Blocker: MIMALLOC-COMPARISON-PAR-STRESS-SELECTION-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-209-MIMALLOC-COMPARISON-THREADSAFE-ABI-SELECTION.md
  - tools/checks/k2_wide_mimalloc_parallel_substrate_stress_guard.sh
---

# 295x-210 PAR Stress Selection

## Decision

Close:

```text
MIMALLOC-COMPARISON-PAR-STRESS-SELECTION-295X-002
```

Closed the thread-safe `hako_mem` ABI selection row and selected the native multi-worker substrate stress as the next narrow concurrency seam.

## Selected Row

Select:

```text
MIMAP-PAR-STRESS-001 native multi-worker substrate stress for per-worker heaps and remote-free pressure
```

## Stop Line

This row does not broaden provider/DLL or host replacement seams, install
hooks, change default runtime behavior, compute winner claims, or make RSS
parity claims unless this card explicitly says so.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_parallel_substrate_stress_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
