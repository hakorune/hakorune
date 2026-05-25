---
Status: Landed
Date: 2026-05-25
Scope: abandoned-heap stress long process-repeat timing pack.
Related:
  - docs/development/current/main/phases/phase-295x/295x-233-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-HIGH-RES-TIMING-SEAM-SELECTION.md
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
---

# 295x-234 Abandoned Heap Stress Long Process-Repeat Timing Pack

## Blocker

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-LONG-PROCESS-REPEAT-TIMING-PACK-295X-002
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
sample_count=3
warmup_count=1
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

The row contract is fixed to:

```text
sample_count=3
warmup_count=1
hako_runtime_config_profile=empty
hako_selected_loadset=empty
winner_claim=0
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
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-POST-LONG-PROCESS-REPEAT-TIMING-SELECTION-295X-002
```

## Stop Line

This row does not compute speed winners, require timing parity, compute RSS
winners, add body-internal timers, change runtime behavior, make `empty` the
default, or open provider/DLL/replacement/hook/global allocator seams.
