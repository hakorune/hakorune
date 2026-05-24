---
Status: Landed
Date: 2026-05-25
Scope: phase-295x high-resolution timing seam selection.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-68-MIMALLOC-COMPARISON-SPEED-STABILITY-OBSERVATION-CLOSEOUT.md
---

# 295x-69 High-Resolution Timing Seam Selection

## Blocker

```text
MIMALLOC-COMPARISON-HIGH-RES-TIMING-SEAM-SELECTION-295X-001
```

## Decision

Select a long process-repeat timing observation pack.

The previous elapsed-time pack proved that the selected workload bodies are too
short for `/usr/bin/time` millisecond-scale evidence when each sample runs one
process invocation. The next pack keeps the same workload identities and repeats
the already-built executable process enough times to escape the 1ms floor.

Selected follow-on:

```text
MIMALLOC-COMPARISON-LONG-PROCESS-REPEAT-TIMING-PACK-295X-001
```

## Contract

```text
timing_repeat_kind=process-invocation-v0
operation_repeat=128
sample_count=3
warmup_count=1
hako_runtime_config_profile=empty
hako_selected_loadset=empty
winner_claim=0
```

This is not an allocator-body-only timing seam. It measures repeated executable
process invocations after build, so it includes process/runtime startup and
evidence-output costs. That cost is explicit in `timing_repeat_kind`.

## Stop Line

This row does not compute speed winners, require timing parity, compute RSS
winners, change workload semantics, add body-internal timers, change runtime
behavior, make `empty` the default, or open provider/DLL/replacement/hook/global
allocator seams.
