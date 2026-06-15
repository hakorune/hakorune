---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-DESIGN-001
Scope: Design the clean exact-AOT boundary for the handle registry /
  typed-handle owner selected by the current 2x asm attribution.
Related:
  - docs/development/current/main/phases/phase-296x/296x-770-MIMALLOC-CURRENT-2X-ASM-BOUNDARY-ATTRIBUTION-001.md
  - docs/development/current/main/phases/phase-296x/296x-709-MIMALLOC-POST-ARRAY-LEN-HELPER-OWNER-REFRESH-001.md
  - docs/development/current/main/phases/phase-296x/296x-731-EXACT-OBJECT-PILOT-CLOSEOUT-001.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-DESIGN-001

## Purpose

296x-770 attributed the current reproducible object-lifecycle body gap to the
handle registry / typed-handle boundary around the current hot helper:

```text
top_symbol=nyash_array_length_h
top_symbol_percent=70.71
array_length_helper_uses_borrowed_ready=1
helper_local_fastpath_already_applied=1
remaining_owner=handle_registry_typed_handle_boundary
remaining_owner_confidence=high
```

This row selects the design seam for that owner. It does not implement the
seam.

## Decision

The clean design is a closed-world handle resolution plan consumed by the
exact-AOT backend.

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-design-v0
source_evidence=296x-770,296x-709,296x-731
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h
remaining_owner=handle_registry_typed_handle_boundary
remaining_owner_confidence=high
selected_design=closed_world_handle_resolution_plan
selected_design_confidence=medium
implementation_allowed=0
helper_local_fastpath_allowed=0
global_host_handle_retirement_allowed=0
global_arc_retirement_allowed=0
mirbuilder_object_management_enabled=0
routeplan_required=1
object_storage_plan_required=1
backend_consumes_plan=1
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
benchmark_name_special_case=0
helper_name_special_case=0
raw_array_layout_lowering_without_proof=0
fallback_to_generic_host_handle_required=1
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-PROOF-SURFACE-001
summary=ok
```

## Layer Contract

```text
RoutePlan:
  proves the callable route is known and closed-world eligible
  proves the call is not plugin/dynamic/reflection owned

ObjectStoragePlan:
  proves the receiver representation can avoid generic handle lookup for this
  exact-AOT site
  proves the object is not published through a boundary requiring HostHandle

exact-AOT backend:
  consumes RoutePlan + ObjectStoragePlan
  may bypass the handle registry only when both plans prove the boundary safe

product runtime:
  keeps the generic HostHandle / Arc world
  remains the fallback for dynamic, escaped, plugin, reflection, or unsupported
  cases
```

Short form:

```text
RoutePlan proves what is called.
ObjectStoragePlan proves what the receiver is.
Backend may bypass handle lookup only when both proofs are present.
```

## Why This Is Not Another Helper Patch

`nyash_array_length_h` is the current hot symbol, but the local helper fastpath
already landed:

```text
array_length_helper_uses_borrowed_ready=1
helper_local_fastpath_already_applied=1
helper_local_fastpath_remaining=0
```

The remaining samples are dominated by the object carrier boundary, not by the
length operation itself. Another helper-name patch would hide the owner instead
of removing the boundary structurally.

## Proof Required Before Implementation

The next row must define a proof surface before any code change.

Required proof vocabulary:

```text
closed_world_handle_resolution_plan_defined=1
receiver_route_known=<0|1>
receiver_storage_plan_known=<0|1>
receiver_handle_publication_required=<0|1>
dynamic_escape_count=<n>
plugin_or_extern_escape_count=<n>
reflection_or_by_name_route_count=<n>
backend_direct_handle_bypass_enabled=0
product_default_changed=0
implementation_allowed=0
```

Only after this proof surface exists may a later implementation row select one
narrow keeper. The implementation row must still keep product fallback intact:

```text
fallback_to_generic_host_handle_required=1
```

## Rejected Designs

```text
reject: edit nyash_array_length_h again
  reason: helper-local fastpath is already applied

reject: global HostHandle retirement
  reason: product runtime still owns the generic object world

reject: global Arc retirement
  reason: per-site proof is required; global retirement is a separate object
  substrate lane

reject: MIRBuilder object management
  reason: MIRBuilder records source meaning; representation is a plan/backend
  decision

reject: raw ArrayBox layout lowering from the helper
  reason: layout/storage proof must come from ObjectStoragePlan

reject: benchmark/helper-name special case
  reason: exact-AOT proof must be route/storage based
```

## Stop Line

```text
do not change nyash_array_length_h in this row
do not add helper-name or benchmark-name compiler branches
do not bypass host_handles.rs globally
do not retire Arc globally
do not move Box/Object management into MIRBuilder
do not lower raw ArrayBox layout without RoutePlan + ObjectStoragePlan proof
do not change product NyRT defaults, provider activation, replacement, hooks,
  or global allocator
```

## Next

```text
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-PROOF-SURFACE-001:
  define the proof/report surface for a closed_world_handle_resolution_plan
  while keeping backend direct handle bypass disabled and implementation closed
```
