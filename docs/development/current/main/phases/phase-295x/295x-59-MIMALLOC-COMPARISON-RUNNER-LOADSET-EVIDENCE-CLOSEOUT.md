---
Status: Landed
Date: 2026-05-25
Scope: phase-295x repeated runner selected-loadset evidence closeout.
Related:
  - docs/development/current/main/design/plugin-loadset-linking-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-58-MIMALLOC-COMPARISON-RUNNER-LOADSET-EVIDENCE.md
---

# 295x-59 Runner Loadset Evidence Closeout

## Blocker

```text
MIMALLOC-COMPARISON-RUNNER-LOADSET-EVIDENCE-CLOSEOUT-295X-001
```

## Decision

Close selected-loadset fields in repeated comparison evidence.

The repeated measurement runner now records the `.hako` runtime/plugin shape
before running workload samples:

```text
hako_runtime_config_profile=<root|empty>
hako_selected_loadset=<root|empty>
hako_plugin_load_policy=eager_selected
hako_selected_library_count=<n>
hako_missing_library_count=<n>
hako_loadset_preflight_ok=<0|1>
```

This makes fixed plugin/loadset footprint explicit in comparison evidence and
prevents future RSS reports from hiding whether the run used root plugin
compatibility or the explicit empty comparison profile.

## Follow-On

```text
MIMALLOC-COMPARISON-STANDALONE-EXE-ROUTE-CONTRACT-295X-001
```

The follow-on should define the standalone EXE route vocabulary on top of the
runtime config and selected-loadset evidence contract. It should not yet create
a full standalone packaging backend.

## Stop Line

This row does not change default NyRT plugin loading, make `empty` the default,
compute RSS winners, require RSS parity, open provider package / DLL generation,
or enable process replacement, hooks, backend matchers, or
`#[global_allocator]`.
