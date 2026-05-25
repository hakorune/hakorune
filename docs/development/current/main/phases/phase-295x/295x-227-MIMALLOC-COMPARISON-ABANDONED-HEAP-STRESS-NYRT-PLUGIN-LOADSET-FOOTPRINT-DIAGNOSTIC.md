---
Status: Current
Date: 2026-05-25
Scope: phase-295x plugin load-set RSS diagnostic on the abandoned-heap stress path.
Blocker: MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-FOOTPRINT-DIAGNOSTIC-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-226-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION.md
  - tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_nyrt_plugin_loadset_footprint_diagnostic_guard.sh
---

# 295x-227 Abandoned Heap Stress NyRT Plugin Load-Set Footprint Diagnostic

## Decision

Close:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-FOOTPRINT-DIAGNOSTIC-295X-002
```

Run a generated-config load-set diagnostic for the no-output exact-EXE.

The diagnostic uses:

```text
tools/allocator/nyrt_plugin_loadset_footprint.py
```

## Interpretation

The load-set report is diagnostic evidence only. It separates:

- minimal config parse cost;
- single plugin load cost;
- core-six existing plugin load cost;
- large plugin load cost;
- all existing plugin load cost;
- root current config behavior;
- per-plugin single-load ranking.

The next useful row should select a shrink candidate based on the report. The
likely options are:

- lazy plugin host initialization for no-plugin exact-EXE paths;
- smaller default load set for exact-EXE no-output / comparison runs;
- split config parse cache from dynamic library loading.

## Observed Representative Run

The exact values are environment-dependent, but the representative no-output
exact-EXE run showed the plugin-host fixed cost is still dominated by the root
config's dynamic library loading.

```text
empty_config:
  config_delta_bytes = 0
  library_loop_delta_bytes = 0
  total_plugin_host_delta_bytes = 0

console_only:
  library_loop_delta_bytes = 1,458,176

core_six_existing:
  library_loop_delta_bytes = 2,412,544

regex_only:
  library_loop_delta_bytes = 1,462,272

all_existing:
  library_loop_delta_bytes = 6,909,952

root_current:
  config_delta_bytes = 2,043,904
  library_loop_delta_bytes = 5,640,192
  total_plugin_host_delta_bytes = 7,684,096
```

Top single-plugin observations in this run:

```text
single_libnyash_python_parser_plugin_so:
  library_loop_delta_bytes = 4,247,552

single_libnyash_math_plugin_so:
  library_loop_delta_bytes = 1,458,176

single_libnyash_filebox_plugin_so:
  library_loop_delta_bytes = 1,462,272

single_libnyash_json_plugin_so:
  library_loop_delta_bytes = 1,462,272

single_libnyash_toml_plugin_so:
  library_loop_delta_bytes = 1,458,176
```

The diagnostic shows that a generated empty config removes the plugin-host RSS
delta for the no-output exact-EXE, while root current config spends roughly
2 MiB on config parse/caching and roughly 5.6 MiB on dynamic library loading.

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-CLOSEOUT-295X-002
```

## Conclusion

The next useful seam is still a plugin library footprint / load-set diagnostic,
not an immediate runtime shrink.

## Stop Line

This row does not broaden provider/DLL or host replacement seams, install
hooks, change default runtime behavior, compute winner claims, or make RSS
parity claims unless this card explicitly says so.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_nyrt_plugin_loadset_footprint_diagnostic_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
