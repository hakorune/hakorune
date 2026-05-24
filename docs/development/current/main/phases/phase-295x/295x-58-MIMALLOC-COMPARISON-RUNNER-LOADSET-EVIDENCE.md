---
Status: Landed
Date: 2026-05-25
Scope: phase-295x comparison runner loadset evidence.
Related:
  - docs/development/current/main/design/plugin-loadset-linking-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-57-MIMALLOC-COMPARISON-PLUGIN-LOADSET-PREFLIGHT-CLOSEOUT.md
---

# 295x-58 Runner Loadset Evidence

## Blocker

```text
MIMALLOC-COMPARISON-RUNNER-LOADSET-EVIDENCE-295X-001
```

## Decision

Add selected `.hako` plugin loadset fields to repeated comparison evidence:

```text
hako_selected_loadset=<root|empty>
hako_plugin_load_policy=eager_selected
hako_selected_library_count=<n>
hako_missing_library_count=<n>
hako_loadset_preflight_ok=<0|1>
```

The fields are produced from the no-dlopen loadset preflight plan before the
runner starts workload samples. This makes the fixed runtime/plugin footprint
visible in every repeated measurement report without changing runtime loading
behavior.

## Contract

`--hako-runtime-config empty` maps to:

```text
hako_selected_loadset=empty
hako_selected_library_count=0
hako_loadset_preflight_ok=1
```

`--hako-runtime-config root` maps to the root compatibility loadset and reports
the configured library count and missing path diagnostics. Root path diagnostics
are evidence only in this row.

## Follow-On

```text
MIMALLOC-COMPARISON-RUNNER-LOADSET-EVIDENCE-CLOSEOUT-295X-001
```

## Stop Line

This row does not make `empty` the default, change NyRT plugin loading, require
root plugin paths to be present, compute RSS winners, require RSS parity, or
open provider/DLL/replacement/hook/global allocator seams.
