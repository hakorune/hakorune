---
Status: Landed
Date: 2026-05-29
Scope: build the selected-method typed-object ResidentScalar plan before implementation.
Blocker: TYPED-OBJECT-RESIDENT-SCALAR-SELECTED-METHOD-PLAN-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-301-TYPED-OBJECT-RESIDENT-SCALAR-GUARD-SURFACE.md
  - docs/development/current/main/design/representation-direct-lowering-ssot.md
---

# 296x-302 Typed Object Resident Scalar Selected Method Plan

## Purpose

Build the selected-method ResidentScalar plan for
`HakoAllocPageModel.acquire_usize/1` before any transform.

This row confirms that the selected method has positive net helper-call delta
under the row301 materialization policy. It does not implement lowering.

## Evidence

```text
output_contract=typed-object-resident-scalar-selected-method-plan-v0
input_contract=typed-object-resident-scalar-guard-surface-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_method_dynamic_weight=524288
current_representation=ExactSlotObject
candidate_representation=ResidentScalar
eligible_field_get_count=13
eligible_field_set_count=8
eligible_i64_count=2
eligible_u64_count=0
eligible_usize_count=17
eligible_handle_count=2
planned_erased_helper_ops=21
planned_materialization_ops_added=0
planned_net_helper_delta=21
dynamic_planned_net_helper_delta=11010048
planned_net_helper_delta_positive=1
resident_field_key_count=11
unknown_receiver_count=0
unknown_field_plan_count=0
unsupported_storage_count=0
weak_field_count=0
barrier_unknown_call_count=1
barrier_phi_count=1
barrier_return_count=5
unknown_call_barrier_policy=materialize_or_no_plan
return_barrier_policy=materialize_only_if_net_positive
selected_plan_silent_fallback_allowed=0
storage_or_slot_proven=1
resident_field_0=HakoAllocPageModel.reject_count.usize
resident_field_0_op_count=6
resident_field_1=HakoAllocPageModel.alloc_count.usize
resident_field_1_op_count=2
resident_field_2=HakoAllocPageModel.free_top.usize
resident_field_2_op_count=2
resident_field_3=HakoAllocPageModel.peak_used.usize
resident_field_3_op_count=2
resident_field_4=HakoAllocPageModel.requested_bytes.usize
resident_field_4_op_count=2
resident_field_5=HakoAllocPageModel.used.usize
resident_field_5_op_count=2
resident_field_6=HakoAllocPageModel.block_size.usize
resident_field_6_op_count=1
resident_field_7=HakoAllocPageModel.block_used.handle
resident_field_7_op_count=1
resident_field_8=HakoAllocPageModel.decommitted.i64
resident_field_8_op_count=1
resident_field_9=HakoAllocPageModel.free.handle
resident_field_9_op_count=1
resident_field_10=HakoAllocPageModel.retired.i64
resident_field_10_op_count=1
selected_next=typed_object_resident_scalar_implementation_owner_selection
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
selected_next=typed_object_resident_scalar_implementation_owner_selection
implementation_open=0
```

The selected plan is positive-net and has no unknown receiver, unknown field
plan, unsupported storage, or weak field. The next row must select the narrow
implementation owner. It must not start a generic residence rewrite.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_typed_object_resident_scalar_selected_method_plan_guard.sh
```
