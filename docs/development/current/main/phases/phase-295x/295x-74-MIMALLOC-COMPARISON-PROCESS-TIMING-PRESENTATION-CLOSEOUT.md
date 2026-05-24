---
Status: Landed
Date: 2026-05-25
Scope: close process-repeat timing presentation and select allocator-body timing contract.
Related:
  - docs/development/current/main/phases/phase-295x/295x-73-MIMALLOC-COMPARISON-PROCESS-TIMING-PRESENTATION-PACK.md
  - tools/allocator/mimalloc_process_timing_presentation.py
---

# 295x-74 Process Timing Presentation Closeout

## Blocker

```text
MIMALLOC-COMPARISON-PROCESS-TIMING-PRESENTATION-CLOSEOUT-295X-001
```

## Closeout

The presentation-only process timing boundary is now explicit:

```text
timing_repeat_kind=process-invocation-v0
timing_claim_kind=process-repeat-presentation-only
allocator_body_timing=0
process_runtime_cost_included=1
evidence_output_cost_included=1
winner_claim=0
```

This is enough to prevent the long timing row from being promoted into a speed
claim. The next useful seam is not more process timing; it is a contract for
body-internal timing that keeps process timing and RSS evidence separate.

## Decision

Select:

```text
MIMALLOC-COMPARISON-ALLOCATOR-BODY-TIMING-CONTRACT-295X-001
```

The contract row should define body timing vocabulary before implementation.
It should start with C runner feasibility and keep `.hako` body timing behind a
separate follow-on selection if runtime/runtime-config behavior would need to
change.

## Stop Line

This row does not add body timers, compute speed winners, compute RSS winners,
require timing parity, change runtime behavior, make `empty` the default
runtime config, resume allocator semantics porting, or open
provider/DLL/replacement/hook/global allocator seams.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_process_timing_presentation_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
