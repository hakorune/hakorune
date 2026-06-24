---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-DESIGN-001
Scope: Design the ObjectStoragePlan proof required for the hot Array receiver
  before closed-world handle bypass can be implemented.
Related:
  - docs/development/current/main/phases/phase-296x/296x-773-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-CANDIDATE-INVENTORY-001.md
  - docs/development/current/main/phases/phase-296x/296x-706-MIMALLOC-DIRECT-ARRAY-LENGTH-BOUNDARY-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-373-ARRAYBOX-PUBLIC-SEMANTICS-AND-DIRECTARRAY-SPLIT-SSOT.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
---

# MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-DESIGN-001

## Purpose

296x-773 found the route proof for the hot `ArrayBox.length` site, but rejected
implementation because the receiver storage proof is missing:

```text
receiver_route_known=1
receiver_route_is_closed_world=1
receiver_storage_plan_known=0
eligible_site_count=0
selected_blocker=receiver_storage_plan_missing
```

This row decides how to create that storage proof without making public
`ArrayBox` internals a backend truth.

## Decision

The storage proof must be an Array receiver storage plan derived from
ObjectStoragePlan / ArrayRepr evidence, not from the public `ArrayBox` handle.

```text
output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-storage-proof-design-v0
source_evidence=296x-773,296x-706,296x-373,array-repr-ssot,object-storage-plan-boundary-ssot
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
target_symbol=nyash_array_length_h
selected_design=array_receiver_storage_proof_via_object_storage_plan
selected_design_confidence=medium
route_proof_available=1
storage_proof_available=0
implementation_allowed=0
backend_direct_handle_bypass_enabled=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
mirbuilder_object_management_enabled=0
public_arraybox_handle_reinterpretation_allowed=0
raw_arraybox_layout_backend_truth=0
arrayrepr_or_object_storage_plan_required=1
fallback_to_public_arraybox_host_handle_required=1
fallback_to_generic_host_handle_required=1
benchmark_name_special_case=0
helper_name_special_case=0
next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-SURFACE-001
summary=ok
```

## Storage Proof Contract

The proof must establish all of:

```text
receiver is compiler-owned or plan-owned, not merely a public HostHandle
receiver has an ObjectStoragePlan or ArrayRepr residence
receiver length can be read from that residence without public helper lookup
receiver is not published through HostHandle before the read
receiver does not cross plugin / extern / reflection / dynamic collection
fallback to public ArrayBox HostHandle remains available
```

The proof must not establish storage from any of:

```text
helper name == nyash_array_length_h
benchmark name
public ArrayBox handle reinterpretation
Rust ArrayBox internal RwLock / Vec layout
backend inference of raw ArrayBox layout
```

## Accepted Shape

The accepted shape is a future proof row with explicit fields:

```text
array_receiver_storage_proof_defined=1
array_receiver_storage_owner=ObjectStoragePlan|ArrayRepr
array_receiver_storage_residence=direct_array|exact_native_struct|scalarized|none
array_receiver_public_handle_reinterpreted=0
array_receiver_host_handle_publication_before_read=0
array_receiver_fallback_public_arraybox=1
array_receiver_backend_raw_layout_inference=0
```

The future implementation, if allowed later, may only consume these fields. It
must not inspect public `ArrayBox` internals directly.

## Rejected Designs

```text
reject: public ArrayBox handle reinterpretation
  reason: public ArrayBox remains the facade/materialized view; direct storage
  belongs to ArrayRepr / ObjectStoragePlan

reject: backend raw ArrayBox layout inference
  reason: backend cannot own Rust runtime layout truth

reject: helper-specific lowering
  reason: proof must be route/storage based, not helper-name based

reject: MIRBuilder object management
  reason: MIRBuilder records object meaning; representation belongs to plans
  and exact-AOT backend consumers

reject: global HostHandle or Arc retirement
  reason: this is a per-site closed-world proof lane
```

## Stop Line

```text
do not implement backend direct handle bypass from this row
do not edit nyash_array_length_h from this row
do not reinterpret public ArrayBox HostHandle as direct storage
do not expose Rust ArrayBox internal layout as backend truth
do not move Box/Object management into MIRBuilder
do not retire HostHandle globally
do not retire Arc globally
do not change product defaults
```

## Next

```text
MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-SURFACE-001:
  define the concrete report fields for array_receiver_storage_proof, keeping
  backend direct handle bypass disabled and product fallback intact
```
