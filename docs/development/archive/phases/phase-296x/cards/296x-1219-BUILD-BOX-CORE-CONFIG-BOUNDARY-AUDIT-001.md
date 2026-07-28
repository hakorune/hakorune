---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Audit box-core + config boundary before any crate split.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1218-BUILD-CRATE-SPLIT-NEXT-BOUNDARY-SELECTION-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
---

# BUILD-BOX-CORE-CONFIG-BOUNDARY-AUDIT-001

## Inventory

```text
box_trait_lines=307
box_factory_rs_file_count=13
box_factory_total_lines=1372
config_rs_file_count=24
config_total_lines=5002
box_core_config_total_lines=6681

config_runtime_refs_visible=1
box_factory_runtime_refs_visible=1
box_factory_provider_refs_visible=1
box_factory_runner_refs_visible=1
box_trait_concrete_box_reexports_visible=1
```

Thin dependency-free vocabulary found:

```text
candidate=box_factory_policy_vocabulary
candidate_file=src/box_factory/policy.rs
candidate_lines=32
candidate_types=FactoryPolicy,FactoryType
candidate_used_by=config_env_box_factory_flags
candidate_used_by=box_factory_registry
```

## Decision

```text
full_box_core_config_split_selected=0
selected_first_slice=box_factory_policy_vocabulary
selected_next_task=BUILD-BOX-CORE-CONFIG-SEAM-DESIGN-001
reason=FactoryPolicy_and_FactoryType_are_dependency_free_and_reduce_config_to_box_factory_coupling
```

Direct extraction is rejected for this row:

```text
reject_full_config_split=1
reject_reason=config_reads_runtime_ring0_and_gc_mode

reject_full_box_factory_split=1
reject_reason=box_factory_reads_runtime_providers_runner_and_config

reject_box_trait_core_split=1
reject_reason=box_trait_reexports_concrete_box_types
```

## Contract

```text
output_contract=build-box-core-config-boundary-audit-v0

audit_only=1
behavior_changed=0
code_moved=0
implementation_allowed=0
selected_next_task=BUILD-BOX-CORE-CONFIG-SEAM-DESIGN-001

summary=ok
```

## Next

```text
next_task=BUILD-BOX-CORE-CONFIG-SEAM-DESIGN-001
purpose=design the first passive box-core crate seam around FactoryPolicy and FactoryType
```
