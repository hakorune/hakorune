---
Status: Landed
Date: 2026-05-25
Scope: add the narrow plugin-host substage RSS diagnostic after the abandoned-heap stress NyRT plugin-host baseline selection.
Blocker: MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-HOST-SUBSTAGE-DIAGNOSTIC-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-224-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-HOST-BASELINE-SELECTION.md
  - docs/development/current/main/phases/phase-295x/295x-226-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION.md
  - tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_nyrt_plugin_host_substage_diagnostic_guard.sh
---

# 295x-225 Abandoned Heap Stress NyRT Plugin Host Substage Diagnostic

## Decision

Close:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-HOST-SUBSTAGE-DIAGNOSTIC-295X-002
```

Add and run env-gated runtime plugin-host RSS checkpoints under the existing
diagnostic switch:

```text
HAKO_NYRT_RSS_CHECKPOINTS=1
```

This keeps the same observation path as the earlier NyRT plugin-host
diagnostic rows and splits the fixed cost into narrower checkpoints without
changing runtime behavior when the env is unset.

## Observed Representative Run

The exact values are environment-dependent, but the representative no-output
exact-EXE run still showed the fixed jump concentrated in plugin host
initialization.

- `after_runtime_hooks` stayed flat.
- `plugin_host_after_host_config_parse` introduced the first noticeable rise.
- `plugin_loader_after_library_loop` carried most of the jump.
- `plugin_loader_after_prebirth_singletons` stayed unchanged from the
  library-loop level.
- `after_plugin_host`, `before_ny_main`, and `after_ny_main` matched the
  stabilized post-host level.

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION-295X-002
```

The follow-on should use the same env-gated observation path and measure the
plugin library load-set footprint after host configuration parsing.

## Conclusion

The next useful seam is still a plugin library footprint / load-set diagnostic,
not an immediate runtime shrink.

## Stop Line

This row does not shrink RSS, disable plugins by default, change default plugin
host behavior, alter provider selection, compute winner claims, open
provider/DLL/replacement/hook/global allocator seams, or make RSS parity
claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_nyrt_plugin_host_substage_diagnostic_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
