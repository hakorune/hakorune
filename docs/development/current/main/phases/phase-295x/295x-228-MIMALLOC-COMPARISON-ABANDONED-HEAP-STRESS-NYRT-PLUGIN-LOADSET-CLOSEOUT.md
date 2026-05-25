---
Status: Current
Date: 2026-05-25
Scope: phase-295x plugin load-set closeout on the abandoned-heap stress path.
Blocker: MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-CLOSEOUT-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-227-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-FOOTPRINT-DIAGNOSTIC.md
  - tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_nyrt_plugin_loadset_closeout_guard.sh
---

# 295x-228 Abandoned Heap Stress NyRT Plugin Load-Set Closeout

## Decision

Close:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-CLOSEOUT-295X-002
```

The diagnostic showed two useful facts:

- `empty_config` keeps plugin-host RSS delta at zero for the no-output
  exact-EXE;
- `root_current` still spends roughly 2 MiB on config parse/caching and
  roughly 5.4 MiB on dynamic library loading.

That means the next seam is a smaller default load set for exact-EXE
comparison runs, not an immediate runtime shrink.

## Diagnostic Summary

The selected shrink candidate is:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-PILOT-295X-002
```

Pilot implementation:

```text
tools/allocator/mimalloc_repeated_measurement_runner.py
  default hako runtime config: empty
  explicit root compatibility: preserved
  report field: hako_runtime_config_default=empty
```

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-PILOT-295X-002
```

## Stop Line

This row does not broaden provider/DLL or host replacement seams, install
hooks, change default runtime behavior, compute winner claims, or make RSS
parity claims unless this card explicitly says so.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_nyrt_plugin_loadset_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
