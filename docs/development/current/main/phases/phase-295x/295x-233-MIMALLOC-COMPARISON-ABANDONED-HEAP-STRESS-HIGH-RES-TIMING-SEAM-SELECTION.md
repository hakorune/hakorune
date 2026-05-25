---
Status: Current
Date: 2026-05-25
Scope: choose an abandoned-heap-specific high-resolution timing seam after the 1ms-floor observation pack.
Related:
  - docs/development/current/main/phases/phase-295x/295x-232-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SPEED-STABILITY-OBSERVATION-CLOSEOUT.md
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
---

# 295x-233 Abandoned Heap Stress High-Resolution Timing Seam Selection

## Blocker

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-HIGH-RES-TIMING-SEAM-SELECTION-295X-002
```

## Decision

Select an abandoned-heap-specific long process-repeat timing pack before
opening any body-internal timing contract:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-LONG-PROCESS-REPEAT-TIMING-PACK-295X-002
```

The previous elapsed-time observation proved that one process invocation per
sample is still too close to the floor for the abandoned-heap stress path.
This row keeps the same workload identities and only widens the timing seam
enough to expose elapsed signal without turning the report into a speed claim.

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

This is still not allocator-body-only timing. It measures repeated exact-EXE
process invocations after build, so it includes process/runtime startup and
evidence-output costs. That cost must stay explicit.

## Stop Line

This row does not compute speed winners, require timing parity, compute RSS
winners, change workload semantics, add body-internal timers, change runtime
behavior, make `empty` the default, or open provider/DLL/replacement/hook/global
allocator seams.

