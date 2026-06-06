---
Status: Active
Decision: accepted
Date: 2026-06-06
Scope: small shared proof envelope for DirectArray and FastMemory proof reporting.
Related:
  - docs/development/current/main/design/fastmem-layout-table-contract-v0-ssot.md
  - docs/development/current/main/design/contract-region-v0-ssot.md
  - src/mir/range_index_fact.rs
---

# MIR ProofEnvelopeV0

## Decision

Share only a small proof envelope.

Do not share access-plan payloads.

```text
Common:
  ProofEnvelopeV0

Profile-specific:
  DirectArrayAccessPlan
  FastMemAccessPlan
  VerifiedTableAccessProof
  VerifiedElementRef
```

This keeps reusable proof identity/reporting thin while avoiding a generic
`VerifiedRegionAccessPlan` that would mix collection semantics with raw memory
access safety.

## Envelope Shape

```text
ProofEnvelopeV0:
  site
  profile
  producer
  proof_ids
  obligation_ids
  verifier_flags
  failure_reason
```

Profiles:

```text
direct_array
fastmem
```

The envelope is report/metadata carrier only. It does not decide lowering and
does not replace domain-specific proof payloads.

## Reusable Facts

Reusable:

```text
RangeIndexFact:
  canonical index in [lower, upper) producer view

RegionStabilityFact concept:
  reusable as stable-base/no-escape ingredient
```

Use `RangeIndexFact` as an input to FastMemory `BoundsProof::RangeFact` only
after checking:

```text
index_value matches TableIndex index
upper_exclusive_value matches a FastMemory table length fact
step == 1
end is exclusive
access site is in the proven body/scope
index is read-only for the access window
unsupported carried writes are absent
```

## DirectArray-Specific

Do not reuse these as FastMemory proof payloads:

```text
DirectArrayExtentFact
DirectArrayProofKind
Checked / ProvedUnchecked
ArrayGet / ArraySet route names
direct_array_i64_load/store route names
AppendOrOverwrite
StackTopPop
caller-precondition collection routes
```

Reason:

```text
DirectArray proves collection access semantics.
FastMemory proves metadata-table address construction and memory access safety.
```

## DirectArray Auto-FastMemory Stop Line

DirectArray may reuse the shared proof envelope and reusable proof ingredients,
but DirectArray access does not automatically become a FastMemory region in the
current lane.

```text
Allowed:
  DirectArray report adapter emits ProofEnvelopeV0-style proof identity
  RangeIndexFact-style facts may be reused as proof inputs after domain checks

Forbidden:
  DirectArray access auto-generates fastmem ContractRegion
  DirectArrayAccessPlan is replaced by FastMemAccessPlan
  DirectArrayExtentFact becomes FastMemory table length proof
  DirectArray route names become FastMemory MemOpKind
```

Any future auto-lowering path needs a separate docs/reference decision because
it changes source/lowering semantics, not just proof reporting.

## FastMemory-Specific

FastMemory owns:

```text
VerifiedElementRef
VerifiedTableAccessProof
TableLengthPolicy
BoundsProof
OverflowProof
alignment / stride / element layout checks
pointer provenance / no-escape checks
```

`FastMemAccessPlan` remains the memory-profile plan table. Add proof fields to
that payload; do not rename it to a generic access plan.

## No-Go

```text
GenericAccessPlan
VerifiedRegionAccessPlan replacing FastMemAccessPlan
common BoundsPolicy required by DirectArray and FastMemory
using DirectArrayExtentFact as FastMemory table length proof
marking TableIndex verified from layout facts alone
```

## Task Order

```text
PROOF-ENV-001:
  docs/code vocabulary for ProofEnvelopeV0 as carrier only

FMEM-TABLE-001:
  add FastMemory VerifiedTableAccessProof fields to FastMemTableAccessPlan
  no lowering

FMEM-TABLE-002:
  consume RangeIndexFact as BoundsProof::RangeFact
  require FastMemory-owned table length fact

FMEM-TABLE-003:
  add OverflowProof for index * stride + field_offset

FMEM-TABLE-004:
  emit JSON/report/check fields and reject verified TableIndex unless all
  required proofs are present

FMEM-TABLE-005:
  lower only VerifiedTableAccess

DIRECT-ARRAY-ADAPTER-LATER:
  optional report adapter that emits ProofEnvelopeV0 from existing DirectArray
  proof ids without changing DirectArray access planning

DIRECTARRAY-FMEM-COMMON-001:
  add the DirectArray/FastMemory proof-envelope report/check adapter
  no source syntax change
  no DirectArray auto-fastmem region
  no shared access-plan payload
  no LLVM lowering behavior change

DIRECTARRAY-FMEM-AUTO-LOWERING-LATER:
  parked until a separate reference decision accepts DirectArray access as a
  source/lowering producer for FastMemory MemOps
```
