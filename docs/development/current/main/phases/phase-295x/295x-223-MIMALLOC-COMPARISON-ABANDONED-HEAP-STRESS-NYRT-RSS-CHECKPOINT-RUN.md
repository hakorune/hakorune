---
Status: Landed
Date: 2026-05-25
Scope: run the NyRT self-RSS checkpoint diagnostic on the abandoned-heap stress path and classify the fixed RSS step.
Blocker: MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-RSS-CHECKPOINT-RUN-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-222-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-RSS-CHECKPOINT-DIAGNOSTIC.md
  - docs/development/current/main/phases/phase-295x/295x-224-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-HOST-BASELINE-SELECTION.md
  - tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_nyrt_rss_checkpoint_run_guard.sh
---

# 295x-223 Abandoned Heap Stress NyRT RSS Checkpoint Run

## Decision

Close:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-RSS-CHECKPOINT-RUN-295X-002
```

Run the empty no-output exact-EXE with:

```text
HAKO_NYRT_RSS_CHECKPOINTS=1
NYASH_NYRT_SILENT_RESULT=1
```

Observed shape:

```text
entry_start          small baseline
after_ring0          no meaningful jump
after_runtime_hooks  no meaningful jump
after_plugin_host    large fixed RSS jump
before_ny_main       stable after plugin host
after_ny_main        stable after empty main
```

This keeps the fixed RSS step visible inside the exact-EXE process without
changing runtime behavior when the env is unset.

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-HOST-BASELINE-SELECTION-295X-002
```

The follow-on should choose a narrow seam for plugin-host baseline diagnosis:

```text
plugin host init disabled diagnostic
plugin config scan footprint
plugin library load footprint
minimal no-plugin exact-EXE mode
```

It should not immediately remove plugin host init from normal NyRT.

## Stop Line

This row does not reduce baseline RSS, change compiler/linker/runtime behavior
by default, compute memory/performance winners, require RSS parity, enable
provider/DLL or host replacement seams, install hooks, or open worker/TLS,
atomics, remote-free stress, abandoned heap stress, OSVM page-source parity, or
native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_nyrt_rss_checkpoint_run_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
