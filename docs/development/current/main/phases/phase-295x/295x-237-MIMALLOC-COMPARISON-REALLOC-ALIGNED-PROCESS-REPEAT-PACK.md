---
Status: Current
Date: 2026-05-25
Scope: add process-repeat evidence to the realloc/aligned port seam.
Related:
  - docs/development/current/main/phases/phase-295x/295x-236-MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-PROCESS-REPEAT-PACK.md
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
---

# 295x-237 Realloc/Aligned Process-Repeat Pack

## Blocker

```text
MIMALLOC-COMPARISON-REALLOC-ALIGNED-PROCESS-REPEAT-PACK-295X-002
```

## Decision

Resume `.hako` mimalloc porting through the existing realloc/aligned workload
with process-repeat evidence.

```text
workload=representative-realloc-aligned-v0
operation_family=realloc-aligned
operation_sequence_id=representative-realloc-aligned-v0-seq
free_order_id=ascending-release-v0
operation_repeat=128
timing_repeat_kind=process-invocation-v0
sample_count=3
warmup_count=1
winner_claim=0
```

This row reuses the narrow `.hako` port seam that already mirrors the
`representative-realloc-aligned-v0` request shape. The new work is not a new
allocator capability; it is the process-repeat evidence layer that keeps the
port seam visible without opening body-timing or provider seams.

Median repeated evidence:

```text
representative-realloc-aligned-v0: hako=70ms c=60ms
representative-realloc-aligned-v0: hako_rss_median=3592192 c_rss_median=3985408
```

## Stop Line

This row does not compute speed winners, compute RSS winners, require timing
parity, add body-internal timers, change runtime behavior, make `empty` the
default runtime config, or open provider/DLL/replacement/hook/global allocator
seams.
