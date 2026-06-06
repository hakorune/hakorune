---
Status: Done
Date: 2026-06-06
Scope: ContractRegionV0 docs-only envelope decision after FastMemory access-plan metadata.
Blocker: MIR-FMEM-008B
Related:
  - docs/development/current/main/design/contract-region-v0-ssot.md
  - docs/development/current/main/design/mir-fastmem-memop-dialect-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-457-VERIFIED-MEM-ACCESS-PLAN.md
---

# 296x-458 ContractRegionV0 Docs

## Purpose

Fix the common design boundary before concrete layout/table lowering:

```text
fastmem is not a separate MIRBuilder.
fastmem is the memory-profile instance of a future ContractRegion envelope.
```

This row is docs-only. It does not rename `FastMemRegion`, change MIR JSON,
add `contract_regions[]`, or open a second profile.

## Decision

Commonize only the envelope:

```text
ContractRegionV0:
  region_id
  profile
  contract_id
  source_span
  origin
  flags
  obligations
  verifier/report envelope
```

Keep the payload profile-specific:

```text
memory:
  MemOp
  MemValueKind
  VerifiedMemAccessPlan
  layout/table contracts
  LLVM GEP/load/store lowering

future simd:
  SimdOp
  VerifiedVectorPlan

future io:
  IoOp
  VerifiedBufferAccessPlan
```

## Repository Reading

Current code remains:

```text
FunctionMetadata.fastmem_regions[]
FunctionMetadata.fastmem_access_plans[]
MirInstruction::MemOp
MemOpKind
```

Interpret it as:

```text
FastMemRegion:
  memory-profile wrapper over future ContractRegionV0

VerifiedMemAccessPlan:
  memory-profile payload, not a generic access-plan replacement
```

## Stop Lines

Do not use this row to open:

```text
FastMemRegion mass rename
FunctionMetadata.contract_regions[]
generic RegionOp
generic VerifiedRegionAccessPlan
fastpath / fastio / fastsimd parser behavior
second profile implementation
Type ABI hot lookup
Provider ABI hot dispatch
product allocator activation
Python-template C bridge retirement
```

## Next

Return to `MIR-FMEM-008B` concrete layout/table contract resolution:

```text
PageMetaLayoutV0 / table contract truth
field offset / type / mutability / alignment
table element representation / stride / length / bounds proof
VerifiedMemAccessPlan status moves from symbolic_only to verified
LLVM GEP/load/store consumes verified memory plans only
```

## Acceptance

This row is complete when:

```text
ContractRegionV0 docs-only SSOT exists
FastMemRegion remains current code/report name
memory-specific payload names remain explicit
mimalloc layout/table lowering remains the next active work
```
