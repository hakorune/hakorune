---
Status: Current
Date: 2026-05-25
Scope: phase-295x native multi-worker substrate stress presentation on the comparison lane
Blocker: MIMALLOC-COMPARISON-PAR-STRESS-PRESENTATION-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-211-MIMALLOC-COMPARISON-PAR-STRESS-EVIDENCE.md
  - tools/checks/k2_wide_phase295x_mimalloc_parallel_substrate_stress_presentation_guard.sh
  - tools/allocator/mimalloc_parallel_substrate_stress_presentation.py
---

# 295x-212 PAR Stress Presentation

## Decision

Close:

```text
MIMALLOC-COMPARISON-PAR-STRESS-PRESENTATION-295X-002
```

Present the native multi-worker substrate stress evidence as a stable comparison
contract so the worker/TLS/atomic/remote-free seam can be read in one place.

## Presentation

The stress evidence is normalized into a presentation contract:

```text
output_contract=mimalloc-comparison-par-stress-presentation-v0
input_contract=mimalloc-comparison-par-stress-evidence-v0
presentation_only=1
```

Stable counts remain unchanged:

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
MIMALLOC-COMPARISON-PAR-STRESS-CLOSEOUT-295X-002
```

## Stop Line

This row does not broaden provider/DLL or host replacement seams, install
hooks, change default runtime behavior, compute winner claims, or make RSS
parity claims unless this card explicitly says so.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_mimalloc_parallel_substrate_stress_presentation_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
