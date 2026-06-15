---
Status: Landed / Blocked
Date: 2026-06-15
Task: EXACT-OBJECT-FLATTENED-NESTED-FIELD-BACKEND-REACHABILITY-001
Scope: Make the enabled flattened nested field route observable in generated
  exact-AOT artifacts for the selected object-lifecycle front.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-725-EXACT-OBJECT-PILOT-EFFECT-ATTRIBUTION-001.md
---

# EXACT-OBJECT-FLATTENED-NESTED-FIELD-BACKEND-REACHABILITY-001

## Purpose

`EXACT-OBJECT-PILOT-EFFECT-ATTRIBUTION-001` showed that the enabled pilot does
not reach the generated exact-AOT hot path:

```text
flattened_nested_route_reached=0
generated_ir_contains_synthetic_nested_fields=0
alignment_result_last_requested_count=0
alignment_result_last_normalized_count=0
alignment_result_last_reason_count=0
alignment_result_last_supported_count=0
selected_owner=backend_route_reachability
```

This row fixes only the backend reachability seam for the selected flattened
nested field plan.

## Current Finding

The first reachability attempt updated the Python `src/llvm_py` field/method
seams, but the measured exact-EXE route uses `ny-llvmc`'s default boundary
driver, not the Python harness route.

```text
python_llvmlite_route_updated=1
measured_exact_exe_driver=ny_llvmc_boundary
python_route_is_measurement_owner=0
boundary_driver_flattened_nested_consumer=0
selected_owner=ny_llvmc_boundary_driver_reachability
```

Do not keep extending the Python seam as the measurement fix.  The next code
change for this row must either add a boundary-driver consumer for the selected
flattened nested field plan or prove that the exact-EXE runner should opt into
the Python harness for this diagnostic lane.

## Result

The measured boundary route is the C ABI shim route:

```text
ny_llvmc_driver=Boundary
boundary_driver=crates/nyash-llvm-compiler/src/boundary_driver_ffi.rs
boundary_compile_symbol=hako_llvmc_compile_json_pure_first
boundary_lowering_owner=lang/c-abi/shims/hako_llvmc_ffi*.inc
```

The shim already has typed-object field get/set and user-box method consumers,
but the selected flattened nested field representation is not present in MIR
JSON as an ObjectStoragePlan / flattened-nested metadata surface:

```text
mir_json_object_storage_plan_count=0
mir_json_flattened_nested_plan_count=0
typed_object_plan_has_facade_alignment_result_handle=1
typed_object_plan_has_alignment_result_primitive_fields=1
boundary_driver_has_input_plan_for_flattened_nested_fields=0
```

Therefore a boundary-driver lowering patch would currently have to infer the
selected representation from Box names / field names / method names.  That is
not allowed in this lane.

```text
backend_reachability_fixed=0
boundary_driver_flattened_nested_consumer=0
missing_owner=object_storage_plan_mir_json_export
selected_next=EXACT-OBJECT-FLATTENED-NESTED-FIELD-PLAN-EXPORT-001
summary=blocked
```

This is a clean block inside the card, not a reason to stop the overall
EXACT-OBJECT-PILOT goal.  The next row must publish the selected
ObjectStoragePlan into MIR JSON before the C ABI shim consumes it.

## Design Pin

This row follows the ObjectStoragePlan boundary SSOT:

```text
MIRBuilder:
  records object meaning only
  does not manage Box representation

BoxCallableRegistry:
  owns callable identity

RoutePlan:
  owns execution route

ObjectStoragePlan:
  owns representation route

exact-AOT backend:
  consumes RoutePlan + ObjectStoragePlan
```

The reachability fix must live in the measured backend route.  It must not turn
MIRBuilder, Type ABI, hako_check, benchmark names, helper names, or source file
names into execution truth.

```text
mirbuilder_object_management_enabled=0
type_abi_execution_truth=0
hako_check_execution_truth=0
benchmark_name_branch_count=0
helper_name_branch_count=0
source_file_name_branch_count=0
```

If the selected fix is a diagnostic route switch to the Python harness, it must
be explicit:

```text
exact_object_pilot_route=python_llvmlite_diagnostic
route_switch_explicit=1
product_default_changed=0
winner_claim=0
```

## Required Output

```text
output_contract=hako-exact-object-flattened-nested-field-backend-reachability-v0
source_evidence=296x-725
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
backend_reachability_fixed=<0|1>
generated_ir_contains_synthetic_nested_fields=<0|1>
alignment_result_last_requested_count=<n>
alignment_result_last_normalized_count=<n>
alignment_result_last_reason_count=<n>
alignment_result_last_supported_count=<n>
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
selected_next=<task>
summary=<ok|blocked>
```

## Task List

```text
1. Locate why the enabled route is not reached
   - inspect field_access.py receiver-box evidence for
     HakoAllocObjectLifecycleFacade.alignment_result
   - inspect method_call.py handling of the flattened nested view
   - inspect whether resolver/vmap carries the owner box type at that point
   - confirm which `ny-llvmc` driver the measured exact-EXE route uses

2. Add the narrow backend reachability fix
   - keep the change in the exact-AOT backend consumer used by the measured
     exact-EXE route
   - do not move object management into MIRBuilder
   - do not branch by benchmark name, helper name, or source file name
   - do not continue patching Python llvmlite seams unless the row explicitly
     switches to a diagnostic Python route
   - blocked for this row because the measured boundary route has no
     ObjectStoragePlan metadata to consume

3. Prove reachability
   - preserve generated artifacts
   - require at least one synthetic owner field evidence entry:
     alignment_result.last_requested
     alignment_result.last_normalized
     alignment_result.last_reason
     alignment_result.last_supported

4. Defer performance claim
   - if reachability is fixed, re-run pilot measurement in the next row
   - do not claim product NyRT default or global Arc retirement
```

## Stop Line

```text
do not change MIRBuilder behavior
do not change product runtime object representation
do not add benchmark/helper-name branches
do not generalize this to global Arc retirement
do not claim body-time win in this row
do not add C shim name inference while ObjectStoragePlan metadata is absent
```
