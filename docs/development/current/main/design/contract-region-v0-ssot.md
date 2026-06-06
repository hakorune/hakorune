---
Status: SSOT
Decision: accepted
Date: 2026-06-06
Scope: Common contract-region envelope for fast memory and future fastpath profiles.
Related:
  - docs/development/current/main/design/mir-fastmem-memop-dialect-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-458-CONTRACT-REGION-V0-DOCS.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
---

# ContractRegionV0

## Decision

`fastmem` is not a separate MIRBuilder. It is the memory-profile instance of a
contract-bound region model.

Long-term shape:

```text
ContractRegionV0:
  common envelope

FastMemRegion:
  memory-profile wrapper over that envelope
```

Do not rename the current `FastMemRegion` code or report fields yet.

Short form:

```text
Envelope is generic.
Payload is profile-specific.
```

## Common Envelope

The common part is:

```text
region_id
profile
contract_id
source_span
origin
flags
obligations
report identity
verifier envelope
producer identity
```

Profiles:

```text
memory:
  current fastmem / mimalloc path

simd:
  future vector fast path

io:
  future buffer/socket fast path
```

The current implementation has only the memory profile.

## Obligations

Obligations are shared vocabulary, but each profile decides which ones are
required:

```text
no_alloc
no_safepoint
no_escape
no_type_abi_hot_lookup
no_provider_abi_hot_dispatch
no_unverified_layout_access
```

Use a stateful obligation model in future code:

```text
required
forbidden
allowed
profile_defined
```

Do not flatten obligations into permanent booleans too early. `fastmem` will
mostly require the strict memory-hot-path set, but future IO/SIMD profiles may
need different blocking, safepoint, or allocation policies.

## Profile-Specific Payloads

Memory-specific payloads stay memory-specific:

```text
MemOpKind
MemValueKind
MemLayoutContract
MemTableContract
MemFieldContract
VerifiedMemAccessPlan
LLVM GEP/load/store lowering
memory-specific escape rules
memory-specific alignment rules
```

Future profiles should add their own payloads instead of forcing memory
concepts into generic names:

```text
memory:
  MemOp
  VerifiedMemAccessPlan

simd:
  SimdOp
  VerifiedVectorPlan

io:
  IoOp
  VerifiedBufferAccessPlan
```

Rejected genericization:

```text
generic RegionOp that hides memory/vector/io semantics
generic VerifiedRegionAccessPlan replacing VerifiedMemAccessPlan
```

## Current Repository Reading

For the current mimalloc lane:

```text
FastMemRegion:
  current memory-profile implementation

FunctionMetadata.fastmem_regions[]:
  current memory-profile region metadata

FunctionMetadata.fastmem_access_plans[]:
  memory-profile verified access plan surface
```

Do not introduce `FunctionMetadata.contract_regions[]` until one of these is
true:

```text
second profile enters implementation
FastMemRegion common header extraction is behavior-preserving
contract_region_* report fields are needed by an active checker
FastMemRegion naming blocks a real implementation task
```

## MIRBuilder Boundary

MIRBuilder remains the common AST-to-MIR representation layer.

MIRBuilder may:

```text
record ContractRegion-style header facts
record memory-profile FastMemRegion metadata
emit MemOp instructions for memory-profile dialect operations
preserve source span / origin / contract id
```

MIRBuilder must not:

```text
choose profile producer routes
choose LLVM vs C
compute layout offsets
choose table representation
choose fast/slow paths
open Type ABI hot lookup
open Provider ABI hot dispatch
claim product activation
```

## Report Reading

Common report fields may be added later:

```text
contract_region_model=1
contract_region_count
contract_region_profile_memory_count
contract_region_profile_simd_count=0
contract_region_profile_io_count=0
contract_region_verifier_pass_count
```

Do not add those fields merely for inventory if the current active row does not
consume them. The active memory lane should continue to use memory-specific
fields:

```text
fastmem_region_count
fastmem_access_plan_count
fastmem_layout_contract_verified
fastmem_table_contract_verified
fastmem_layout_ref_escape_count=0
fastmem_unverified_layout_access_count=0
```

## Task Order

Current order:

```text
1. Define ContractRegionV0 docs-only.
2. Keep FastMemRegion as memory-profile wrapper.
3. Continue MIR-FMEM-008B concrete layout/table contract resolution.
4. Verify field offsets, field types, alignment, table representation, stride,
   and bounds proof for VerifiedMemAccessPlan.
5. Open LLVM GEP/load/store from verified memory plans only.
6. Extract a shared ContractRegionHeader only after it stops being speculative.
```

## Non-Goals

```text
FastMemRegion mass rename
generic RegionOp
generic VerifiedRegionAccessPlan replacing memory plans
new fastpath / fastsimd / fastio parser behavior
new profile implementation
product allocator activation
Python-template C bridge retirement
```
