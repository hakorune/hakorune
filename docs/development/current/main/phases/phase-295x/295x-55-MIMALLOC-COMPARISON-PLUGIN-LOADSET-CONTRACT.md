---
Status: Landed
Date: 2026-05-25
Scope: phase-295x plugin loadset/linking contract.
Related:
  - docs/development/current/main/design/plugin-loadset-linking-ssot.md
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-54-MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-MEASUREMENT-CLOSEOUT.md
---

# 295x-55 Plugin Loadset Contract

## Blocker

```text
MIMALLOC-COMPARISON-PLUGIN-LOADSET-CONTRACT-295X-001
```

## Decision

Define plugin linking as a selected-loadset contract.

The comparison lane will not rely on hidden root plugin loading, implicit lazy
loading, or post-failure provider discovery. Runtime plugin selection should be
visible as:

```text
selected_loadset=<root|empty|all|app|core>
plugin_load_policy=eager_selected
selected_library_count=<n>
```

The default compatibility behavior remains `root`. Minimal comparison profiles
continue to be explicit and opt-in.

## Contract

The SSOT is:

```text
docs/development/current/main/design/plugin-loadset-linking-ssot.md
```

The next implementation row should add a preflight plan artifact:

```text
output_contract=hako-plugin-loadset-plan-v0
selected_loadset=...
plugin_load_policy=eager_selected
library_count=...
missing_library_count=...
provider_activation=0
host_replacement=0
hook_installed=0
global_allocator_installed=0
winner_claim=0
```

The preflight row must not load plugin libraries. It should only parse the
selected config, classify the requested loadset, and report library/path
diagnostics.

## Follow-On

```text
MIMALLOC-COMPARISON-PLUGIN-LOADSET-PREFLIGHT-PLAN-295X-001
```

## Stop Line

This row does not change default NyRT plugin loading, teach NyRT to read
`hako.toml` directly, delete `nyash.toml` compatibility, make `empty` the
default runtime profile, compute RSS winners, require RSS parity, or open
provider/DLL/replacement/hook/global allocator seams.
