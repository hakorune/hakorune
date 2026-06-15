---
Status: Landed
Date: 2026-06-15
Task: EXACT-OBJECT-PILOT-001
Scope: Implement one exact-object pilot from the shadow report without changing
  product default runtime behavior.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-712-EXACT-OBJECT-PLAN-SHADOW-001.md
  - src/object_storage_plan.rs
---

# EXACT-OBJECT-PILOT-001

## Purpose

Use the shadow-selected primitive-only object as the first exact-object pilot.
The pilot must consume RoutePlan/ObjectStoragePlan evidence and must not move
Box management into MIRBuilder.

Selected candidate:

```text
pilot_candidate=HakoAllocObjectLifecycleAlignmentResult
selected_pilot_confidence=medium
```

## Preflight Note

The candidate object is primitive-only, but the object-lifecycle facade stores it
through the `HakoAllocObjectLifecycleFacade.alignment_result` handle field. The
pilot must therefore prove the publication boundary before enabling lowering.

```text
observed_publication_boundary=Facade.alignment_result handle field
do_not_assume_stack_object_from_primitive_fields_only=1
```

If this boundary cannot be removed through a closed-world ObjectStoragePlan, the
row should close as `summary=blocked` and select a narrower nested-object storage
planning row instead of adding backend special cases.

## Required Output

```text
output_contract=hako-exact-object-pilot-v0
source_evidence=296x-712
target_front=object_lifecycle_body
pilot_candidate=HakoAllocObjectLifecycleAlignmentResult
object_storage_plan_execution_enabled=<0|1>
pilot_exact_object_enabled=<0|1>
closed_world_plan_required=1
mirbuilder_object_management_enabled=0
mirbuilder_special_case_count=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
source_hako_changed=0
runtime_object_changed=0
fallback_to_generic_box_supported=1
summary=<ok|blocked>
```

## Stop Line

```text
do not implement repo-wide Arc retirement
do not remove HostHandle globally
do not add benchmark-name branches
do not add helper-name branches
do not change product default runtime behavior
do not move object representation decisions into MIRBuilder
```

## Acceptance

```text
the pilot is guarded by exact-AOT closed-world evidence
generic object fallback remains available
product default route remains unchanged
new report fields show no MIRBuilder object-management truth
```

## Result

```text
output_contract=hako-exact-object-pilot-v0
source_evidence=296x-712
target_front=object_lifecycle_body
pilot_candidate=HakoAllocObjectLifecycleAlignmentResult
object_storage_plan_execution_enabled=0
pilot_exact_object_enabled=0
closed_world_plan_required=1
mirbuilder_object_management_enabled=0
mirbuilder_special_case_count=0
benchmark_name_branch_count=0
helper_name_branch_count=0
observed_publication_boundary=Facade.alignment_result_handle_field
publication_boundary_count=8
facade_alignment_result_set_count=1
facade_alignment_result_get_count=7
candidate_method_call_count=8
candidate_birth_call_count=1
product_default_changed=0
source_hako_changed=0
runtime_object_changed=0
fallback_to_generic_box_supported=1
selected_next=EXACT-OBJECT-NESTED-PUBLICATION-PLAN-001
summary=blocked
```

Evidence:

```text
tool=tools/allocator/hako_exact_object_pilot_preflight.py
report=/tmp/hakorune_713_exact_object_pilot_preflight.report
source_mir_json=/tmp/hakorune_709_post_array_len_owner.1781488363.report.artifacts.d/app.mir.json
```

Decision:

```text
implementation_allowed=0
reason=published nested object through Facade.alignment_result handle field
next_task=EXACT-OBJECT-NESTED-PUBLICATION-PLAN-001
```
