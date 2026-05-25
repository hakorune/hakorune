---
Status: Current
Date: 2026-05-25
Scope: phase-295x abandoned-heap stress seam selection on the comparison lane
Blocker: MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SELECTION-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-213-MIMALLOC-COMPARISON-PAR-STRESS-CLOSEOUT.md
  - tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_selection_guard.sh
---

# 295x-214 Abandoned Heap Stress Selection

## Decision

Close:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SELECTION-295X-002
```

The PAR-STRESS closeout fixed the worker/TLS/atomic/remote-free substrate
shape. The next useful seam is an abandoned-heap stress contract refresh that
can probe reclaim-adjacent behavior without widening provider, hook, or host
replacement seams.

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CONTRACT-REFRESH-295X-002
```

## Stop Line

This row does not broaden provider/DLL or host replacement seams, install
hooks, change default runtime behavior, compute winner claims, or make RSS
parity claims unless this card explicitly says so.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
