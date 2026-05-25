---
Status: Landed
Date: 2026-05-25
Scope: phase-295x speed/stability observation pack on the abandoned-heap stress path.
Blocker: MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SPEED-STABILITY-OBSERVATION-PACK-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-230-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-CLOSEOUT.md
  - tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_speed_stability_observation_pack_guard.sh
---

# 295x-231 Abandoned Heap Stress Speed / Stability Observation Pack

## Decision

Close:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SPEED-STABILITY-OBSERVATION-PACK-295X-002
```

Add external elapsed-time evidence to the existing repeated comparison pack.

The selected bench set remains:

```text
representative-small-block-v0
representative-realloc-aligned-v0
representative-mixed-small-v0
representative-huge-ish-v0
```

This is an observation pack. It records timing and exit stability without
declaring speed winners.

## Contract

Use:

```text
sample_count=5
warmup_count=1
hako_runtime_config_profile=empty
hako_selected_loadset=empty
winner_claim=0
```

The timing source is `/usr/bin/time` around the already-built executable
invocation. It excludes the `.hako` exact-MIR build step and the C runner compile
step.

## Evidence

The repeated runner emits per-workload elapsed timing fields for both sides:

```text
workload_<n>_hako_external_elapsed_min_ms
workload_<n>_hako_external_elapsed_median_ms
workload_<n>_hako_external_elapsed_max_ms
workload_<n>_c_external_elapsed_min_ms
workload_<n>_c_external_elapsed_median_ms
workload_<n>_c_external_elapsed_max_ms
```

On the observed sample, workload_0 rounded to `1ms / 1ms` for both sides.

## Follow-On

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SPEED-STABILITY-OBSERVATION-CLOSEOUT-295X-002
```

## Stop Line

This row does not compute speed winners, require time parity, compute RSS
winners, add new workload families, change runtime behavior, make `empty` the
default, or open provider/DLL/replacement/hook/global allocator seams.
