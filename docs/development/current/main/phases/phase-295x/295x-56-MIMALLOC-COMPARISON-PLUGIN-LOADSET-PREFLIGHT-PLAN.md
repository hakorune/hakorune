---
Status: Landed
Date: 2026-05-25
Scope: phase-295x plugin loadset preflight plan artifact.
Related:
  - docs/development/current/main/design/plugin-loadset-linking-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-55-MIMALLOC-COMPARISON-PLUGIN-LOADSET-CONTRACT.md
---

# 295x-56 Plugin Loadset Preflight Plan

## Blocker

```text
MIMALLOC-COMPARISON-PLUGIN-LOADSET-PREFLIGHT-PLAN-295X-001
```

## Decision

Add a no-dlopen preflight plan tool for plugin loadsets:

```text
tools/allocator/hako_plugin_loadset_plan.py
```

The tool parses a selected runtime config and emits the selected plugin library
plan without loading shared libraries or executing provider code.

## Contract

The plan output is JSON:

```text
output_contract=hako-plugin-loadset-plan-v0
selected_loadset=<root|default|all|empty|no_plugins>
plugin_load_policy=eager_selected
library_count=<n>
missing_library_count=<n>
preflight_ok=<0|1>
provider_activation=0
host_replacement=0
hook_installed=0
global_allocator_installed=0
winner_claim=0
```

For this row:

- `empty` / `no_plugins` select no libraries and should be preflight-ok;
- `root` / `default` / `all` select every `[libraries]` entry from the selected
  config and report path diagnostics;
- unsupported future loadsets such as `app` and `core` fail fast until a
  package manifest owner is introduced.

## Follow-On

```text
MIMALLOC-COMPARISON-PLUGIN-LOADSET-PREFLIGHT-CLOSEOUT-295X-001
```

The closeout may decide whether to wire this plan into comparison-runner
evidence or keep it as a standalone diagnostic before the next workload row.

## Stop Line

This row does not call `dlopen`, change default NyRT plugin loading, generate a
runtime `nyash.toml`, teach NyRT to read `hako.toml` directly, make `empty` the
default runtime profile, compute RSS winners, require RSS parity, or open
provider/DLL/replacement/hook/global allocator seams.
