---
Status: Current
Date: 2026-05-25
Scope: phase-295x plugin load-set footprint selection on the external malloc-large path
Blocker: MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-200-MIMALLOC-COMPARISON-NYRT-PLUGIN-HOST-SUBSTAGE-DIAGNOSTIC.md
  - tools/checks/k2_wide_phase295x_malloc_large_nyrt_plugin_loadset_footprint_selection_guard.sh
---

# 295x-201 NyRT Plugin Load-Set Footprint Selection

## Decision

Close:

```text
MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION-295X-002
```

Selected the plugin load-set footprint diagnostic after the malloc-large plugin-host substage diagnostic.

The next row runs the same no-output exact-EXE under multiple temporary
`nyash.toml` files:

- `empty_config`;
- `console_only`;
- `core_six_existing`;
- `regex_only`;
- `all_existing`;
- `root_current`.

This isolates dynamic library load RSS from the normal root config.

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-DIAGNOSTIC-295X-002
```

## Stop Line

This row does not broaden provider/DLL or host replacement seams, install
hooks, change default runtime behavior, compute winner claims, or make RSS
parity claims unless this card explicitly says so.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_malloc_large_nyrt_plugin_loadset_footprint_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
