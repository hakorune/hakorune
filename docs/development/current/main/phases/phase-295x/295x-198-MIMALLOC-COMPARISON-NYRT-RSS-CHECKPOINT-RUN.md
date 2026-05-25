---
Status: Current
Date: 2026-05-25
Scope: run the NyRT self-RSS checkpoint diagnostic on the external `malloc-large` empty no-output exact-EXE path.
Blocker: MIMALLOC-COMPARISON-NYRT-RSS-CHECKPOINT-RUN-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-197-MIMALLOC-COMPARISON-NYRT-RSS-CHECKPOINT-DIAGNOSTIC.md
  - tools/checks/k2_wide_phase295x_malloc_large_nyrt_rss_checkpoint_run_guard.sh
---

# 295x-198 Mimalloc Comparison NyRT RSS Checkpoint Run

## Decision

Close:

```text
MIMALLOC-COMPARISON-NYRT-RSS-CHECKPOINT-RUN-295X-002
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

This means the fixed exact-EXE RSS cost is not caused by `.hako` evidence
printing or the empty app body. The main step is currently around global
plugin host initialization.

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-NYRT-PLUGIN-HOST-BASELINE-SELECTION-295X-002
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
bash tools/checks/k2_wide_phase295x_malloc_large_nyrt_rss_checkpoint_run_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
