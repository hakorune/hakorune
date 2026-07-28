Status: Done
Date: 2026-06-18
Scope: mir_core growth preflight for build-time crate split
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - crates/hakorune_mir_core/README.md

# BUILD-MIR-CORE-GROWTH-PREFLIGHT-001

## Purpose

Inventory dependency-free MIR data that can move into `hakorune-mir-core`
before the first real `hakorune-mir-plans` split.

## Inventory

Existing `hakorune-mir-core` ownership:

```text
BasicBlockId
BindingId
ValueId / LocalId
MirType / ConstValue / primitive ops
Effect / EffectMask
MirValueKind / TypedValueId
```

Selected first growth slice:

```text
selected_group=control_flow_id_newtypes
selected_types=LoopId,ExitEdgeId,ContinueEdgeId
reason=pure_id_newtypes
builder_dependency=0
backend_dependency=0
runtime_dependency=0
behavior_change_required=0
```

800-line file audit:

```text
large_file_threshold=800
large_file_count=0
large_file_modularization_required_for_this_slice=0
```

## Rejected For First Slice

```text
control_form_logic=too_behavioral_for_mir_core
region_observation_types=gc_observation_layer_not_core_id_slice
slot_registry_types=region_subsystem_local_for_now
join_ir_ids=joinir_split_needs_separate_owner
```

## Contract

```text
output_contract=build-mir-core-growth-preflight-v0

inventory_only=1
selected_first_growth_slice=control_flow_id_newtypes
boxshape_only=1
boxcount_allowed=0
behavior_changed=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-CORE-GROWTH-ID-SLICE-001
then=BUILD-MIR-PLANS-CRATE-PREFLIGHT-001
```
