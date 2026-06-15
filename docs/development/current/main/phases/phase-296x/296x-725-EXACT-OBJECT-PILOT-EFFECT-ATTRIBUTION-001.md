---
Status: Landed
Date: 2026-06-15
Task: EXACT-OBJECT-PILOT-EFFECT-ATTRIBUTION-001
Scope: Attribute why the first guarded exact-object pilot did not improve the
  object-lifecycle body timing surface.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-723-EXACT-OBJECT-PILOT-001U.md
  - docs/development/current/main/phases/phase-296x/296x-724-EXACT-OBJECT-PILOT-MEASUREMENT-001.md
---

# EXACT-OBJECT-PILOT-EFFECT-ATTRIBUTION-001

## Purpose

`EXACT-OBJECT-PILOT-MEASUREMENT-001` measured the enabled flattened nested field
pilot and found no body-time win:

```text
body_elapsed_ratio_before=112.969
body_elapsed_ratio_after=114.326
winner_claim=0
```

This row determines whether the enabled route reached the generated exact-AOT
hot path or whether the selected nested object is not a meaningful remaining
owner.

## Result

```text
output_contract=hako-exact-object-pilot-effect-attribution-v0
source_evidence=296x-724
target_front=object_lifecycle_body
pilot_exact_object_enabled=1
flattened_nested_route_expected=1
flattened_nested_route_reached=0
generated_ir_contains_synthetic_nested_fields=0
runtime_handle_boundary_removed_for_nested_candidate=0
body_elapsed_ratio_after=114.326
artifact_dir=/tmp/row725-exact-object-pilot-effect-attribution
alignment_result_last_requested_count=0
alignment_result_last_normalized_count=0
alignment_result_last_reason_count=0
alignment_result_last_supported_count=0
facade_alignment_result_count=24
alignment_result_type_field_count=25
selected_owner=backend_route_reachability
selected_next=EXACT-OBJECT-FLATTENED-NESTED-FIELD-BACKEND-REACHABILITY-001
implementation_started=0
summary=ok
```

The enabled pilot did not improve timing because the flattened nested owner
field route is not visible in generated artifacts.  The generated MIR still
contains `HakoAllocObjectLifecycleFacade.alignment_result` and
`HakoAllocObjectLifecycleAlignmentResult.last_*` evidence, but contains no
`alignment_result.last_*` synthetic owner-field evidence.

The next row must make the backend route reach the actual generated field
access / nested method path or prove that a different generated representation
name is the correct observable route.

## Required Output

```text
output_contract=hako-exact-object-pilot-effect-attribution-v0
source_evidence=296x-724
target_front=object_lifecycle_body
pilot_exact_object_enabled=1
flattened_nested_route_expected=1
flattened_nested_route_reached=<0|1>
generated_ir_contains_synthetic_nested_fields=<0|1>
runtime_handle_boundary_removed_for_nested_candidate=<0|1>
body_elapsed_ratio_after=114.326
selected_owner=<backend_route_reachability|non_hot_candidate|measurement_noise|none>
selected_next=<task|closeout>
implementation_started=0
summary=<ok|blocked>
```

## Task List

```text
1. Build/inspect exact-AOT output for the object-lifecycle front
   - preserve generated IR/LLVM/EXE artifacts if the runner supports it
   - look for synthetic flattened nested fields:
     alignment_result.last_requested
     alignment_result.last_normalized
     alignment_result.last_reason
     alignment_result.last_supported

2. Attribute route reachability
   - if synthetic fields are absent, select a backend route-reachability row
   - if synthetic fields are present but handle/method boundaries remain,
     select the exact remaining boundary
   - if synthetic fields are present and boundaries are removed, classify this
     candidate as non-hot for the body surface and close the pilot

3. Keep implementation closed
   - this row is inspection/report only
   - do not add another ObjectStoragePlan lowering change until the owner is
     selected from generated-output evidence
```

## Stop Line

```text
do not change MIRBuilder behavior
do not add benchmark/helper-name branches
do not generalize to global Arc retirement
do not make a product NyRT default speedup claim
do not start a second object pilot before attributing this one
```
