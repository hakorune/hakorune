---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-PROOF-SURFACE-001
Scope: Define the proof/report surface required before an exact-AOT
  closed-world handle resolution implementation can be considered.
Related:
  - docs/development/current/main/phases/phase-296x/296x-771-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-770-MIMALLOC-CURRENT-2X-ASM-BOUNDARY-ATTRIBUTION-001.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-PROOF-SURFACE-001

## Purpose

296x-771 selected `closed_world_handle_resolution_plan` as the clean seam for
the current handle registry / typed-handle owner. This row defines the proof
surface that a later inventory or implementation row must satisfy.

This is still not an implementation row.

## Decision

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-proof-surface-v0
source_evidence=296x-771,296x-770,object-storage-plan-boundary-ssot
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h
selected_plan=closed_world_handle_resolution_plan
proof_surface_defined=1
closed_world_handle_resolution_plan_defined=1
routeplan_proof_required=1
object_storage_plan_proof_required=1
backend_consumes_route_and_storage_plans=1
backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
mirbuilder_object_management_enabled=0
runtime_object_changed=0
global_host_handle_retirement_allowed=0
global_arc_retirement_allowed=0
helper_local_fastpath_allowed=0
benchmark_name_special_case=0
helper_name_special_case=0
raw_array_layout_lowering_without_proof=0
fallback_to_generic_host_handle_required=1
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-CANDIDATE-INVENTORY-001
summary=ok
```

## Required Report Fields

Any later candidate inventory or implementation row must emit these fields:

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-candidate-inventory-v0
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h
closed_world_handle_resolution_plan_defined=1

receiver_route_known=<0|1>
receiver_route_owner=RoutePlan|none
receiver_route_is_closed_world=<0|1>
receiver_route_is_plugin_or_dynamic=<0|1>
receiver_route_uses_reflection_or_by_name=<0|1>

receiver_storage_plan_known=<0|1>
receiver_storage_owner=ObjectStoragePlan|none
receiver_storage_is_exact=<0|1>
receiver_storage_requires_host_handle=<0|1>
receiver_handle_publication_required=<0|1>

dynamic_escape_count=<n>
plugin_or_extern_escape_count=<n>
reflection_or_by_name_route_count=<n>
host_handle_publication_count=<n>
unsupported_storage_reason_count=<n>

candidate_site_count=<n>
eligible_site_count=<n>
rejected_site_count=<n>
selected_candidate_count=<n>
selected_candidate_confidence=low|medium|high

backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
summary=ok
```

## Implementation Gate

A later implementation row may only open if an inventory row proves all of:

```text
receiver_route_known=1
receiver_route_is_closed_world=1
receiver_route_is_plugin_or_dynamic=0
receiver_route_uses_reflection_or_by_name=0
receiver_storage_plan_known=1
receiver_storage_is_exact=1
receiver_storage_requires_host_handle=0
receiver_handle_publication_required=0
dynamic_escape_count=0
plugin_or_extern_escape_count=0
reflection_or_by_name_route_count=0
host_handle_publication_count=0
eligible_site_count>=1
selected_candidate_confidence=high
```

Even then, the implementation row must keep fallback intact:

```text
fallback_to_generic_host_handle_required=1
product_default_changed=0
```

## Failure Classification

The inventory row must reject rather than silently bypass if any of these are
true:

```text
receiver route is unknown
receiver route is dynamic/plugin/reflection/by-name
receiver storage plan is missing
receiver storage still requires HostHandle
receiver can escape to plugin/extern/dynamic collection/reflection
receiver publication through HostHandle is required
backend cannot consume both RoutePlan and ObjectStoragePlan
```

## Stop Line

```text
do not implement backend direct handle bypass from this row
do not edit nyash_array_length_h from this row
do not add benchmark/helper/source-name branches
do not bypass host_handles.rs globally
do not retire HostHandle globally
do not retire Arc globally
do not move Box/Object management into MIRBuilder
do not lower raw ArrayBox layout without route and storage proof
```

## Next

```text
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-CANDIDATE-INVENTORY-001:
  build a read-only candidate inventory for the proof fields above, keep
  backend_direct_handle_bypass_enabled=0, and select no implementation unless
  the inventory proves a high-confidence candidate
```
