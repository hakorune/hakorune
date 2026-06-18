---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Move first passive box-core policy vocabulary to hakorune-box-core.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1220-BUILD-BOX-CORE-CONFIG-SEAM-DESIGN-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
---

# BUILD-BOX-CORE-CONFIG-PASSIVE-SPLIT-001

## Change

```text
new_crate=hakorune-box-core
new_crate_path=crates/hakorune_box_core
new_crate_scope=box_factory_policy_vocabulary
passive_owner_types=FactoryPolicy,FactoryType
main_crate_dependency_added=1
main_crate_facade=src/box_factory/policy.rs
historical_import_path_preserved=crate::box_factory::{FactoryPolicy,FactoryType}
behavior_changed=0
```

The first passive split intentionally moves only dependency-free policy data.
Active factory registry logic, concrete Box construction, config env helpers,
`NyashBox`, `BoxCore`, `BoxBase`, and concrete Box re-exports stay in the main
crate.

## Verification

```text
cargo_test_hakorune_box_core=green
cargo_check_default=green
current_state_pointer_guard=green
diff_check=green
```

## Contract

```text
output_contract=build-box-core-config-passive-split-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
new_crate_depends_on_main_crate=0
new_crate_depends_on_runtime=0
new_crate_depends_on_concrete_boxes=0
factory_registry_owner=main_crate
config_env_owner=main_crate

summary=ok
```

## Next

```text
next_task=BUILD-BOX-CORE-CONFIG-POST-SPLIT-MEASUREMENT-001
purpose=measure the cold release build after the first box-core passive split
implementation_allowed=0
measurement_allowed=1
```
