---
Status: Landed
Date: 2026-05-25
Scope: phase-295x abandoned-heap stress closeout on the comparison lane
Blocker: MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CLOSEOUT-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-217-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-PRESENTATION.md
  - tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_presentation_guard.sh
  - tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_closeout_guard.sh
---

# 295x-218 Abandoned Heap Stress Closeout

## Decision

Close:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CLOSEOUT-295X-002
```

The evidence pair is now stable as a presentation contract. This row closes the
abandoned-heap stress pack and parks the lane on a narrow baseline-breakdown
selection instead of widening provider or host-replacement seams.

## Closeout Summary

The stable presentation contract remains:

```text
output_contract=mimalloc-comparison-abandoned-heap-stress-presentation-v0
input_contract=mimalloc-comparison-abandoned-heap-stress-evidence-v0
presentation_only=1
```

The stable proof-pair fields remain unchanged:

```text
same=1,1,0
remote=2,1,1,0
abandoned=3,1,1,1,1
pending=0,6,4,3
counts=4,1,1,1,1
mailbox=0,0,0
shape=9
missing=0,1,10,0
active_owner=0,2,0,1
same_owner=0,2,2,2
remote_pending=0,3,3
decommitted=0,4,1
live=1,0,1,1,1,0
retired=1,0,1,1,1
would=0,0,0,0,0,0
counts=7,2,5,1,2,1,1,1,1,1,16,0
```

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-BASELINE-BREAKDOWN-SELECTION-295X-002
```

The next row should inspect whether the fixed evidence footprint is dominated by
the empty exact-EXE runtime path, the proof-pair presentation path, or some
other narrow baseline seam. It should not open provider/DLL, host replacement,
or winner-claim seams.

## Stop Line

This row does not broaden provider/DLL or host replacement seams, install
hooks, change default runtime behavior, compute winner claims, or make RSS
parity claims unless this card explicitly says so.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
