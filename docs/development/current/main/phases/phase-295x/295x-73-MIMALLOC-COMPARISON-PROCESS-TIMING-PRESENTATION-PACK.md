---
Status: Landed
Date: 2026-05-25
Scope: add presentation-only process-repeat timing report without allocator-body timing claims.
Related:
  - docs/development/current/main/phases/phase-295x/295x-72-MIMALLOC-COMPARISON-POST-LONG-TIMING-SELECTION.md
  - tools/allocator/mimalloc_repeated_measurement_runner.py
  - tools/allocator/mimalloc_process_timing_presentation.py
---

# 295x-73 Process Timing Presentation Pack

## Blocker

```text
MIMALLOC-COMPARISON-PROCESS-TIMING-PRESENTATION-PACK-295X-001
```

## Contract

`tools/allocator/mimalloc_process_timing_presentation.py` formats a repeated
measurement report into a presentation-only timing report:

```text
output_contract=mimalloc-comparison-process-timing-presentation-v0
input_contract=mimalloc-comparison-repeated-measurement-v0
timing_repeat_kind=process-invocation-v0
timing_claim_kind=process-repeat-presentation-only
allocator_body_timing=0
process_runtime_cost_included=1
evidence_output_cost_included=1
winner_claim=0
```

The report preserves per-workload RSS median evidence and process-repeat
elapsed medians, but it deliberately marks every workload as:

```text
workload_N_allocator_body_timing=0
workload_N_winner_claim=0
```

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-PROCESS-TIMING-PRESENTATION-CLOSEOUT-295X-001
```

The closeout should keep this presentation as a human-facing boundary and then
choose whether to open allocator-body timing contract work or return to a
narrow `.hako` port seam.

## Stop Line

This row does not compute speed winners, compute RSS winners, require timing
parity, add allocator-body timers, change runtime behavior, make `empty` the
default runtime config, or open provider/DLL/replacement/hook/global allocator
seams.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_process_timing_presentation_pack_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
