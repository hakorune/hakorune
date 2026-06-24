---
Status: Landed
Date: 2026-05-29
Scope: define direct storage substrate after selected ResidentScalar feasibility closed zero-net.
Blocker: REPRESENTATION-OWNER-REFRESH-AFTER-RESIDENCE-ZERO-NET-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-304-TYPED-OBJECT-RESIDENT-SCALAR-FEASIBILITY-CLOSEOUT.md
  - docs/development/current/main/design/representation-direct-lowering-ssot.md
---

# 296x-305 Representation Direct Storage Substrate SSOT

## Purpose

Replace the planned generic owner refresh with an authority design row.

Row304 showed that helper-backed ResidentScalar is zero-net. The next step is
to define the substrate needed for true helper-free `NativeDirect` hot regions.

## Evidence

```text
output_contract=representation-direct-storage-substrate-ssot-v0
input_contract=typed-object-resident-scalar-feasibility-closeout-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_design_owner=NativeDirectStorageSubstrate
selected_reason=resident_scalar_with_helper_load_writeback_has_net_helper_call_delta_zero
public_object_defined=1
exact_slot_object_defined=1
resident_scalar_cache_defined=1
addressable_slot_defined=1
direct_slot_lease_defined=1
materialized_local_struct_defined=1
value_aggregate_delta_defined=1
native_direct_defined=1
raw_runtime_vec_pointer_exposure_allowed=0
pinned_storage_required_for_direct_slot_lease=1
materialization_policy_required=1
escape_barrier_policy_required=1
observer_barrier_policy_required=1
net_helper_delta_positive_required=1
first_feasibility_candidate_0=typed_object_direct_slot_lease
first_feasibility_candidate_1=method_local_stack_aggregate
first_feasibility_candidate_2=array_slot_native_direct
first_feasibility_candidate_3=result_capsule_value_aggregate_region
selected_next=typed_object_direct_slot_lease_feasibility
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
selected_design_owner=NativeDirectStorageSubstrate
selected_next=typed_object_direct_slot_lease_feasibility
implementation_open=0
```

Do not retry selected-method ResidentScalar. The direct-storage substrate must
answer whether an opaque handle can become helper-free through `DirectSlotLease`
or whether a pinned storage backend / materialized local struct is required.

## SSOT

```text
docs/development/current/main/design/representation-direct-storage-substrate-ssot.md
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_representation_direct_storage_substrate_ssot_guard.sh
```
