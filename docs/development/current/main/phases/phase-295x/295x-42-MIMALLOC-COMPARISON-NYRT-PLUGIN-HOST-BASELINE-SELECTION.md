---
Status: Landed
Date: 2026-05-25
Scope: phase-295x plugin-host baseline diagnostic seam selection.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-41-MIMALLOC-COMPARISON-NYRT-RSS-CHECKPOINT-RUN.md
---

# 295x-42 NyRT Plugin Host Baseline Selection

## Blocker

```text
MIMALLOC-COMPARISON-NYRT-PLUGIN-HOST-BASELINE-SELECTION-295X-001
```

## Decision

Close the selection row and open a narrow plugin-host substage diagnostic:

```text
MIMALLOC-COMPARISON-NYRT-PLUGIN-HOST-SUBSTAGE-DIAGNOSTIC-295X-001
```

The previous checkpoint run showed the fixed `.hako` no-output exact-EXE RSS
jump happens between `after_runtime_hooks` and `after_plugin_host`.

The next diagnostic will keep the same env-gated observation path and add
runtime plugin-host checkpoints for:

- unified host `load_libraries` entry;
- v2 loader config load;
- host config read / parse;
- v2 library loop;
- v2 plugin-root loop;
- singleton prebirth;
- unified host `load_all_plugins` return.

## Stop Line

This row does not shrink RSS, change default plugin host behavior, disable
plugins by default, alter provider selection, compute memory winners, open
provider/DLL/replacement/hook/global allocator seams, or introduce broad
runtime initialization instrumentation.

The selected follow-on is diagnostic-only and remains gated by
`HAKO_NYRT_RSS_CHECKPOINTS=1`.
