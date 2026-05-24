---
Status: Landed
Date: 2026-05-25
Scope: phase-295x post loadset-aware measurement selection.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-65-MIMALLOC-COMPARISON-LOADSET-AWARE-REPEATED-MEASUREMENT-CLOSEOUT.md
---

# 295x-66 Post Loadset-Aware Measurement Selection

## Blocker

```text
MIMALLOC-COMPARISON-POST-LOADSET-AWARE-MEASUREMENT-SELECTION-295X-001
```

## Decision

Select a speed/stability observation row before more mimalloc porting.

Use the existing four workload families:

```text
representative-small-block-v0
representative-realloc-aligned-v0
representative-mixed-small-v0
representative-huge-ish-v0
```

These are the right first speed benches because their workload identity,
operation family, RSS collection, and loadset evidence are already fixed. New
benchmarks can come later after the current pack can report time and stability.

## Follow-On

```text
MIMALLOC-COMPARISON-SPEED-STABILITY-OBSERVATION-PACK-295X-001
```

## Stop Line

This row does not compute speed winners, memory winners, require parity, add new
workloads, change runtime behavior, or open provider/DLL/replacement/hook/global
allocator seams.
