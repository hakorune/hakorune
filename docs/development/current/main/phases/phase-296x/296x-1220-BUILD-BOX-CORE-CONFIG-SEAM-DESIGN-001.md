---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Design first passive box-core + config seam.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1219-BUILD-BOX-CORE-CONFIG-BOUNDARY-AUDIT-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
---

# BUILD-BOX-CORE-CONFIG-SEAM-DESIGN-001

## Decision

Create a small passive crate for box-core policy vocabulary:

```text
new_crate=hakorune-box-core
new_crate_scope=box_factory_policy_vocabulary
first_owner_types=FactoryPolicy,FactoryType
main_crate_facade=src/box_factory/policy.rs
main_crate_surface_preserved=crate::box_factory::{FactoryPolicy,FactoryType}
```

This seam deliberately does not move `NyashBox`, `BoxCore`, or concrete Box
re-exports. Those still depend on concrete box modules and runtime-facing
ownership semantics.

## Allowed

```text
move_passive_policy_vocabulary=1
add_main_crate_dependency=hakorune-box-core
keep_compat_facade=1
behavior_changed=0
```

## Forbidden

```text
move_NyashBox_trait=0
move_BoxCore_trait=0
move_BoxBase=0
move_concrete_box_reexports=0
move_UnifiedBoxRegistry=0
move_config_env_helpers=0
move_runtime_or_provider_logic=0
```

## Contract

```text
output_contract=build-box-core-config-seam-design-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
selected_next_task=BUILD-BOX-CORE-CONFIG-PASSIVE-SPLIT-001

summary=ok
```

## Next

```text
next_task=BUILD-BOX-CORE-CONFIG-PASSIVE-SPLIT-001
purpose=move FactoryPolicy and FactoryType into hakorune-box-core behind the existing box_factory facade
```
