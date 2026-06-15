---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-CANDIDATE-INVENTORY-001
Scope: Read-only candidate inventory for the closed-world handle resolution
  plan proof surface.
Related:
  - docs/development/current/main/phases/phase-296x/296x-772-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-PROOF-SURFACE-001.md
  - docs/development/current/main/phases/phase-296x/296x-706-MIMALLOC-DIRECT-ARRAY-LENGTH-BOUNDARY-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-712-EXACT-OBJECT-PLAN-SHADOW-001.md
  - src/object_storage_plan.rs
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-CANDIDATE-INVENTORY-001

## Purpose

296x-772 defined the proof surface required before a closed-world handle
resolution implementation can open. This row fills that surface from current
evidence.

This is an inventory row. It does not implement backend bypass.

## Evidence

The target route is semantically known from 296x-706:

```text
route_id=generic_method.len
route_kind=array_slot_len
box_name=ArrayBox
method=length
receiver_origin_box=ArrayBox
helper_symbol=nyash.array.slot_len_h
return_shape=scalar_i64
value_demand=scalar_i64
publication_policy=no_publication
```

However, the object storage shadow evidence does not contain a receiver
storage plan for this Array receiver. Existing ObjectStoragePlan vocabulary can
represent exact objects and HostHandle escapes, but the selected exact-object
pilot targeted `HakoAllocObjectLifecycleAlignmentResult`, not the hot Array
receiver:

```text
selected_pilot_candidate=HakoAllocObjectLifecycleAlignmentResult
selected_pilot_confidence=medium
```

Therefore the current candidate fails the implementation gate on storage proof.

## Result

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-candidate-inventory-v0
source_evidence=296x-772,296x-706,296x-712
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h
closed_world_handle_resolution_plan_defined=1

receiver_route_known=1
receiver_route_owner=RoutePlan
receiver_route_is_closed_world=1
receiver_route_is_plugin_or_dynamic=0
receiver_route_uses_reflection_or_by_name=0

receiver_storage_plan_known=0
receiver_storage_owner=none
receiver_storage_is_exact=0
receiver_storage_requires_host_handle=1
receiver_handle_publication_required=1

dynamic_escape_count=0
plugin_or_extern_escape_count=0
reflection_or_by_name_route_count=0
host_handle_publication_count=1
unsupported_storage_reason_count=1

candidate_site_count=1
eligible_site_count=0
rejected_site_count=1
selected_candidate_count=0
selected_candidate_confidence=low

backend_direct_handle_bypass_enabled=0
implementation_allowed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
benchmark_name_special_case=0
helper_name_special_case=0
fallback_to_generic_host_handle_required=1
selected_blocker=receiver_storage_plan_missing
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-DESIGN-001
summary=ok
```

## Decision

Do not implement closed-world handle bypass yet.

The route side is strong enough:

```text
receiver_route_known=1
receiver_route_is_closed_world=1
```

The storage side is not:

```text
receiver_storage_plan_known=0
receiver_storage_is_exact=0
receiver_storage_requires_host_handle=1
```

The next row must design how the hot Array receiver can get an ObjectStoragePlan
proof without hardcoding `nyash_array_length_h`, without treating ArrayBox raw
layout as backend truth, and without changing product runtime behavior.

## Stop Line

```text
do not implement backend direct handle bypass from this inventory
do not edit nyash_array_length_h from this inventory
do not treat route proof alone as storage proof
do not lower raw ArrayBox layout without ObjectStoragePlan proof
do not bypass host_handles.rs globally
do not retire HostHandle globally
do not retire Arc globally
do not move Box/Object management into MIRBuilder
```

## Next

```text
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-DESIGN-001:
  design a narrow ObjectStoragePlan proof for the hot Array receiver while
  keeping backend direct handle bypass disabled and product fallback intact
```
