---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Close current box-core/config passive seam burst.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1222-BUILD-BOX-CORE-CONFIG-POST-SPLIT-MEASUREMENT-001.md
  - docs/development/current/main/phases/phase-296x/296x-1226-BUILD-BOX-CORE-CONFIG-PLUGIN-EXEC-MODE-POST-SPLIT-MEASUREMENT-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
---

# BUILD-BOX-CORE-CONFIG-CLOSEOUT-001

## Result

The current box-core/config passive seam burst is closed:

```text
new_crate=hakorune-box-core
moved_owner_type=FactoryPolicy
moved_owner_type=FactoryType
moved_owner_type=PluginExecMode
main_crate_facade_count=2
behavior_changed=0
```

Preserved compatibility surfaces:

```text
crate::box_factory::{FactoryPolicy,FactoryType}
crate::config::env::PluginExecMode
```

Still intentionally not moved:

```text
NyashBox
BoxCore
BoxBase
UnifiedBoxRegistry
BoxFactory
config env readers
provider policy / FileBox mode
runtime / provider / runner logic
```

## Measurement Summary

```text
factory_policy_post_split_real_sec=150.16
plugin_exec_mode_post_split_real_sec=159.44
build_time_winner_claim=0
reason=single_run_variability_visible_and_slices_are_too_small_for_attribution
```

## Next

Return to global build crate boundary selection before another implementation:

```text
selected_next_task=BUILD-CRATE-SPLIT-NEXT-BOUNDARY-SELECTION-002
implementation_allowed=0
selection_allowed=1
```

Potential future audits:

```text
candidate=provider_policy_filebox_mode
candidate=box_trait_core_reexport_split
candidate=backend_or_frontend_active_owner_recheck
candidate=mir_builder_boundary_inventory_refresh
```

## Contract

```text
output_contract=build-box-core-config-closeout-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
current_box_core_config_burst_closed=1
implementation_allowed=0
selected_next_task=BUILD-CRATE-SPLIT-NEXT-BOUNDARY-SELECTION-002

summary=ok
```
