---
Status: Landed
Date: 2026-06-15
Task: EXACT-OBJECT-FLATTENED-NESTED-FIELD-BOUNDARY-CONSUMER-001
Scope: Teach the measured ny-llvmc boundary C ABI shim to consume exported
  ObjectStoragePlan metadata for the selected flattened nested field route.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-727-EXACT-OBJECT-FLATTENED-NESTED-FIELD-PLAN-EXPORT-001.md
---

# EXACT-OBJECT-FLATTENED-NESTED-FIELD-BOUNDARY-CONSUMER-001

## Purpose

`EXACT-OBJECT-FLATTENED-NESTED-FIELD-PLAN-EXPORT-001` made the selected
flattened nested ObjectStoragePlan visible in MIR JSON:

```text
object_storage_plan_mir_json_export_enabled=1
flattened_nested_plan_count=1
flattened_nested_field_count=4
boundary_driver_flattened_nested_consumer=0
```

This row consumes that plan in the measured `ny-llvmc` boundary route.  The
consumer must read `object_storage_plans` metadata and must not infer
representation from benchmark names, helper names, source file names, or method
names.

## Decision

```text
boundary_driver_flattened_nested_consumer=1
uses_object_storage_plan_metadata=1
backend_lowering_enabled=1
mirbuilder_object_management_enabled=0
product_default_changed=0
```

The consumer target is narrow:

```text
owner_box=HakoAllocObjectLifecycleFacade
owner_field=alignment_result
nested_box=HakoAllocObjectLifecycleAlignmentResult
representation=flattened_nested_fields
```

## Design Pin

This row is a backend plan-consumer row, not a MIRBuilder object-management
row.

```text
mirbuilder_object_management_enabled=0
object_storage_plan_is_representation_truth=1
routeplan_is_call_execution_truth=1
backend_consumes_object_storage_plan=1
product_default_changed=0
```

Allowed boundary behavior:

```text
field access:
  may lower Facade.alignment_result through ObjectStoragePlan metadata

nested method call:
  may use method names only after receiver binding is proven to be the
  ObjectStoragePlan-provided flattened nested view

fallback:
  generic object route remains available when plan metadata or proof is absent
```

Forbidden boundary behavior:

```text
infer flattened representation from method names alone
infer flattened representation from benchmark/helper/source names
remove Arc / HostHandle globally
change product NyRT default
```

## Required Output

```text
output_contract=hako-exact-object-flattened-nested-field-boundary-consumer-v0
source_evidence=296x-727
target_front=object_lifecycle_body
object_storage_plan_mir_json_export_enabled=1
boundary_driver_flattened_nested_consumer=1
uses_object_storage_plan_metadata=1
flattened_nested_plan_count=1
flattened_nested_field_count=4
alignment_result_last_requested_consumed=1
alignment_result_last_normalized_consumed=1
alignment_result_last_reason_consumed=1
alignment_result_last_supported_consumed=1
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
selected_next=EXACT-OBJECT-PILOT-001V
summary=ok
```

## Task List

```text
1. Inspect boundary C ABI shim consumer surface
   - owner: lang/c-abi/shims/hako_llvmc_ffi*.inc
   - identify typed-object field access and same-module method-call plan readers

2. Add read-only ObjectStoragePlan parser / lookup
   - parse root object_storage_plans
   - find representation=flattened_nested_fields
   - expose owner field and nested field mapping to existing lowering helpers

3. Wire the selected consumer
   - field_get / field_set for Facade.alignment_result
   - direct nested method calls on AlignmentResult
   - generic fallback remains available when plan metadata is absent

4. Add guard / artifact proof
   - prove generated artifact contains flattened nested field route
   - prove no benchmark/helper/source-name branch
   - do not remeasure body time in this row
```

## Next Task Order

```text
1. Finish this row:
   field_access_lowering_connected=1
   nested_method_lowering_connected=1
   generated_artifact_reachability_proven=1

2. EXACT-OBJECT-PILOT-001V:
   retry the guarded exact-object pilot only after generated artifact
   reachability is proven.

3. EXACT-OBJECT-PILOT-MEASUREMENT-002:
   measure product exact-AOT route after reachability.  No product NyRT claim.

4. EXACT-OBJECT-PILOT-CLOSEOUT-001:
   close as per-site exact-AOT boundary win, or close without keeper if the
   route does not produce a measured win.
```

## Progress

```text
728A_boundary_metadata_reader_added=1
object_storage_plan_reader_file=lang/c-abi/shims/hako_llvmc_ffi_object_storage_plan.inc
object_storage_plan_reader_included_from=lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
uses_object_storage_plan_metadata=1
field_access_lowering_connected=1
nested_method_lowering_connected=1
generated_artifact_reachability_proven=1
summary=ok
```

## Result

```text
output_contract=hako-exact-object-flattened-nested-field-boundary-consumer-v0
source_evidence=296x-727
target_front=object_lifecycle_body
object_storage_plan_mir_json_export_enabled=1
boundary_driver_flattened_nested_consumer=1
uses_object_storage_plan_metadata=1
flattened_nested_plan_count=1
flattened_nested_field_count=4
alignment_result_last_requested_consumed=1
alignment_result_last_normalized_consumed=1
alignment_result_last_reason_consumed=1
alignment_result_last_supported_consumed=1
field_access_lowering_connected=1
nested_method_lowering_connected=1
generated_artifact_reachability_proven=1
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
fallback_to_generic_box_supported=1
backend_lowering_enabled=1
selected_next=EXACT-OBJECT-PILOT-001V
summary=ok
```

Proof:

```text
python3 -m py_compile tools/allocator/hako_exact_object_flattened_nested_field_boundary_consumer.py
tools/allocator/hako_exact_object_flattened_nested_field_boundary_consumer.py --repo-root .
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/k2_wide_phase296x_exact_object_flattened_nested_field_boundary_consumer_guard.sh
```

The reader seam now feeds the measured `ny-llvmc` boundary route.  The consumer
connects owner field access and nested method calls through ObjectStoragePlan
metadata.  The next row is `EXACT-OBJECT-PILOT-001V`.

## Stop Line

```text
do not add MIRBuilder object management
do not infer flattening from Box/method/helper/benchmark names without plan metadata
do not change product NyRT default
do not remove generic fallback
do not claim Arc retirement
do not claim body-time win
```
