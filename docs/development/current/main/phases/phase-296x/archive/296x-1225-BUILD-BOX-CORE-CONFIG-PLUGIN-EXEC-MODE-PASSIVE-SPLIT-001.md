---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Move PluginExecMode passive vocabulary to hakorune-box-core.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1224-BUILD-BOX-CORE-CONFIG-PLUGIN-EXEC-MODE-SEAM-DESIGN-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
---

# BUILD-BOX-CORE-CONFIG-PLUGIN-EXEC-MODE-PASSIVE-SPLIT-001

## Change

```text
passive_owner_crate=hakorune-box-core
new_owner_module=crates/hakorune_box_core/src/plugin.rs
passive_owner_type=PluginExecMode
main_crate_facade=src/config/env/box_factory_flags.rs
historical_import_path_preserved=crate::config::env::PluginExecMode
behavior_changed=0
```

Parsing `NYASH_PLUGIN_EXEC_MODE` remains in the main crate. The plugin factory
still reads `crate::config::env::plugin_exec_mode()` and matches the same enum
variants through the existing config facade.

## Verification

```text
cargo_test_hakorune_box_core=green
cargo_check_default=green
current_state_pointer_guard=green
diff_check=green
```

## Contract

```text
output_contract=build-box-core-config-plugin-exec-mode-passive-split-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
new_crate_depends_on_main_crate=0
plugin_env_reader_owner=main_crate
plugin_factory_logic_owner=main_crate

summary=ok
```

## Next

```text
next_task=BUILD-BOX-CORE-CONFIG-PLUGIN-EXEC-MODE-POST-SPLIT-MEASUREMENT-001
purpose=measure cold release build after moving PluginExecMode passive vocabulary
implementation_allowed=0
measurement_allowed=1
```
