---
Status: Active
Date: 2026-06-06
Scope: Design stop for FMEM-TABLE-003 overflow proof and field-offset ownership.
Related:
  - docs/development/current/main/phases/phase-296x/296x-467-FMEM-TABLE-002B-RANGE-BOUNDS-PROOF.md
  - docs/development/current/main/phases/phase-296x/296x-460-VERIFIED-TABLE-ACCESS-PROOF-DECISION.md
  - docs/development/current/main/design/fastmem-layout-table-contract-v0-ssot.md
  - src/mir/fastmem_access_plan.rs
  - src/mir/fastmem_layout_contract.rs
---

# 296x-468 FMEM-TABLE-003 Design Consult

## Current State

`FMEM-TABLE-002B` connected `RangeIndexFact` as a FastMemory `TableIndex`
bounds proof only after a matching `FastMemTableLengthFact` exists.

Current verified proof shape can reach:

```text
table_length_resolved=1
bounds_proof_valid=1
stride_resolved=1
alignment_valid=1
element_layout_verified=1
field_offset_resolved=0
overflow_proof_valid=0
lowerable=0
```

## Blocker

`FMEM-TABLE-003` asks for:

```text
OverflowProof for index * stride + field_offset
```

But the current `FastMemTableAccessPlan` is a plan for the `TableIndex` MemOp
only. It knows:

```text
table_id
table value
index value
element layout / repr / stride / alignment
length / bounds proof
```

It does **not** know which field is later loaded or stored from the resulting
element reference.

Therefore, setting:

```text
field_offset_resolved=1
overflow_proof_valid=1
```

inside the current `TableIndex` plan would require guessing a field offset, or
pretending it is always zero. That would violate the existing rule:

```text
Layout verified != Access verified
```

## Decision Needed

Question:

```text
How should FastMemory connect a TableIndex element reference to the field
access whose offset participates in the overflow proof?
```

Candidate A:

```text
TableIndex-only overflow proof with field_offset=0
```

Rejected direction. This proves only `index * stride`, not
`page_table[index].field`, and it can make later lowering look safer than it is.

Candidate B:

```text
Verifier-owned TableFieldAccessLink
  table_plan.result == field_plan.base
  same block / dominated use window
  field offset from resolve_fastmem_field_contract()
```

Recommended next design. It keeps `TableIndex` and `FieldLoad/FieldStore`
separate in MIR, but publishes a verified link that can feed field offset and
access size into the overflow proof.

Candidate C:

```text
Combined VerifiedTableFieldAccess payload
```

Also viable, but heavier. This would represent the whole
`page_table[index].field` access as one verified payload row. It may be useful
later for lowering, but it is more invasive than the first link row.

## Recommended Split

Use **B first, C later if lowering needs it**.

Owner split:

```text
MIRBuilder:
  emits TableIndex and FieldLoad/FieldStore MemOps with symbolic ids only

fastmem_layout_contract:
  owns canonical field_id, byte_offset, field_type, alignment, field_class

fastmem_access_plan:
  owns table plan, field plan, and the TableFieldAccessLink proof row

Verifier / access-plan refresh:
  may set field_offset_resolved only when a FieldLoad/FieldStore consumes the
  TableIndex result and the field contract resolves

Lowering:
  must consume verified proof rows only
  must not recompute offsets or infer field/table relationships
```

## Next Task Order

```text
FMEM-TABLE-003A:
  add TableFieldAccessLink / field-offset proof row
  link TableIndex.result to FieldLoad/FieldStore.base
  source field_offset only from resolve_fastmem_field_contract()
  keep overflow_proof_valid=0
  keep TableIndex lowerable=0

FMEM-TABLE-003B:
  add OverflowProof using:
    index
    element_stride
    linked field_offset
    linked field access size
    resolved table length / bounds proof
  keep TableIndex lowerable=0 unless every proof bit is complete

FMEM-TABLE-004:
  add JSON/report/check rejection for incomplete proofs

FMEM-TABLE-005:
  open LLVM lowering only for fully verified table-field access
```

## Stop Line

Do not implement overflow proof until `field_offset_resolved` has an explicit
source.

Specifically do not:

```text
assume field_offset=0 for page_table[index].field
mark overflow_proof_valid=1 from bounds alone
mark TableIndex lowerable
let lowering recompute or guess offsets
query Type ABI / Provider ABI
choose page-map strategy
open product activation / hook / global allocator / winner claim
```

## Acceptance For This Consult Row

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
