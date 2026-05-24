---
Status: Landed
Date: 2026-05-25
Scope: choose the post-long-timing follow-on after process-repeat timing escaped the 1ms floor.
Related:
  - docs/development/current/main/phases/phase-295x/295x-71-MIMALLOC-COMPARISON-LONG-PROCESS-REPEAT-TIMING-CLOSEOUT.md
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
---

# 295x-72 Post Long-Timing Selection

## Blocker

```text
MIMALLOC-COMPARISON-POST-LONG-TIMING-SELECTION-295X-001
```

## Decision

Select presentation-only process timing before resuming `.hako` mimalloc port
work or opening allocator-body timing:

```text
MIMALLOC-COMPARISON-PROCESS-TIMING-PRESENTATION-PACK-295X-001
```

The long timing evidence is useful, but its repeat kind is:

```text
timing_repeat_kind=process-invocation-v0
```

That means the elapsed values include exact-EXE process/runtime startup,
selected loadset behavior, and evidence-output cost. The next row should make
that boundary visible in a small presentation report instead of letting readers
mistake process-repeat timing for allocator-body timing.

## Deferred

```text
allocator-body timing:
  next after presentation closeout, starting with a contract row

.hako mimalloc porting:
  resume after timing presentation/body-timing selection no longer clouds the
  comparison surface
```

## Stop Line

This row does not compute speed winners, compute RSS winners, require timing
parity, add allocator-body timers, resume allocator semantics porting, change
runtime defaults, or open provider/DLL/replacement/hook/global allocator seams.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_post_long_timing_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
