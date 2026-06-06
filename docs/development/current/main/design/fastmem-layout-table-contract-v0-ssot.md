---
Status: Active
Date: 2026-06-06
Scope: memory-profile layout/table contract resolver for MIR-FMEM-008B.
Related:
  - docs/development/current/main/design/mir-fastmem-memop-dialect-ssot.md
  - docs/development/current/main/design/contract-region-v0-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-457-VERIFIED-MEM-ACCESS-PLAN.md
---

# FastMemory Layout/Table Contract V0

## Decision

`FastMemAccessPlan` rows move from `symbolic_only` to `verified` only through a
memory-profile contract resolver.

```text
MemOpAccess:
  symbolic ids from MIRBuilder

FastMemory layout/table contract:
  resolves ids to offsets, types, mutability, field class, table repr, stride,
  alignment, and bounds policy

VerifiedMemAccessPlan:
  lowering input
```

Lowering must not recompute or infer layout facts.

## Responsibility

```text
MIRBuilder:
  records region_id, contract_id, field_id, table_id, source span

FastMemory contract resolver:
  resolves ids
  owns field alias normalization
  owns layout/table facts
  marks plans verified or rejected

LLVM lowering:
  consumes verified rows only
```

## Memory-Specific, Not Generic

This resolver is the memory-profile payload under the future common
`ContractRegionV0` envelope.

```text
Generic:
  region id
  profile
  contract id
  obligations
  verifier/report envelope

Memory-specific:
  MemOpKind
  MemValueKind
  FastMemLayoutContractV0
  FastMemTableContractV0
  VerifiedMemAccessPlan
```

Do not rename `FastMemRegion` or replace `VerifiedMemAccessPlan` with a generic
access plan in this slice.

## PageMetaLayoutV0

Canonical field names:

```text
owner_worker_id
block_size
free_head
local_free_head
remote_head
capacity
used
```

Compatibility aliases:

```text
owner_id -> owner_worker_id
```

Aliases are verifier/contract input compatibility only. Verified plans and JSON
must carry the canonical `field_id`.

## Field Classes

```text
owner_worker_id:
  plain_scalar

block_size:
  plain_scalar

free_head:
  plain_pointer

local_free_head:
  local_free_head

remote_head:
  atomic_remote_head

capacity:
  plain_scalar

used:
  plain_scalar
```

`remote_head` may be loaded as metadata only when an explicit later row opens
AtomicRemoteHead. Plain `FieldStore(remote_head)` is rejected in this slice.

## Layout Calculation

V0 may reuse `src/mir/raw_layout.rs` for `repr_c_v0` numeric and pointer-sized
field offsets. The resolver owns the mapping from memory-profile field classes
to raw-layout scalar storage.

Stop lines:

```text
lowering_recomputed_layout_offset_count=0
unverified_offset_load_count=0
unverified_offset_store_count=0
```

## Table Contract V0

The first table id is:

```text
page_table
```

V0 facts:

```text
element_layout_id=PageMetaLayoutV0
element_repr=pointer_to_element
element_stride=target_usize
alignment=target_usize
index_policy=explicit_check
```

`length` may remain unknown until a real page-map table contract lands. Unknown
length keeps `TableIndex` non-lowerable, but it must be reported explicitly.

## Acceptance

Required positive evidence:

```text
fastmem_verified_mem_access_plan_count > 0
fastmem_verified_field_access_count > 0
fastmem_layout_id_resolved_count > 0
fastmem_field_id_resolved_count > 0
```

Required stop-line evidence:

```text
fastmem_unverified_layout_access_count=0
fastmem_unknown_alignment_count=0
fastmem_atomic_field_plain_store_count=0
fastmem_lowering_recomputed_layout_offset_count=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

LLVM GEP/load/store lowering opens only after verified rows exist and table
bounds/alignment policy is explicit.

## TableIndex Proof Decision

Decision:

```text
Main line:
  VerifiedTableAccess proof row

Optional short row:
  VerifiedElementRef-only field GEP smoke

Deferred:
  page-map strategy / two-level table product shape
```

Rule:

```text
Layout verified != Access verified
```

`PageMetaLayoutV0` proves field offsets and alignment for an element layout. It
does not prove that `page_table[index]` points to a valid element.

`TableIndex` is lowerable only with a verifier-produced `VerifiedTableAccess`
that proves:

```text
table_length_resolved
bounds_proof_valid
stride_resolved
field_offset_resolved
overflow_proof_valid
alignment_valid
element_layout_verified
```

Bounds and overflow are separate proofs. `index < len` does not by itself prove
`index * stride + field_offset` cannot overflow target `usize`.

The short LLVM smoke may lower fields only from `VerifiedElementRef`. It must
not lower `page_table[index].field`.
