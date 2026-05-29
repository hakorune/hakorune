---
Status: Landed
Date: 2026-05-29
Scope: freeze the typed-object ResidentScalar guard surface before any transform.
Blocker: TYPED-OBJECT-RESIDENT-SCALAR-GUARD-SURFACE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-300-FIRST-REPRESENTATION-PILOT-SELECTION.md
  - docs/development/current/main/phases/phase-296x/296x-218-MIR-TYPED-FIELD-DIRECT-OP-NET-INVENTORY.md
  - docs/development/current/main/design/representation-direct-lowering-ssot.md
---

# 296x-301 Typed Object Resident Scalar Guard Surface

## Purpose

Freeze the first typed-object representation pilot before implementation.

This row prevents the earlier block-local residence non-keeper from repeating:
materialization/writeback is allowed only when the selected plan remains
positive-net. Silent fallback is a row failure.

## Evidence

```text
output_contract=typed-object-resident-scalar-guard-surface-v0
input_contract=first-representation-pilot-selection-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_family=typed_object_exact_slot_residence
selected_method=HakoAllocPageModel.acquire_usize/1
selected_method_reason=row218_largest_dynamic_net_helper_delta
current_representation=ExactSlotObject
candidate_representation=ResidentScalar
selected_method_helper_ops_before=21
planned_erased_helper_ops=21
planned_materialization_ops_added=0
planned_net_helper_delta=21
planned_net_helper_delta_positive=1
dynamic_planned_net_helper_delta=11010048
storage_or_slot_proven=1
unknown_call_barrier_policy=materialize_or_no_plan
observer_return_barrier_policy=materialize_only_if_net_positive
writeback_policy=forbidden_unless_positive_net_after_writeback
selected_plan_silent_fallback_allowed=0
materialization_policy_required=1
previous_block_local_residence_zero_net_guard=1
generic_typed_field_residence_retry=0
implementation_open=0
optimization_open=0
selected_next=typed_object_resident_scalar_selected_method_plan
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_method=HakoAllocPageModel.acquire_usize/1
selected_next=typed_object_resident_scalar_selected_method_plan
implementation_open=0
```

The next row must build an explicit selected-method plan for
`HakoAllocPageModel.acquire_usize/1`. It must show exactly where helper calls
are erased and where any required materialization happens. A plan that moves
helpers from field access to writeback with no positive net delta is rejected.

## Non-Goals

```text
generic typed-field residence
generic MIR CSE
exact-slot helper expansion
ArraySlot retry
result capsule ValueAggregate retry
provider activation
allocator replacement
hooks
global allocator
winner claim
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_typed_object_resident_scalar_guard_surface_guard.sh
```
