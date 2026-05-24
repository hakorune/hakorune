---
Status: Landed
Date: 2026-05-25
Scope: add C-runner allocator-body timing for one workload before .hako timing.
Related:
  - docs/development/current/main/phases/phase-295x/295x-75-MIMALLOC-COMPARISON-ALLOCATOR-BODY-TIMING-CONTRACT.md
  - tools/allocator/c_mimalloc_explicit_runner.c
  - tools/allocator/c_mimalloc_explicit_runner.sh
---

# 295x-76 C Body Timing Pilot

## Blocker

```text
MIMALLOC-COMPARISON-C-BODY-TIMING-PILOT-295X-001
```

## Pilot

The explicit C mimalloc runner now emits a monotonic workload-body timing field
for the selected workload:

```text
c_body_timing_available=1
hako_body_timing_available=0
body_timing_repeat_kind=workload-body-monotonic-v0
body_timing_scope=allocator-workload-body
body_timing_is_process_timing=0
body_elapsed_ns=...
```

The timer is inside the C process and wraps only the selected workload dispatch.
It excludes `dlopen`, mimalloc symbol lookup, final RSS sampling, report
formatting, and shell-level `/usr/bin/time` measurement.

Process timing stays separate:

```text
timing_repeat_kind=process-invocation-v0
external_elapsed_ms=...
```

## Evidence Scope

The pilot uses `representative-small-block-v0` first because that workload has
stable allocation/free/requested-byte parity with the `.hako` evidence path.

The row proves only that C-side body timing can be emitted without changing the
comparison stop lines. It does not compare against `.hako` body timing yet.

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-HAKO-BODY-TIMING-FEASIBILITY-SELECTION-295X-001
```

That row should decide whether `.hako` body timing can be exposed without
changing runtime defaults or opening a broad app-visible clock surface.

## Stop Line

This row does not add `.hako` body timing, compute speed winners, compute RSS
winners, require timing parity, change runtime behavior, make `empty` the
default runtime config, resume allocator semantics porting, or open
provider/DLL/replacement/hook/global allocator seams.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_c_body_timing_pilot_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
