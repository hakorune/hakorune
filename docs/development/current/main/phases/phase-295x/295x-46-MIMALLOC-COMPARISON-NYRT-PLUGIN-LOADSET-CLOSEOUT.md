---
Status: Landed
Date: 2026-05-25
Scope: phase-295x plugin load-set diagnostic closeout.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-45-MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-DIAGNOSTIC.md
---

# 295x-46 NyRT Plugin Load-Set Closeout

## Blocker

```text
MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-CLOSEOUT-295X-001
```

## Decision

Close the load-set diagnostic and select a comparison-runner-only minimal
config pilot:

```text
MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-PILOT-295X-001
```

The evidence shows:

- generated `empty_config` keeps plugin-host RSS delta at zero for the
  no-output exact-EXE;
- root current config contributes both config parse/caching and dynamic plugin
  library load cost;
- `all_existing` and the current root config load far more than the empty
  comparison probe needs;
- the largest single-plugin observation in the representative run was
  `libnyash_python_parser_plugin.so`.

The selected pilot must not change default NyRT behavior. It should only test
whether the phase-295x exact-EXE comparison runner can execute no-plugin
comparison workloads from a generated minimal `nyash.toml` working directory.

## Stop Line

This row does not shrink default runtime RSS, change plugin host defaults,
disable plugins by default, alter provider selection, compute memory winners,
open provider/DLL/replacement/hook/global allocator seams, or require RSS
parity.
