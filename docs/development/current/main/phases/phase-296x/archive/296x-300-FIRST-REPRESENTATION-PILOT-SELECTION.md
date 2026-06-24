---
Status: Landed
Date: 2026-05-29
Scope: select the first representation/direct-lowering pilot from row299 candidate inventory.
Blocker: FIRST-REPRESENTATION-PILOT-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-299-REPRESENTATION-CANDIDATE-INVENTORY.md
  - docs/development/current/main/design/representation-direct-lowering-ssot.md
---

# 296x-300 First Representation Pilot Selection

## Purpose

Select one first representation/direct-lowering pilot from the normalized
candidate inventory.

This row does not implement the selected pilot. Because the selected candidate
is high risk, the next row must be a guard surface that fixes region boundaries,
materialization points, and fallback ownership before any transform.

## Evidence

```text
output_contract=first-representation-pilot-selection-v0
input_contract=representation-candidate-inventory-v0
workload_id=representative-object-lifecycle-small-block-v0
candidate_count=3
positive_net_candidate_count=2
selected_candidate=typed_object_exact_slot_residence
selected_current_representation=ExactSlotObject
selected_candidate_representation=ResidentScalar
selected_hot_pct=50.97
selected_helper_ops_before=80
selected_helper_ops_erased=80
selected_materialization_ops_added=0
selected_net_helper_delta=80
selected_net_helper_delta_positive=1
selected_escape_barrier_count=0
selected_observer_barrier_count=16
selected_unknown_call_barrier_count=7
selected_storage_or_slot_proven=1
selected_implementation_risk=high
selected_reason=largest_hot_owner_and_largest_positive_net_delta_requires_guard_surface_before_transform
next_row=typed_object_resident_scalar_guard_surface
guard_surface_required=1
implementation_open=0
optimization_open=0
rejected_candidate_0=array_slot_native_direct
rejected_reason_0=low_risk_but_small_net_delta_and_direct_op_pipeline_already_proved
rejected_candidate_1=result_capsule_value_aggregate
rejected_reason_1=net_zero_due_public_method_return_materialization
silent_fallback_allowed=0
materialization_policy_required=1
net_helper_delta_positive_required=1
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_candidate=typed_object_exact_slot_residence
next_row=typed_object_resident_scalar_guard_surface
implementation_open=0
```

The ArraySlot candidate is lower risk, but it has already served as the pipeline
proof and only offers a small current net delta. The result capsule candidate is
still net zero for the current recordSuccess regions. The typed-object candidate
is the only large positive-net representation candidate, so it is the right
first pilot, but it must enter through a guard surface rather than an immediate
implementation.

## Next Guard Surface

The next row must fix:

```text
target region:
  selected hot typed-object methods and slots

materialization policy:
  helper load/writeback only at known entry, escape, observer, or return points

barriers:
  unknown calls
  public observer/return boundaries
  dynamic slot/storage uncertainty

acceptance:
  positive net helper delta remains > 0
  selected plan cannot silently fall back
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_first_representation_pilot_selection_guard.sh
```
