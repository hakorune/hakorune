---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Select next box-core/config passive seam after first box-core split.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1222-BUILD-BOX-CORE-CONFIG-POST-SPLIT-MEASUREMENT-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
---

# BUILD-BOX-CORE-CONFIG-NEXT-SEAM-SELECTION-001

## Inventory

```text
current_blocker=BUILD-BOX-CORE-CONFIG-NEXT-SEAM-SELECTION-001
existing_box_core_owner_types=FactoryPolicy,FactoryType

candidate=PluginExecMode
candidate_owner_file=src/config/env/box_factory_flags.rs
candidate_used_by=src/box_factory/plugin.rs
candidate_dependency_free=1
candidate_scope=box_factory_plugin_route_policy_vocabulary

candidate=ProviderPolicy,FileBoxMode
candidate_owner_file=src/config/provider_env.rs
candidate_used_by=src/runner/modes/common_util/provider_registry.rs
candidate_used_by=src/box_factory/registry.rs
candidate_scope=provider_filebox_selection_policy
candidate_dependency_free_enums=1
candidate_env_reader_coupled=1

candidate=NyashBox,BoxCore,BoxBase
candidate_owner_file=src/boxes/box_trait.rs
candidate_blocked_by=concrete_box_reexports_and_runtime_object_semantics
```

## Decision

```text
selected_next_seam=plugin_exec_mode_vocabulary
selected_next_task=BUILD-BOX-CORE-CONFIG-PLUGIN-EXEC-MODE-SEAM-DESIGN-001
reason=small_dependency_free_box_factory_plugin_policy_enum

rejected_seam=provider_policy_filebox_mode
rejected_reason=still_belongs_to_provider_filebox_boundary_audit_before_move

rejected_seam=box_trait_core
rejected_reason=box_trait_mixes_core_traits_with_concrete_box_reexports
```

`PluginExecMode` is the smallest continuation of the current `hakorune-box-core`
policy vocabulary. It can move without moving env readers or plugin factory
logic.

## Contract

```text
output_contract=build-box-core-config-next-seam-selection-v0

selection_only=1
behavior_changed=0
code_moved=0
selected_next_task=BUILD-BOX-CORE-CONFIG-PLUGIN-EXEC-MODE-SEAM-DESIGN-001

summary=ok
```
