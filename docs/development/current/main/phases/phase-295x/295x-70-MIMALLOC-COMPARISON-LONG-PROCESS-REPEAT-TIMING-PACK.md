---
Status: Landed
Date: 2026-05-25
Scope: phase-295x long process-repeat timing observation pack.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-69-MIMALLOC-COMPARISON-HIGH-RES-TIMING-SEAM-SELECTION.md
---

# 295x-70 Long Process-Repeat Timing Pack

## Blocker

```text
MIMALLOC-COMPARISON-LONG-PROCESS-REPEAT-TIMING-PACK-295X-001
```

## Implementation

The comparison runners now support:

```text
operation_repeat=N
timing_repeat_kind=process-invocation-v0
```

This row uses:

```text
operation_repeat=128
```

For `.hako`, the exact-MIR EXE is built once per sample and then invoked `N`
times inside the timed process-repeat loop. For C, the C runner binary is built
once per sample and then invoked `N` times inside the same style of timed loop.

The repeated measurement runner passes the same `operation_repeat` to both
sides and validates that:

```text
operation_repeat matches
timing_repeat_kind matches
operation_family matches
operation_sequence_id matches
free_order_id matches
winner_claim=0
```

## Observation

Command shape:

```text
python3 tools/allocator/mimalloc_repeated_measurement_runner.py \
  --sample-count 3 \
  --warmup-count 1 \
  --hako-runtime-config empty \
  --operation-repeat 128 \
  --allow-ldconfig-discovery
```

Median elapsed evidence:

```text
representative-small-block-v0:     hako=70ms  c=60ms
representative-realloc-aligned-v0: hako=70ms  c=60ms
representative-mixed-small-v0:     hako=70ms  c=60ms
representative-huge-ish-v0:        hako=70ms  c=80ms
```

This escapes the previous 1ms timing floor. The numbers are process-repeat
evidence, not allocator-body-only evidence.

## Follow-On

```text
MIMALLOC-COMPARISON-LONG-PROCESS-REPEAT-TIMING-CLOSEOUT-295X-001
```

## Stop Line

This row does not compute speed winners, require timing parity, compute RSS
winners, add body-internal timers, change runtime behavior, make `empty` the
default, or open provider/DLL/replacement/hook/global allocator seams.
