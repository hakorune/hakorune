---
Status: Current
Date: 2026-05-25
Scope: phase-295x native multi-worker substrate stress closeout on the comparison lane
Blocker: MIMALLOC-COMPARISON-PAR-STRESS-CLOSEOUT-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-212-MIMALLOC-COMPARISON-PAR-STRESS-PRESENTATION.md
  - tools/checks/k2_wide_phase295x_mimalloc_parallel_substrate_stress_closeout_guard.sh
---

# 295x-213 PAR Stress Closeout

## Decision

Close:

```text
MIMALLOC-COMPARISON-PAR-STRESS-CLOSEOUT-295X-002
```

The presentation row made the native multi-worker substrate stress contract
stable. The next useful seam is not more presentation; it is an
abandoned-heap stress selection that can probe reclaim behavior after the
worker/TLS/atomic/remote-free shape has already been fixed.

## Closeout Summary

The stable comparison contract remains:

```text
output_contract=mimalloc-comparison-par-stress-presentation-v0
input_contract=mimalloc-comparison-par-stress-evidence-v0
presentation_only=1
```

The stable counts stay unchanged:

```text
worker_count=4
iterations_per_worker=64
expected_remote_free_count=256
observed_remote_free_count=256
drained_nodes=256
payload_sum_nonzero=1
```

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SELECTION-295X-002
```

## Stop Line

This row does not broaden provider/DLL or host replacement seams, install
hooks, change default runtime behavior, compute winner claims, or make RSS
parity claims unless this card explicitly says so.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_mimalloc_parallel_substrate_stress_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
