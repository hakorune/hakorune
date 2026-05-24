---
Status: Landed
Date: 2026-05-25
Scope: phase-295x plugin load-set footprint selection.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-43-MIMALLOC-COMPARISON-NYRT-PLUGIN-HOST-SUBSTAGE-DIAGNOSTIC.md
---

# 295x-44 NyRT Plugin Load-Set Footprint Selection

## Blocker

```text
MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION-295X-001
```

## Decision

Select a generated-config load-set footprint diagnostic:

```text
MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-DIAGNOSTIC-295X-001
```

The next row runs the same no-output exact-EXE under multiple temporary
`nyash.toml` files:

- `empty_config`;
- `console_only`;
- `core_six_existing`;
- `regex_only`;
- `all_existing`;
- `root_current`.

This isolates dynamic library load RSS from the normal root config.

## Stop Line

This row does not shrink RSS, change default plugin-host behavior, alter
provider selection, disable plugins by default, compute memory winners, open
provider/DLL/replacement/hook/global allocator seams, or require RSS parity.
