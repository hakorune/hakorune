---
Status: Landed
Date: 2026-05-29
Scope: close the selected typed-object ResidentScalar pilot after feasibility shows zero net helper delta.
Blocker: TYPED-OBJECT-RESIDENT-SCALAR-LOWERING-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-303-TYPED-OBJECT-RESIDENT-SCALAR-IMPLEMENTATION-OWNER-SELECTION.md
  - docs/development/current/main/phases/phase-296x/296x-302-TYPED-OBJECT-RESIDENT-SCALAR-SELECTED-METHOD-PLAN.md
---

# 296x-304 Typed Object Resident Scalar Feasibility Closeout

## Purpose

Close the selected typed-object ResidentScalar lowering pilot before code
changes. The row303 owner was valid as the narrow implementation owner, but
implementation feasibility shows that opaque typed-object handles still require
helper loads and writebacks for the selected method.

This row prevents repeating the earlier selected-method residence non-keeper.

## Evidence

```text
output_contract=typed-object-resident-scalar-feasibility-closeout-v0
input_contract=typed-object-resident-scalar-implementation-owner-selection-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_method=HakoAllocPageModel.acquire_usize/1
feasibility_source=cfg-aware-typed-field-residence-plan-v0
block_count=12
eligible_resident_field_count=9
scalar_field_get_count=11
scalar_field_set_count=8
erased_field_get_count=11
erased_field_set_count=8
inserted_helper_load_count=11
inserted_helper_writeback_count=8
same_block_reused_get_count=0
coalesced_writeback_count=0
net_helper_call_delta=0
net_helper_call_delta_positive=0
cross_block_field_count=3
phi_dirty_required_count=1
flush_before_return_count=8
fallback_field_count=0
rejected_handle_field_count=2
implementation_recommendation=do_not_implement_cfg_aware_residence_for_selected_method
selected_pilot_closed=1
selected_reason=opaque_handle_runtime_store_requires_load_writeback_materialization_and_erases_no_net_helpers
selected_next=representation_owner_refresh_after_residence_zero_net
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
selected_pilot_closed=1
selected_next=representation_owner_refresh_after_residence_zero_net
implementation_open=0
```

Do not implement the selected-method ResidentScalar pilot in this shape. The
current LLVM lowering only sees opaque typed-object handles; without a true
direct storage representation, helper loads and writebacks replace the erased
field helpers and produce zero net helper-call delta.

The next row must refresh representation ownership instead of trying another
selected-method residence implementation.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_typed_object_resident_scalar_feasibility_closeout_guard.sh
```
