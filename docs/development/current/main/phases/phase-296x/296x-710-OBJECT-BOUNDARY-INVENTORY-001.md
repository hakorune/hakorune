---
Status: Planned
Date: 2026-06-15
Task: OBJECT-BOUNDARY-INVENTORY-001
Scope: Inventory Arc / HostHandle / runtime helper / Box method / dynamic route
  boundaries in the object-lifecycle body before any ObjectStoragePlan
  implementation.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/design/box-callable-registry-ssot.md
  - docs/development/current/main/design/box-object-model-replacement-map-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-709-MIMALLOC-POST-ARRAY-LEN-HELPER-OWNER-REFRESH-001.md
---

# OBJECT-BOUNDARY-INVENTORY-001

## Purpose

Classify the remaining gap after array-length helper narrowing without moving
Box management into MIRBuilder.

This is an inventory row. It does not implement ObjectStoragePlan and does not
change runtime object representation.

## Required Output

```text
output_contract=hako-object-boundary-inventory-v0
source_evidence=296x-709
target_front=object_lifecycle_body
mirbuilder_object_management_enabled=0
box_callable_registry_is_callable_truth=1
routeplan_is_call_execution_truth=1
object_storage_plan_is_representation_truth=1
arc_dynbox_boundary_count=<n>
host_handle_boundary_count=<n>
runtime_helper_boundary_count=<n>
dynamic_box_method_route_count=<n>
box_callable_routeplan_dynamic_count=<n>
closed_world_direct_method_candidate_count=<n>
exact_stack_object_candidate_count=<n>
exact_native_struct_candidate_count=<n>
scalarized_object_candidate_count=<n>
object_escape_count=<n>
plugin_or_extern_escape_count=<n>
array_or_map_escape_count=<n>
return_escape_count=<n>
selected_object_boundary_owner=<owner|none>
selected_owner_confidence=<low|medium|high>
implementation_started=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
summary=ok
```

## Stop Line

```text
do not change MIRBuilder
do not implement ObjectStoragePlan
do not retire Arc
do not remove HostHandle
do not change product defaults
do not add helper-name or benchmark-name branches
do not treat Type ABI or hako_check as execution truth
```

## Next Task If Owner Is Clear

```text
OBJECT-STORAGE-PLAN-SSOT-001:
  code-facing vocabulary / guard surface for ObjectStoragePlan

EXACT-OBJECT-PLAN-SHADOW-001:
  shadow exact-object candidate report with no execution change
```
