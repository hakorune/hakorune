---
Status: Landed
Date: 2026-05-25
Scope: decide whether to open .hako allocator-body timing after the C pilot.
Related:
  - docs/development/current/main/phases/phase-295x/295x-76-MIMALLOC-COMPARISON-C-BODY-TIMING-PILOT.md
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
---

# 295x-77 Hako Body Timing Feasibility Selection

## Blocker

```text
MIMALLOC-COMPARISON-HAKO-BODY-TIMING-FEASIBILITY-SELECTION-295X-001
```

## Finding

The C runner can emit allocator-body timing because it owns a local
`CLOCK_MONOTONIC` timer around the selected workload dispatch.

The `.hako` exact-EXE side does not currently expose a narrow app-visible
monotonic clock seam for workload-body timing. Adding one would be a language /
runtime surface decision, not a comparison-only formatter change.

## Decision

Do not open `.hako` body timing in this row.

Select a narrow port-resume seam instead:

```text
MIMALLOC-COMPARISON-PORT-RESUME-SEAM-SELECTION-295X-001
```

The comparison lane now has:

```text
RSS evidence:
  repeated external-time evidence and baseline attribution

process timing:
  process-invocation-v0, presentation-only

C body timing:
  workload-body-monotonic-v0, C side only

.hako body timing:
  parked until an explicit clock/runtime seam is designed
```

This is enough to continue `.hako` mimalloc porting without pretending that C
body timing and `.hako` process timing are equivalent.

## Future Clock Seam

If `.hako` body timing is reopened later, it should be a separate contract row
that defines:

```text
clock source:
  monotonic, non-wall-clock, benchmark-only or general language API

runtime behavior:
  no plugin loadset change
  no provider/DLL/replacement activation
  deterministic fail-fast on unsupported backend

evidence:
  hako_body_timing_available=1
  body_timing_repeat_kind=workload-body-monotonic-v0
```

## Stop Line

This row does not add `.hako` body timers, add a clock API, change runtime
behavior, compute speed winners, compute RSS winners, require timing parity,
resume broad `usize` migration, or open provider/DLL/replacement/hook/global
allocator seams.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_hako_body_timing_feasibility_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
