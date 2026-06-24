---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Design PluginExecMode passive box-core seam.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1223-BUILD-BOX-CORE-CONFIG-NEXT-SEAM-SELECTION-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
---

# BUILD-BOX-CORE-CONFIG-PLUGIN-EXEC-MODE-SEAM-DESIGN-001

## Decision

Move only the passive plugin execution mode enum:

```text
passive_owner_crate=hakorune-box-core
passive_owner_module=plugin
owner_type=PluginExecMode
main_crate_facade=src/config/env/box_factory_flags.rs
historical_import_path_preserved=crate::config::env::PluginExecMode
```

The env parser and fail-fast error reporting stay in the main crate because
they read environment variables and own process-exit behavior.

## Allowed

```text
move_PluginExecMode_enum=1
add_box_core_plugin_module=1
keep_env_reader_in_main_crate=1
keep_box_factory_plugin_logic_in_main_crate=1
```

## Forbidden

```text
move_plugin_exec_mode_parser=0
move_plugin_factory_logic=0
move_provider_policy_enums=0
move_filebox_mode_enums=0
move_runtime_or_provider_logic=0
```

## Contract

```text
output_contract=build-box-core-config-plugin-exec-mode-seam-design-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
selected_next_task=BUILD-BOX-CORE-CONFIG-PLUGIN-EXEC-MODE-PASSIVE-SPLIT-001

summary=ok
```
