---
Status: Active
Date: 2026-06-06
Row: MIR-FMEM-008B
Scope: TableIndex bounds/length proof decision before LLVM GEP/load/store lowering.
Related:
  - docs/development/current/main/design/fastmem-layout-table-contract-v0-ssot.md
  - docs/development/current/main/design/mir-proof-envelope-v0-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-459-FASTMEM-LAYOUT-TABLE-CONTRACTS.md
---

# 296x-460 VerifiedTableAccess Proof Decision

## Decision

Main line:

```text
Require a MIR/verifier-owned bounds proof row before TableIndex becomes
lowerable.
```

Short optional row:

```text
VerifiedElementRef-only field GEP smoke
```

Deferred:

```text
Page map strategy / two-level table / fixed PageTableLengthV0 product shape
```

## Rule

```text
Layout verified != Access verified
```

`PageMetaLayoutV0` field offsets, sizes, classes, and alignment prove the shape
of a metadata element. They do not prove that a particular pointer/index pair is
safe to access.

LLVM lowering may consume only:

```text
VerifiedElementRef
VerifiedTableAccess
```

It must not lower from layout facts alone.

## VerifiedElementRef

Purpose:

```text
field-only LLVM GEP/load/store smoke without TableIndex
```

Shape:

```text
VerifiedElementRef:
  base_ptr
  layout_id
  provenance
  no_escape
```

Allowed:

```text
FieldLoad / FieldStore from a verifier-produced element ref
```

Forbidden:

```text
page_table[index].field
unknown index + field offset
remote_head plain FieldStore
speed/product/winner claims
```

## VerifiedTableAccess

`TableIndex` becomes lowerable only when all of these are true:

```text
table_length_resolved=1
bounds_proof_valid=1
stride_resolved=1
field_offset_resolved=1
overflow_proof_valid=1
alignment_valid=1
element_layout_verified=1
```

Bounds proof and overflow proof are separate.

```text
bounds:
  index is in table range

overflow:
  index * stride + field_offset cannot overflow target usize
  offset + access_size stays inside the verified table/object range
```

## Proof Vocabulary

Initial proof vocabulary:

```text
TableLengthPolicy:
  ConstLen(n)
  Pow2MaskLen(n, mask)
  GuardedLen(n, guard_id)

BoundsProof:
  ConstantIndexInRange
  MaskProof
  DominatingGuard
  RangeFact

OverflowProof:
  UsizeMulAddNoOverflow
  OffsetWithinObject
```

Later page-map strategies may add:

```text
TwoLevel { l1_len, l2_len }
StrategyProof
```

## DirectArray Reuse Boundary

Worker-reviewed decision:

```text
Commonize:
  ProofEnvelopeV0
  RangeIndexFact as BoundsProof::RangeFact input
  RegionStabilityFact concept as stable-base/no-escape ingredient

Do not commonize:
  DirectArrayExtentFact
  DirectArrayAccessPlan
  DirectArrayProofKind
  generic VerifiedRegionAccessPlan
```

`DirectArrayExtentFact` is tied to `ArrayBox` / `DirectArrayI64` receiver
capacity, field origin, and collection semantics. FastMemory needs a
memory-profile table length/provenance fact instead.

Task split:

```text
1. Add ProofEnvelopeV0 carrier only.
2. Add FastMemory VerifiedTableAccessProof fields.
3. Consume RangeIndexFact only as one BoundsProof source.
4. Add OverflowProof separately.
5. Keep DirectArray routes untouched.
```

## Lowering Contract

LLVM lowering reads verified plans only.

```text
lowerer:
  consumes VerifiedElementRef / VerifiedTableAccess
  emits GEP/load/store

lowerer must not:
  query Type ABI
  query Provider ABI
  call Python-template C bridge
  infer table length
  infer page-map strategy
  recompute field offsets
  attach inbounds GEP without proof
```

## Acceptance

```text
TableIndex without length remains non-lowerable
TableIndex with length + bounds + overflow can become lowerable
remote_head plain FieldStore still rejects
owner_id alias does not appear in verified plans
Python-template C bridge remains diagnostic baseline
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## No-Go

```text
table-length-unresolved but lowerable
runtime provider call for length
Type ABI query in hot lowering path
mask expression treated as bounds without MaskProof
index * stride lowered without OverflowProof
remote_head lowered as plain FieldStore
field-only lowering that secretly lowers page_table[index].field
```
