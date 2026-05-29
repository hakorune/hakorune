---
Status: Landed
Date: 2026-05-29
Scope: compare representation/direct-lowering candidates with one shared positive-net inventory contract.
Blocker: REPRESENTATION-CANDIDATE-INVENTORY-296X-001
Related:
  - docs/development/current/main/design/representation-direct-lowering-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-298-REPRESENTATION-DIRECT-LOWERING-SSOT.md
  - docs/development/current/main/phases/phase-296x/296x-218-MIR-TYPED-FIELD-DIRECT-OP-NET-INVENTORY.md
  - docs/development/current/main/phases/phase-296x/296x-280-CAPSULE-VALUE-RESULT-CALLER-REGION-INVENTORY.md
  - docs/development/current/main/phases/phase-296x/296x-208-MIR-ARRAY-SLOT-RESIDENCE-INVENTORY.md
---

# 296x-299 Representation Candidate Inventory

## Purpose

Compare the first representation/direct-lowering candidates with one shared
inventory vocabulary before selecting an implementation pilot.

This row does not implement a transform. It normalizes already-landed evidence
from typed-object exact slots, result capsules, and ArraySlot direct lowering.

## Evidence

```text
output_contract=representation-candidate-inventory-v0
input_contract=representation-direct-lowering-ssot-v0
workload_id=representative-object-lifecycle-small-block-v0
candidate_count=3
positive_net_candidate_count=2
top_positive_net_candidate=typed_object_exact_slot_residence
top_positive_net_delta=80
lowest_risk_positive_net_candidate=array_slot_native_direct
lowest_risk_positive_net_delta=1
candidate_0_family=typed_object_exact_slot_residence
candidate_0_current_representation=ExactSlotObject
candidate_0_candidate_representation=ResidentScalar
candidate_0_hot_pct=50.97
candidate_0_helper_ops_before=80
candidate_0_helper_ops_erased=80
candidate_0_materialization_ops_added=0
candidate_0_net_helper_delta=80
candidate_0_net_helper_delta_positive=1
candidate_0_escape_barrier_count=0
candidate_0_observer_barrier_count=16
candidate_0_unknown_call_barrier_count=7
candidate_0_storage_or_slot_proven=1
candidate_0_implementation_risk=high
candidate_0_risk_reason=largest_positive_net_but_prior_selected_method_residence_and_direct_op_attempts_hit_representation_boundaries
candidate_0_selected_as_first_pilot=0
candidate_1_family=result_capsule_value_aggregate
candidate_1_current_representation=ExactSlotObject
candidate_1_candidate_representation=ValueAggregate
candidate_1_hot_pct=4.78
candidate_1_helper_ops_before=14
candidate_1_helper_ops_erased=14
candidate_1_materialization_ops_added=14
candidate_1_net_helper_delta=0
candidate_1_net_helper_delta_positive=0
candidate_1_escape_barrier_count=0
candidate_1_observer_barrier_count=3
candidate_1_unknown_call_barrier_count=0
candidate_1_storage_or_slot_proven=1
candidate_1_implementation_risk=medium
candidate_1_risk_reason=value_aggregate_contract_clean_but_current_record_success_region_materializes_at_public_return
candidate_1_selected_as_first_pilot=0
candidate_2_family=array_slot_native_direct
candidate_2_current_representation=ExactSlotObject
candidate_2_candidate_representation=NativeDirect
candidate_2_hot_pct=38.99
candidate_2_helper_ops_before=2
candidate_2_helper_ops_erased=2
candidate_2_materialization_ops_added=1
candidate_2_net_helper_delta=1
candidate_2_net_helper_delta_positive=1
candidate_2_escape_barrier_count=0
candidate_2_observer_barrier_count=0
candidate_2_unknown_call_barrier_count=1
candidate_2_storage_or_slot_proven=1
candidate_2_implementation_risk=low
candidate_2_risk_reason=small_positive_net_region_and_direct_op_pipeline_already_proved_as_selected_method_keeper
candidate_2_selected_as_first_pilot=0
first_pilot_selection_required=1
selected_next=first_representation_pilot_selection
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
selected_next=first_representation_pilot_selection
implementation_open=0
optimization_open=0
```

Do not implement directly from this row. The next row must choose one first
representation pilot explicitly:

```text
typed_object_exact_slot_residence:
  largest positive net and largest hot owner, but high implementation risk.

array_slot_native_direct:
  lowest-risk positive net and already-proved direct-op pipeline, but small net.

result_capsule_value_aggregate:
  clean ValueAggregate model, but current recordSuccess regions are net zero
  because public method returns force materialization.
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_representation_candidate_inventory_guard.sh
```
