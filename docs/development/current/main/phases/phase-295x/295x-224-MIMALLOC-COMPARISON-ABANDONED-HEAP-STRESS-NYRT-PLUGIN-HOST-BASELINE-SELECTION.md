---
Status: Landed
Date: 2026-05-25
Scope: select a narrow plugin-host baseline diagnosis after the abandoned-heap stress NyRT RSS checkpoint run.
Blocker: MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-HOST-BASELINE-SELECTION-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-223-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-RSS-CHECKPOINT-RUN.md
  - docs/development/current/main/phases/phase-295x/295x-225-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-HOST-SUBSTAGE-DIAGNOSTIC.md
  - tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_nyrt_plugin_host_baseline_selection_guard.sh
---

# 295x-224 Abandoned Heap Stress NyRT Plugin Host Baseline Selection

## Decision

Close:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-HOST-BASELINE-SELECTION-295X-002
```

The abandoned-heap stress NyRT RSS checkpoint run kept the fixed RSS jump
concentrated in plugin host initialization between `after_runtime_hooks` and
`after_plugin_host`.

The next diagnostic keeps the same env-gated observation path with
`HAKO_NYRT_RSS_CHECKPOINTS=1` and opens a narrow plugin-host substage seam.

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-HOST-SUBSTAGE-DIAGNOSTIC-295X-002
```

The follow-on should keep the same env-gated observation path and add runtime
plugin-host checkpoints for:

- unified host `load_libraries` entry;
- v2 loader config load;
- host config read / parse;
- v2 library loop;
- v2 plugin-root loop;
- singleton prebirth;
- unified host `load_all_plugins` return.

It should not immediately remove plugin host init from normal NyRT.

## Stop Line

This row does not reduce baseline RSS, change compiler/linker/runtime behavior
by default, compute memory/performance winners, require RSS parity, enable
provider/DLL or host replacement seams, install hooks, or open worker/TLS,
atomics, remote-free stress, abandoned heap stress, OSVM page-source parity, or
native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_nyrt_plugin_host_baseline_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
