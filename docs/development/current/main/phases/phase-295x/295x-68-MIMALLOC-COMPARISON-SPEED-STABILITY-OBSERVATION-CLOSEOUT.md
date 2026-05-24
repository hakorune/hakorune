---
Status: Landed
Date: 2026-05-25
Scope: phase-295x speed/stability observation closeout.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-67-MIMALLOC-COMPARISON-SPEED-STABILITY-OBSERVATION-PACK.md
---

# 295x-68 Speed / Stability Observation Closeout

## Blocker

```text
MIMALLOC-COMPARISON-SPEED-STABILITY-OBSERVATION-CLOSEOUT-295X-001
```

## Closeout

The selected speed/stability observation pack ran the existing comparison
workload set:

```text
representative-small-block-v0
representative-realloc-aligned-v0
representative-mixed-small-v0
representative-huge-ish-v0
```

The pack uses:

```text
sample_count=5
warmup_count=1
hako_runtime_config_profile=empty
hako_selected_loadset=empty
winner_claim=0
```

## Observation

RSS evidence remains usable for the current comparison rows. The external
elapsed-time evidence is present and ordered, but the selected workloads are too
small for `/usr/bin/time` millisecond-scale evidence:

```text
workload_0 hako/c elapsed median: 1ms / 1ms
workload_1 hako/c elapsed median: 1ms / 1ms
workload_2 hako/c elapsed median: 1ms / 1ms
workload_3 hako/c elapsed median: 1ms / 1ms
```

This means the current elapsed-time field is a stability/exit observation, not a
speed comparison signal.

## Decision

Close this pack without speed winners.

Select a high-resolution timing seam before using elapsed-time evidence for
speed comparison:

```text
MIMALLOC-COMPARISON-HIGH-RES-TIMING-SEAM-SELECTION-295X-001
```

Candidate direction:

```text
- keep the same workload identities and loadset evidence;
- add a timing source with sub-millisecond resolution;
- keep RSS winner claims and speed winner claims closed;
- decide whether to increase iteration counts, repeat inside the executable,
  or collect monotonic timestamps around the workload body.
```

## Stop Line

This row does not compute speed winners, require time parity, compute RSS
winners, change workload semantics, change runtime behavior, make `empty` the
default, or open provider/DLL/replacement/hook/global allocator seams.
