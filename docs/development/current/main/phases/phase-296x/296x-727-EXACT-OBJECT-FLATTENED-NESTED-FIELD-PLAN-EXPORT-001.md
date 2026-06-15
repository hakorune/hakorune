---
Status: Landed
Date: 2026-06-15
Task: EXACT-OBJECT-FLATTENED-NESTED-FIELD-PLAN-EXPORT-001
Scope: Publish the selected flattened nested ObjectStoragePlan into MIR JSON so
  the measured ny-llvmc boundary route has a plan to consume.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-726-EXACT-OBJECT-FLATTENED-NESTED-FIELD-BACKEND-REACHABILITY-001.md
---

# EXACT-OBJECT-FLATTENED-NESTED-FIELD-PLAN-EXPORT-001

## Purpose

`EXACT-OBJECT-FLATTENED-NESTED-FIELD-BACKEND-REACHABILITY-001` found that the
measured exact-EXE route goes through `ny-llvmc`'s boundary C ABI shim, not the
Python llvmlite backend.  The boundary shim has typed-object and user-box method
consumers, but it has no ObjectStoragePlan metadata to consume:

```text
mir_json_object_storage_plan_count=0
mir_json_flattened_nested_plan_count=0
boundary_driver_has_input_plan_for_flattened_nested_fields=0
```

This row publishes the selected flattened nested field representation as a
read-only MIR JSON plan.  It does not lower the plan yet.

## Decision

The exported plan is representation metadata for exact-AOT backend consumers.
It is not MIRBuilder truth and not product runtime behavior.

```text
mirbuilder_object_management_enabled=0
object_storage_plan_mir_json_export_enabled=1
object_storage_plan_execution_enabled=0
boundary_driver_flattened_nested_consumer=0
product_default_changed=0
```

The selected plan describes:

```text
owner_box=HakoAllocObjectLifecycleFacade
owner_field=alignment_result
nested_box=HakoAllocObjectLifecycleAlignmentResult
representation=flattened_nested_fields
fields:
  alignment_result.last_requested
  alignment_result.last_normalized
  alignment_result.last_reason
  alignment_result.last_supported
```

## Required Output

```text
output_contract=hako-exact-object-flattened-nested-field-plan-export-v0
source_evidence=296x-726
target_front=object_lifecycle_body
object_storage_plan_mir_json_export_enabled=1
flattened_nested_plan_count=<n>
flattened_nested_field_count=<n>
owner_box=HakoAllocObjectLifecycleFacade
owner_field=alignment_result
nested_box=HakoAllocObjectLifecycleAlignmentResult
alignment_result_last_requested_exported=<0|1>
alignment_result_last_normalized_exported=<0|1>
alignment_result_last_reason_exported=<0|1>
alignment_result_last_supported_exported=<0|1>
backend_lowering_enabled=0
boundary_driver_flattened_nested_consumer=0
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
selected_next=EXACT-OBJECT-FLATTENED-NESTED-FIELD-BOUNDARY-CONSUMER-001
summary=ok
```

## Task List

```text
1. Define the MIR JSON shape
   - root-level or metadata-level `object_storage_plans`
   - include owner box/field, nested box, representation kind, field list,
     fallback policy, and source evidence
   - no backend lowering in this row

2. Export the selected plan
   - use the existing ObjectStoragePlan vocabulary
   - do not infer from benchmark names in the backend
   - keep the export narrow to the selected closed-world front

3. Add a read-only adapter / guard
   - prove the MIR JSON contains exactly the selected flattened nested plan
   - prove all four synthetic field names are present
   - prove boundary consumer remains disabled

4. Select the next consumer row
   - only after plan export is visible, add the C ABI boundary consumer
   - do not remeasure performance in this row
```

## Implementation Breakdown

Use this order.  Each step is docs/report/metadata only; no backend lowering is
enabled in this row.

```text
727A. Inspect current MIR JSON emit owner
  - owner: src/runner/mir_json_emit
  - confirm typed_object_plans are already exported
  - confirm object_storage_plans are absent before the change

727B. Add ObjectStoragePlan MIR JSON export shape
  - root field: object_storage_plans
  - representation: flattened_nested_fields
  - include owner_box / owner_field / nested_box
  - include flattened primitive field list
  - include source_evidence=296x-726
  - include backend_lowering_enabled=0

727C. Export only the selected proven plan
  - derive from existing typed-object plan metadata
  - require Facade.alignment_result to point to AlignmentResult
  - require the four primitive nested fields to be present
  - emit no plan when proof is missing

727D. Add read-only guard surface
  - assert object_storage_plan_mir_json_export_enabled=1
  - assert flattened_nested_plan_count=1
  - assert flattened_nested_field_count=4
  - assert boundary_driver_flattened_nested_consumer=0

727E. Update current docs after guard proof
  - mark this card landed only after guard output is green
  - select EXACT-OBJECT-FLATTENED-NESTED-FIELD-BOUNDARY-CONSUMER-001
```

The implementation must not create a hidden source-name or benchmark-name
branch.  The selected names are plan keys, and the export is valid only because
the typed-object plans already prove the owner/nested relationship.

## Acceptance

```text
object_storage_plan_mir_json_export_enabled=1
flattened_nested_plan_count=1
flattened_nested_field_count=4
alignment_result_last_requested_exported=1
alignment_result_last_normalized_exported=1
alignment_result_last_reason_exported=1
alignment_result_last_supported_exported=1
backend_lowering_enabled=0
boundary_driver_flattened_nested_consumer=0
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
summary=ok
```

## Result

```text
output_contract=hako-exact-object-flattened-nested-field-plan-export-v0
source_evidence=296x-726
target_front=object_lifecycle_body
object_storage_plan_mir_json_export_enabled=1
flattened_nested_plan_count=1
flattened_nested_field_count=4
owner_box=HakoAllocObjectLifecycleFacade
owner_field=alignment_result
nested_box=HakoAllocObjectLifecycleAlignmentResult
alignment_result_last_requested_exported=1
alignment_result_last_normalized_exported=1
alignment_result_last_reason_exported=1
alignment_result_last_supported_exported=1
backend_lowering_enabled=0
boundary_driver_flattened_nested_consumer=0
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
selected_next=EXACT-OBJECT-FLATTENED-NESTED-FIELD-BOUNDARY-CONSUMER-001
summary=ok
```

Proof:

```text
cargo test -q collect_object_storage_plan_values --lib
cargo test -q build_mir_json_root_includes_object_storage_plans_surface --lib
bash tools/checks/k2_wide_phase296x_exact_object_flattened_nested_field_plan_export_guard.sh
```

## Stop Line

```text
do not lower flattened nested fields in this row
do not change MIRBuilder object management
do not change product runtime object representation
do not add C ABI shim name inference without plan metadata
do not claim body-time win
do not claim Arc retirement
```
