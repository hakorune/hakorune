---
Status: Landed
Date: 2026-05-25
Scope: choose the post-long process-repeat follow-on after the abandoned-heap stress timing pack escaped the 1ms floor.
Related:
  - docs/development/current/main/phases/phase-295x/295x-234-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-LONG-PROCESS-REPEAT-TIMING-PACK.md
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
---

# 295x-235 Abandoned Heap Stress Post Long Process-Repeat Timing Selection

## Blocker

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-POST-LONG-PROCESS-REPEAT-TIMING-SELECTION-295X-002
```

## Decision

The long process-repeat timing evidence is useful, but it still includes
process/runtime startup and evidence-output costs.

Select:

```text
MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-PROCESS-REPEAT-PACK-295X-002
```

resume the actual `.hako` mimalloc port work using the current RSS + process-repeat evidence rather than treating this as allocator-body-only timing.

The next row should keep allocator-body timing and presentation-only
alternatives parked unless they are needed to keep the port seam narrow and
honest.

## Deferred

```text
allocator-body timing:
  stays parked until a body-clock seam is needed

presentation-only report:
  stays parked unless the process-repeat evidence needs a human-facing split

provider/DLL/replacement/hook/global allocator seams:
  remain closed
```

## Stop Line

This row does not compute speed winners, compute RSS winners, require timing
parity, add body-internal timers, change runtime behavior, make `empty` the
default runtime config, or open provider/DLL/replacement/hook/global allocator
seams.
