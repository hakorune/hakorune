---
Status: Done
Date: 2026-06-06
Scope: MIR-FMEM-008B layout/table MemOps producer pilot.
Blocker: MIR-FMEM-008B
Related:
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/design/mir-fastmem-memop-dialect-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-455-PRODUCER-SLICE-SELECTION.md
---

# 296x-456 Layout/Table Access Metadata

## Purpose

`MIR-FMEM-008B` starts the smallest layout/table producer slice for the
MIR-to-LLVM replacement-front path by preserving the symbolic access truth
needed by verified lowering:

```text
TableIndex
FieldLoad
FieldStore
```

This row does not open GEP/load/store lowering yet. It must not mix
allocator-owner runtime, AtomicRemoteHead, TLS transfer, Python-template C
bridge retirement, or product activation.

## Decision

The pilot is split by responsibility:

```text
MIRBuilder:
  Preserve symbolic access ids only.
  Do not compute offsets, choose table representation, or choose routes.

Verifier / contract:
  Own layout_id / field_id / table_id / index / alignment truth.
  Reject unsupported or incomplete access metadata before lowering.

Lowering:
  Consume verified access plans only.
  Emit LLVM GEP/load/store.
  Do not infer fields, recompute offsets, or call helper fallbacks.

Planner:
  Later owner for page-map strategy and producer route selection.
```

## Selected / Deferred

Selected in this row:

```text
fastmem_selected_memops=TableIndex,FieldLoad,FieldStore
```

Explicitly deferred:

```text
CurrentAllocOwnerId
OwnerEq
AtomicRemoteHead
TLS backing transfer
owner slot reuse
Python-template C diagnostic baseline retirement
product activation
hook install
global allocator claim
winner claim
```

## Structural Precondition

The existing MIR shape carried `kind + operands`, but `FieldLoad` /
`FieldStore` lost the field name and `TableIndex` lost the table name. That
would force LLVM lowering to infer access truth.

008B therefore starts by adding symbolic access metadata to `MemOp`:

```text
FieldLoad:
  field_id required

FieldStore:
  field_id required when source target is a field access

TableIndex:
  table_id required when source target is a named table
```

Layout resolution is still contract/verifier-owned. A missing layout plan must
fail before LLVM lowering opens behavior.

## Acceptance Fields

Required evidence:

```text
mir_fmem_008b_layout_table_producer_pilot=1
fastmem_selected_memops=TableIndex,FieldLoad,FieldStore
fastmem_deferred_memops=CurrentAllocOwnerId,OwnerEq,AtomicRemoteHead

memop_table_index_lowered_count
memop_field_load_lowered_count
memop_field_store_lowered_count

memop_current_alloc_owner_id_lowered_count=0
memop_owner_eq_lowered_count=0
memop_atomic_remote_head_lowered_count=0

fastmem_field_id_missing_count=0
fastmem_table_id_missing_count=0
fastmem_unverified_layout_access_count=0
fastmem_table_index_unchecked_count=0
fastmem_unknown_alignment_count=0
fastmem_atomic_field_plain_store_count=0
fastmem_layout_ref_escape_count=0
fastmem_lowering_recomputed_layout_offset_count=0

type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Stop Line

If field/table symbolic ids are not available in MIR/JSON, stop before LLVM
lowering. Lowering must never recover access truth from operand names, source
strings, or backend helper conventions.

## Landed

```text
MemOpAccess:
  layout_id
  field_id
  table_id

MIRBuilder:
  preserves field_id for FieldLoad / FieldStore
  preserves table_id for simple named TableIndex sources

MIR JSON:
  emits layout_id / field_id / table_id when present

Verifier:
  rejects TableIndex without table_id
  rejects FieldLoad / FieldStore without field_id
  rejects access metadata on non-layout/table value MemOps

Contracts:
  OwnerEq is closed again with CurrentAllocOwnerId for MIR-FMEM-008C
```

## Next

`MIR-FMEM-008B` still needs a verified layout/table access plan before LLVM
lowering opens:

```text
VerifiedMemAccessPlan
VerifiedFieldAccess
VerifiedTableAccess
layout/field/table contract resolution
GEP/load/store lowering from verified plan only
```
