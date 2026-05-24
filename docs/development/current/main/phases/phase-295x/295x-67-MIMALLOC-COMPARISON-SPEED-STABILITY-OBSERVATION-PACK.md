---
Status: Landed
Date: 2026-05-25
Scope: phase-295x speed/stability observation pack.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-66-MIMALLOC-COMPARISON-POST-LOADSET-AWARE-MEASUREMENT-SELECTION.md
---

# 295x-67 Speed / Stability Observation Pack

## Blocker

```text
MIMALLOC-COMPARISON-SPEED-STABILITY-OBSERVATION-PACK-295X-001
```

## Decision

Add external elapsed-time evidence to the existing repeated measurement pack.

The selected bench set remains:

```text
representative-small-block-v0
representative-realloc-aligned-v0
representative-mixed-small-v0
representative-huge-ish-v0
```

The runner now records, per side and workload:

```text
external_elapsed_min_ms
external_elapsed_median_ms
external_elapsed_max_ms
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

## Follow-On

```text
MIMALLOC-COMPARISON-SPEED-STABILITY-OBSERVATION-CLOSEOUT-295X-001
```

## Stop Line

This row does not compute speed winners, require time parity, compute RSS
winners, add new workload families, change runtime behavior, make `empty` the
default, or open provider/DLL/replacement/hook/global allocator seams.
