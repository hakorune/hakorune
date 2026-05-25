---
Status: Landed
Date: 2026-05-25
Scope: phase-295x smaller-default-load-set evidence on the external malloc-large path
Blocker: MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-EVIDENCE-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-203-MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-CLOSEOUT.md
  - tools/checks/k2_wide_phase295x_malloc_large_nyrt_plugin_loadset_smaller_default_set_evidence_guard.sh
---

# 295x-204 NyRT Plugin Load-Set Smaller Default Set Evidence

## Decision

Close:

```text
MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-EVIDENCE-295X-002
```

Ran repeated comparison samples with the empty default runtime config and confirmed it stays materially smaller than explicit root compatibility.

## Evidence

Repeated comparison samples with `sample-count=5` and `warmup-count=1`
showed the smaller default load set staying materially below explicit root
compatibility on every selected workload:

| workload | empty default median RSS | explicit root median RSS |
| --- | ---: | ---: |
| representative-small-block-v0 | 3,670,016 | 9,609,216 |
| representative-realloc-aligned-v0 | 3,649,536 | 9,379,840 |
| representative-mixed-small-v0 | 3,584,000 | 9,506,816 |
| representative-huge-ish-v0 | 3,637,248 | 9,494,528 |

The repeated runner default is now `empty`, while explicit `root`
compatibility remains available for comparison runs. The runner reports this
as `hako_runtime_config_default=empty`.

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-CLOSEOUT-295X-002
```

## Stop Line

This row does not broaden provider/DLL or host replacement seams, install
hooks, change default runtime behavior, compute winner claims, or make RSS
parity claims unless this card explicitly says so.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_malloc_large_nyrt_plugin_loadset_smaller_default_set_evidence_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
