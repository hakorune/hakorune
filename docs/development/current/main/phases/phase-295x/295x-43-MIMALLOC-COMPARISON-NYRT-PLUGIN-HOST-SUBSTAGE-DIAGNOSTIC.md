---
Status: Landed
Date: 2026-05-25
Scope: phase-295x plugin-host substage RSS diagnostic.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-42-MIMALLOC-COMPARISON-NYRT-PLUGIN-HOST-BASELINE-SELECTION.md
---

# 295x-43 NyRT Plugin Host Substage Diagnostic

## Blocker

```text
MIMALLOC-COMPARISON-NYRT-PLUGIN-HOST-SUBSTAGE-DIAGNOSTIC-295X-001
```

## Decision

Add and run env-gated runtime plugin-host RSS checkpoints under the existing
diagnostic switch:

```text
HAKO_NYRT_RSS_CHECKPOINTS=1
```

## Observed Representative Run

The exact values are environment-dependent, but the representative no-output
exact-EXE run showed the large jump is mostly inside the dynamic library load
loop, with a smaller config parse/caching component and no singleton prebirth
increase.

```text
after_runtime_hooks:
  1,363,968 bytes

plugin_host_after_host_config_parse:
  3,497,984 bytes
  config_delta_bytes = 2,134,016

plugin_loader_after_library_loop:
  9,113,600 bytes
  library_loop_delta_bytes = 5,615,616

plugin_loader_after_prebirth_singletons:
  9,113,600 bytes
  prebirth_delta_bytes = 0

after_plugin_host / before_ny_main / after_ny_main:
  9,113,600 bytes
  empty no-output main adds no RSS
```

## Conclusion

The next useful seam is a plugin library footprint / load-set diagnostic, not
an immediate runtime shrink.

Selected follow-on:

```text
MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION-295X-001
```

## Stop Line

This row does not shrink RSS, disable plugins by default, change provider
selection, alter plugin loading semantics, compute winner claims, open
provider/DLL/replacement/hook/global allocator seams, or make RSS parity claims.
