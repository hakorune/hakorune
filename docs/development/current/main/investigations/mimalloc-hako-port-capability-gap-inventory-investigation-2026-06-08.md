---
Status: Investigation
Date: 2026-06-08
Scope: companion design note for the compact mimalloc capability-gap SSOT.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/investigations/mimalloc-hako-port-capability-gap-inventory-task-ledger-2026-06-08.md
  - docs/development/current/main/design/fastmem-layout-table-contract-v0-ssot.md
  - docs/development/current/main/design/mimalloc-replacement-front-fidelity-ssot.md
---

# Mimalloc Capability-Gap Investigation

This note keeps the detailed design surfaces that were removed from the active
SSOT so the decision doc can stay compact and restart-friendly.

## MemOp Region Plan

`fastmem` source is accepted only when MIR can represent the region with
explicit memory operations.

```text
MemOp:
  single executable MIR instruction

MemOpKind:
  dialect vocabulary

FastMemRegion:
  side-table metadata / contract truth

MemOp.region:
  carries FastMemRegionId
```

Allowed families:

```text
MemOpKind::AddrOf
MemOpKind::Add / MemOpKind::Sub
MemOpKind::LogicalShr / MemOpKind::BitAnd
MemOpKind::TableIndex
MemOpKind::FieldLoad / MemOpKind::FieldStore
future MemOpKind::TypedLoad / MemOpKind::TypedStore
future MemOpKind::AtomicCas / MemOpKind::AtomicExchange / MemOpKind::AtomicFetchAdd
```

Forbidden in `fastmem`:

```text
arbitrary raw dereference
arbitrary external call
arbitrary Box method call
allocation / safepoint
await / nowait
closure capture
metadata pointer escape
Provider ABI crossing
Type ABI hot lookup
```

## PageKey Exact Route

`PageKey` derivation uses exact address-width arithmetic.

```text
ordinary i64 >>:
  remains the current signed i64 route for this lane

fastmem MemAddr >> constant:
  logical target-usize shift
  shift count must satisfy 0 <= shift < address_width

fastmem MemAddr & constant_mask:
  exact target-usize mask

PageKeyMake:
  MemAddr -> PageKey
  consumes segment/page shift and mask facts
```

## PageMapBridge

`PageMapBridge` maps an exact page key to layout-verified page metadata.

```text
input:
  PageKey or fastmem MemAddr-derived key

output:
  PageMetaHandle / MemLayoutRef<PageMeta>

accepted v0 bridge kinds:
  flat_side_table
  two_level_segment_table
  page_base_mask
  header_backptr
```

Bridge invariants:

```text
page_map_bridge_type_abi_hot_lookup_count=0
page_map_bridge_provider_abi_hot_dispatch_count=0
fastmem_contract_runtime_lookup_count=0
fastmem_unverified_offset_load_count=0
```

## TypedPageMetaHandle

`TypedPageMetaHandle` is the layout-verified metadata capability returned by
`PageMapBridge`.

```text
PageMetaLayoutV0:
  owner_worker_id
  block_size
  free_head
  local_free_head
  remote_head
  capacity
  used
```

Stop line:

```text
TypedPageMetaHandle does not allow arbitrary offset loads.
TypedPageMetaHandle does not escape fastmem/replacement-front metadata scope.
TypedPageMetaHandle does not imply product allocator activation.
```

## Safe Capability Wrapper Plan

```text
AddressToken:
  no-escape address fact
  no dereference
  no general pointer arithmetic

PageKey:
  exact address-width shift/mask result

PageMapBridge:
  PageKey / MemAddr-derived key -> typed page metadata

PageMetaHandle:
  layout-verified PageMeta capability

AllocOwnerId:
  allocator arena owner identity

AtomicRemoteHead:
  page-local remote-free atomic head capability
```

Wrapper route invariants:

```text
safe_capability_wrapper_route=fastmem_memop_alias
safe_capability_wrapper_lowering_route=fastmem_memop_alias
safe_capability_wrapper_memop_equivalence=1
safe_capability_wrapper_rawptr_surface=0
safe_capability_wrapper_deref_surface=0
safe_capability_wrapper_escape_count=0
```

## Mimalloc Shape Coverage Score

The replacement front is not a keeper by throughput alone.

```text
mimalloc_speed_score:
  throughput interpretation only

mimalloc_shape_score:
  structural mimalloc-shape evidence

mimalloc_safety_score:
  boundary/safety evidence

mimalloc_coverage_score:
  required coverage evidence for keeper candidacy
```

Shape components:

```text
mimalloc_shape_component_page_map_bridge
mimalloc_shape_component_typed_page_meta
mimalloc_shape_component_tls_arena
mimalloc_shape_component_alloc_owner
mimalloc_shape_component_owner_check
mimalloc_shape_component_same_owner_local_free
mimalloc_shape_component_atomic_remote_head
mimalloc_shape_component_safe_wrappers
mimalloc_shape_component_no_global_lock_hot_path
mimalloc_shape_component_no_range_scan_hot_path
```

## Producer Transition

`hako_alloc` is the mimalloc `.hako` body, not a separate allocator family.
`python_template_c_bridge` remains the explicit diagnostic baseline.

Current roles:

```text
MIRBuilder:
  emit MemOp + FastMemRegion metadata only
  preserve span/region/contract identity
  do not choose producer or route

Planner:
  choose producer/route plan

Verifier:
  enforce layout, escape, and ABI boundaries

Lowering:
  consume the selected plan
```

## AllocOwnerId / TLS Arena Owner State

`AllocOwnerId` is allocator-local owner identity, distinct from OS thread id,
runtime worker id, and `.hako` task id.

```text
slot+generation representation
no escape
equality-only on hot path
zero means unowned/invalid
same/remote/unowned/stale/invalid counts stay observation-only here
```

## Source / MIR / Lowering Visibility

```text
ordinary .hako:
  no RawPtr<T>
  no broad pointer arithmetic
  no dereference syntax

fastmem source region:
  fastmem ContractName { ... }
  mem.addr
  mem.load / mem.store
  mem.atomic*
  logical address shift/mask
  verified layout field access

MIR / plan only:
  FastMemRegion metadata side table
  MirInstruction::MemOp
  MemOpKind::AddrOf
  MemOpKind::LogicalShr
  MemOpKind::BitAnd
  MemOpKind::TableIndex
  MemOpKind::FieldLoad
  MemOpKind::FieldStore
  future MemOpKind::AtomicCas
  future MemOpKind::AtomicExchange
  future MemOpKind::AtomicFetchAdd
  AddressToken
  PageKeyMake
  LogicalShrExact
  BitMaskExact
  PageMapBridgeLookup
  AtomicRemotePush
  AtomicRemoteDrain
```

## Current Reading For Pro Consultation

The narrow question remains:

```text
What is the smallest contract-bound fast memory sublanguage that lets `.hako`
express mimalloc-style address-derived page maps and remote-free paths without
opening general unsafe pointer arithmetic or making Type ABI / Provider ABI hot?
```
