---
Status: Landed
Date: 2026-05-25
Scope: phase-295x smaller-default-load-set closeout on the abandoned-heap stress path.
Blocker: MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SPEED-STABILITY-OBSERVATION-PACK-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-229-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-EVIDENCE.md
  - tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_nyrt_plugin_loadset_smaller_default_set_closeout_guard.sh
---

# 295x-230 Abandoned Heap Stress NyRT Plugin Load-Set Smaller Default Set Closeout

## Decision

Close:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-CLOSEOUT-295X-002
```

The repeated comparison evidence showed the empty default runtime config stays materially smaller than explicit root compatibility on every selected workload.
That closes the smaller-default-load-set evidence seam.

## Evidence

Repeated comparison samples with `sample-count=5` and `warmup-count=1`
showed the smaller default load set staying materially below explicit root
compatibility on every selected workload:

| workload | empty default median RSS | explicit root median RSS |
| --- | ---: | ---: |
| representative-small-block-v0 | 3,588,096 | 9,457,664 |
| representative-realloc-aligned-v0 | 3,657,728 | 9,580,544 |
| representative-mixed-small-v0 | 3,641,344 | 9,637,888 |
| representative-huge-ish-v0 | 3,612,672 | 9,478,144 |

The repeated runner default is now `empty`, while explicit `root`
compatibility remains available for comparison runs. The runner reports this
as `hako_runtime_config_default=empty`.

## Diagnostic Summary

The next useful seam is a speed/stability observation pack over the existing
comparison workloads. The comparison lane should now measure elapsed timing
alongside the already-fixed RSS evidence before widening anything else.

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SPEED-STABILITY-OBSERVATION-PACK-295X-002
```

## Stop Line

This row does not broaden provider/DLL or host replacement seams, install
hooks, change default runtime behavior, compute winner claims, or make RSS
parity claims unless this card explicitly says so.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_nyrt_plugin_loadset_smaller_default_set_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
