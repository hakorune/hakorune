---
Status: Landed
Date: 2026-05-29
Scope: select the narrow implementation owner for the selected-method ResidentScalar plan.
Blocker: TYPED-OBJECT-RESIDENT-SCALAR-IMPLEMENTATION-OWNER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-302-TYPED-OBJECT-RESIDENT-SCALAR-SELECTED-METHOD-PLAN.md
  - docs/development/current/main/design/representation-direct-lowering-ssot.md
---

# 296x-303 Typed Object Resident Scalar Implementation Owner Selection

## Purpose

Select the narrow implementation owner for the row302 selected-method
ResidentScalar plan.

This row does not implement lowering. It keeps the next code row bounded to a
small LLVM-lowering module plus a thin `field_access.py` hook.

## Evidence

```text
output_contract=typed-object-resident-scalar-implementation-owner-selection-v0
input_contract=typed-object-resident-scalar-selected-method-plan-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_plan_helper_ops=21
selected_plan_net_helper_delta=21
selected_owner=llvm_py_typed_object_resident_scalar_lowering
selected_owner_file=src/llvm_py/instructions/typed_object_resident_scalar.py
thin_hook_file=src/llvm_py/instructions/field_access.py
selected_reason=keep_resident_scalar_state_and_materialization_policy_out_of_field_access_exact_helper_route
new_env_var_required=0
activation_gate=HAKO_TYPED_OBJECT_STORE=single_thread_exact,HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1,selected_method_plan
default_emission_unchanged=1
generic_residence_rewrite=0
runtime_helper_abi_unchanged=1
mirbuilder_change_required=0
hako_source_change_required=0
selected_next=typed_object_resident_scalar_lowering_pilot
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
selected_owner=llvm_py_typed_object_resident_scalar_lowering
selected_next=typed_object_resident_scalar_lowering_pilot
implementation_open=0
```

The next row may add:

```text
src/llvm_py/instructions/typed_object_resident_scalar.py
```

The existing field lowering may only call into this owner through a small hook
when all gates hold:

```text
HAKO_TYPED_OBJECT_STORE=single_thread_exact
HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER=1
current_function == HakoAllocPageModel.acquire_usize/1
field key appears in the row302 selected plan
```

Do not add a new environment variable. The exact-slot diagnostic lane already
keeps default emission closed.

## Rejected

```text
field_access_py_owns_resident_state:
  rejected because field_access.py already owns exact helper dispatch and would
  become the representation planner.

mirbuilder_change:
  rejected for this pilot because the row302 plan can be consumed in LLVM
  lowering without changing MIR semantics.

runtime_helper_change:
  rejected because helpers are fallback/materialization surface only.

generic_residence_rewrite:
  rejected because prior residence attempts produced zero-net or no-material
  results; this pilot is selected-method only.
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_typed_object_resident_scalar_implementation_owner_selection_guard.sh
```
