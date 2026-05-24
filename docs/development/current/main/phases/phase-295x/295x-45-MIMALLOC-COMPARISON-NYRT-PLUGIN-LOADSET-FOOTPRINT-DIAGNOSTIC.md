---
Status: Landed
Date: 2026-05-25
Scope: phase-295x plugin load-set RSS diagnostic.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-44-MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION.md
---

# 295x-45 NyRT Plugin Load-Set Footprint Diagnostic

## Blocker

```text
MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-DIAGNOSTIC-295X-001
```

## Decision

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
- root current config behavior.
- per-plugin single-load ranking.

The next useful row should select a shrink candidate based on the report. The
likely options are:

- lazy plugin host initialization for no-plugin exact-EXE paths;
- smaller default load set for exact-EXE no-output / comparison runs;
- split config parse cache from dynamic library loading.

## Observed Representative Run

```text
empty_config:
  config_delta_bytes = 0
  library_loop_delta_bytes = 0
  total_plugin_host_delta_bytes = 0

console_only:
  library_loop_delta_bytes = 1,335,296

core_six_existing:
  library_loop_delta_bytes = 2,449,408

regex_only:
  library_loop_delta_bytes = 1,343,488

all_existing:
  library_loop_delta_bytes = 6,627,328

root_current:
  config_delta_bytes = 2,134,016
  library_loop_delta_bytes = 5,595,136
  total_plugin_host_delta_bytes = 7,729,152
```

Top single-plugin observations in this run:

```text
single_libnyash_python_parser_plugin_so:
  library_loop_delta_bytes = 4,255,744

single_libnyash_math_plugin_so:
  library_loop_delta_bytes = 1,462,272

single_libnyash_filebox_plugin_so:
  library_loop_delta_bytes = 1,388,544

single_libnyash_json_plugin_so:
  library_loop_delta_bytes = 1,388,544

single_libnyash_toml_plugin_so:
  library_loop_delta_bytes = 1,388,544
```

The diagnostic shows that a generated empty config removes the plugin-host RSS
delta for the no-output exact-EXE, while root current config spends roughly
2 MiB on config parse/caching and roughly 5.6 MiB on dynamic library loading.

Selected follow-on:

```text
MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-CLOSEOUT-295X-001
```

## Stop Line

This row does not shrink RSS, change default plugin-host behavior, alter
provider selection, disable plugins by default, compute memory winners, open
provider/DLL/replacement/hook/global allocator seams, or require RSS parity.
