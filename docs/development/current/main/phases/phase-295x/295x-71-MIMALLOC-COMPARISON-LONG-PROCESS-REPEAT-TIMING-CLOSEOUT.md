---
Status: Landed
Date: 2026-05-25
Scope: phase-295x long process-repeat timing closeout.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-70-MIMALLOC-COMPARISON-LONG-PROCESS-REPEAT-TIMING-PACK.md
---

# 295x-71 Long Process-Repeat Timing Closeout

## Blocker

```text
MIMALLOC-COMPARISON-LONG-PROCESS-REPEAT-TIMING-CLOSEOUT-295X-001
```

## Closeout

The long process-repeat timing pack is stable enough to use as a timing
observation seam:

```text
operation_repeat=128
timing_repeat_kind=process-invocation-v0
sample_count=3
warmup_count=1
hako_selected_loadset=empty
winner_claim=0
```

The pack fixed the previous timing-floor problem. It still intentionally
includes process/runtime startup and evidence-output costs, so it must not be
presented as allocator-body-only timing.

## Decision

Close this pack and select a post-long-timing decision row:

```text
MIMALLOC-COMPARISON-POST-LONG-TIMING-SELECTION-295X-001
```

The next row should choose one of:

```text
- resume .hako mimalloc port work using current RSS + process-repeat evidence;
- add a body-internal timing seam for allocator-body-only timing;
- prepare a presentation-only report that clearly separates RSS, process-repeat
  timing, and still-parked winner claims.
```

## Stop Line

This row does not compute speed winners, require timing parity, compute RSS
winners, add body-internal timers, change runtime behavior, make `empty` the
default, or open provider/DLL/replacement/hook/global allocator seams.
