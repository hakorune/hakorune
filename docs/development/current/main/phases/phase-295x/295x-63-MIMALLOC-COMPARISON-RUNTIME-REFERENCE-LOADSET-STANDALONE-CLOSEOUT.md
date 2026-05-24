---
Status: Landed
Date: 2026-05-25
Scope: phase-295x runtime reference docs closeout.
Related:
  - docs/reference/runtime/plugin-loadsets.md
  - docs/reference/runtime/standalone-exe-routes.md
  - docs/development/current/main/phases/phase-295x/295x-62-MIMALLOC-COMPARISON-RUNTIME-REFERENCE-LOADSET-STANDALONE.md
---

# 295x-63 Runtime Reference Loadset / Standalone Closeout

## Blocker

```text
MIMALLOC-COMPARISON-RUNTIME-REFERENCE-LOADSET-STANDALONE-CLOSEOUT-295X-001
```

## Decision

Close the runtime reference docs for plugin loadsets and standalone EXE routes.

Do not add standalone labels to comparison evidence yet. The current repeated
runner evidence already exposes the concrete runtime/loadset facts:

```text
hako_runtime_config_profile
hako_selected_loadset
hako_plugin_load_policy
hako_selected_library_count
```

The comparison lane should return to mimalloc measurement work by refreshing the
full repeated pack with those selected-loadset fields present.

## Follow-On

```text
MIMALLOC-COMPARISON-LOADSET-AWARE-REPEATED-MEASUREMENT-PACK-295X-001
```

## Stop Line

This row does not add standalone evidence labels, change runtime behavior,
generate standalone packages, compute RSS winners, require RSS parity, or open
provider/DLL/replacement/hook/global allocator seams.
