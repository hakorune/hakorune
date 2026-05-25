---
Status: Current
Date: 2026-05-25
Scope: add process-repeat evidence to the reuse-cycle small port seam.
Related:
  - docs/development/current/main/phases/phase-295x/295x-235-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-POST-LONG-PROCESS-REPEAT-TIMING-SELECTION.md
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
---

# 295x-236 Reuse-Cycle Small Process-Repeat Pack

## Blocker

```text
MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-PROCESS-REPEAT-PACK-295X-002
```

## Decision

Resume `.hako` mimalloc porting through the existing reuse-cycle small-block
workload with process-repeat evidence.

```text
workload=representative-reuse-cycle-small-v0
operation_family=reuse-cycle-small
operation_sequence_id=representative-reuse-cycle-small-v0-seq
free_order_id=even-odd-release-then-reacquire-v0
operation_repeat=128
timing_repeat_kind=process-invocation-v0
sample_count=3
warmup_count=1
winner_claim=0
```

This row reuses the narrow `.hako` port seam that already mirrors the
`representative-reuse-cycle-small-v0` request shape. The new work is not a new
allocator capability; it is the process-repeat evidence layer that keeps the
port seam visible without opening body-timing or provider seams.

Median repeated evidence:

```text
representative-reuse-cycle-small-v0: hako=80ms c=60ms
representative-reuse-cycle-small-v0: hako_rss_median=3526656 c_rss_median=3985408
```

## Stop Line

This row does not compute speed winners, compute RSS winners, require timing
parity, add body-internal timers, change runtime behavior, make `empty` the
default runtime config, or open provider/DLL/replacement/hook/global allocator
seams.
