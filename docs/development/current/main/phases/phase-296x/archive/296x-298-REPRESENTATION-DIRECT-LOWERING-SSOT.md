---
Status: Landed
Date: 2026-05-29
Scope: define the representation/direct-lowering authority contract after micro-helper lane closeout.
Blocker: REPRESENTATION-DIRECT-LOWERING-SSOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-297-MICRO-HELPER-LANE-CLOSEOUT-AND-REPRESENTATION-DIRECT-LOWERING-SELECTION.md
  - docs/development/current/main/design/representation-direct-lowering-ssot.md
---

# 296x-298 Representation Direct Lowering SSOT

## Purpose

Define the authority contract for the next C-like lowering lane.

This row does not implement a transform. It promotes the next optimization
question from "which helper should we fuse?" to "which representation should
the compiler use for hot `.hako` object/capsule/array operations?"

## Evidence

```text
output_contract=representation-direct-lowering-ssot-v0
input_contract=micro-helper-lane-closeout-and-representation-direct-lowering-selection-v0
workload_id=representative-object-lifecycle-small-block-v0
representation_ladder=PublicObject,ExactSlotObject,ResidentScalar,ValueAggregate,NativeDirect
runtime_helper_role=fallback_materialization_debug_proof
mirbuilder_policy_owner=semantic_ops_and_source_shape_facts_only
representation_planner_owner=RepresentationFact_and_RepresentationPlan
lowerer_policy_owner=consume_selected_plan_only
silent_fallback_allowed=0
net_helper_delta_positive_required=1
materialization_policy_required=1
first_inventory_required=1
first_inventory_candidate_0=typed_object_exact_slot_residence
first_inventory_candidate_1=result_capsule_value_aggregate
first_inventory_candidate_2=array_slot_native_direct
selected_next=representation_candidate_inventory
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_owner=representation_candidate_inventory
next_row=representation_candidate_inventory
optimization_open=0
```

The next row must compare typed-object exact-slot residence, result-capsule
ValueAggregate, and ArraySlot NativeDirect using one shared inventory contract
before selecting an implementation pilot.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_representation_direct_lowering_ssot_guard.sh
```
