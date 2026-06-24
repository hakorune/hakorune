---
Status: Done
Date: 2026-06-06
Scope: MIR-FMEM-008B verified layout/table access plan before LLVM GEP/load/store lowering.
Blocker: MIR-FMEM-008B
Related:
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/design/mir-fastmem-memop-dialect-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-456-LAYOUT-TABLE-PRODUCER-PILOT.md
---

# 296x-457 Verified Mem Access Plan

## Purpose

`296x-456` preserved symbolic `MemOpAccess` metadata:

```text
FieldLoad / FieldStore:
  field_id

TableIndex:
  table_id
```

That metadata is not enough for LLVM lowering. `296x-457` opens the next
`MIR-FMEM-008B` slice: create the verified plan boundary that resolves symbolic
layout/table ids before any `TableIndex` / `FieldLoad` / `FieldStore` lowering
can emit LLVM GEP/load/store.

## Decision

The lowering chain is:

```text
MemOpAccess:
  symbolic source ids only

VerifiedMemAccessPlan:
  verifier-owned lowering contract

LLVM lowering:
  consumes verified plans only
```

Short form:

```text
MIRBuilder records.
Verifier resolves and accepts.
Lowering emits.
```

## Required Shape

Add a function-local verified access-plan table:

```text
FunctionMetadata.fastmem_access_plans[]
```

Each row is keyed by the exact MemOp site:

```text
function
block
instruction_index
region_id
memop_kind
```

Field access rows must carry:

```text
layout_id
field_id
byte_offset
field_type
alignment
access = load | store
mutability
field_class
```

Table access rows must carry:

```text
table_id
element_layout_id
element_repr = inline_element | pointer_to_element
element_stride
length
alignment
index_policy = range_fact | mask_fact | explicit_check
```

## Responsibility Boundary

MIRBuilder may only emit symbolic ids and site metadata:

```text
layout_id
field_id
table_id
source span
region id
```

MIRBuilder must not compute:

```text
byte_offset
field type
field mutability
element stride
table representation
alignment
bounds policy
```

Verifier / contract code owns those facts. LLVM lowering must not recover them
from source strings, operand names, helper names, or backend conventions.

## Selected / Deferred

Selected for this slice:

```text
VerifiedMemAccessPlan metadata skeleton
VerifiedFieldAccess rows
VerifiedTableAccess rows
MIR JSON metadata for verified plans
hako_check positive evidence keys
```

Deferred until the next slice:

```text
actual LLVM GEP/load/store behavior
CurrentAllocOwnerId / OwnerEq
AtomicRemoteHead
TLS backing transfer
owner slot reuse
Python-template C diagnostic baseline retirement
product activation
hook install
global allocator claim
winner claim
```

If the implementation can add a fully verified LLVM GEP/load/store pilot without
hardcoded layout/table truth, it may do so in a follow-up commit under this same
`MIR-FMEM-008B` blocker. Do not open lowering by guessing offsets.

## Report / Check Fields

New positive evidence:

```text
fastmem_verified_mem_access_plan_count
fastmem_verified_field_access_count
fastmem_verified_table_access_count
```

Existing fail-fast fields remain required:

```text
fastmem_field_id_missing_count=0
fastmem_table_id_missing_count=0
fastmem_unverified_layout_access_count=0
fastmem_table_index_unchecked_count=0
fastmem_unknown_alignment_count=0
fastmem_atomic_field_plain_store_count=0
fastmem_layout_ref_escape_count=0
fastmem_lowering_recomputed_layout_offset_count=0
```

Boundary proof remains closed:

```text
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Acceptance

This slice is complete when:

```text
FunctionMetadata exposes fastmem_access_plans
MIR JSON emits fastmem_access_plans
hako_check reports the three positive verified-plan counters
missing verified-plan rows fail before lowering
owner-runtime MemOps remain deferred
Python-template C remains diagnostic baseline only
```

The acceptance does not require production allocator activation or bridge
retirement.

## Landed

```text
src/mir/fastmem_access_plan.rs:
  FastMemAccessPlan
  FastMemFieldAccessPlan
  FastMemTableAccessPlan
  FastMemAccessPlanStatus
  refresh_function_fastmem_access_plans

FunctionMetadata:
  fastmem_access_plans[]

Semantic refresh:
  refreshes fastmem_access_plans after FastMemory MemOps are present

MIR JSON:
  emits metadata.fastmem_access_plans[]

Python LLVM metadata loader:
  loads fastmem_access_plans_by_site for the future lowering consumer

hako_check report vocabulary:
  fastmem_verified_mem_access_plan_count
  fastmem_verified_field_access_count
  fastmem_verified_table_access_count
```

Current implementation state:

```text
plan status:
  symbolic_only

reason:
  canonical layout/table contracts do not yet provide byte offsets, table
  representation, alignment, or bounds policy

lowering:
  still closed
```

## Next

Open the next `MIR-FMEM-008B` slice for concrete layout/table contract
resolution:

```text
PageMetaLayoutV0 / table contracts
Verified field offsets / types / alignment
Verified table element representation / stride / bounds proof
then LLVM GEP/load/store lowering from verified plans only
```

## Stop Line

Stop before opening LLVM layout/table lowering if any of these are true:

```text
layout/table truth exists only in Python-template C
LLVM lowerer recomputes field offsets
LLVM lowerer guesses table representation
TableIndex has no bounds proof
alignment is unknown
atomic fields can be written through plain FieldStore
LayoutRef or pointer-like MemValue escapes
```
