---
Status: Current
Date: 2026-05-25
Scope: phase-295x plugin-host baseline seam selection after the env-gated NyRT self-RSS checkpoint run on the external `malloc-large` path.
Blocker: MIMALLOC-COMPARISON-NYRT-PLUGIN-HOST-BASELINE-SELECTION-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-198-MIMALLOC-COMPARISON-NYRT-RSS-CHECKPOINT-RUN.md
  - tools/checks/k2_wide_phase295x_malloc_large_nyrt_plugin_host_baseline_selection_guard.sh
---

# 295x-199 NyRT Plugin Host Baseline Selection

## Decision

Close:

```text
MIMALLOC-COMPARISON-NYRT-PLUGIN-HOST-BASELINE-SELECTION-295X-002
```

The NyRT checkpoint run showed the fixed exact-EXE RSS jump is still
concentrated in plugin host initialization between `after_runtime_hooks` and
`after_plugin_host`.

The next diagnostic keeps the same env-gated observation path with
`HAKO_NYRT_RSS_CHECKPOINTS=1` and opens a narrow plugin-host substage seam.

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-NYRT-PLUGIN-HOST-SUBSTAGE-DIAGNOSTIC-295X-002
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
bash tools/checks/k2_wide_phase295x_malloc_large_nyrt_plugin_host_baseline_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
