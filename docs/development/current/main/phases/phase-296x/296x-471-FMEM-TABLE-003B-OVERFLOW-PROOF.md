---
Status: Active
Date: 2026-06-06
Scope: FMEM-TABLE-003B overflow proof for FastMemory TableIndex access plans.
Related:
  - docs/development/current/main/phases/phase-296x/296x-470-FMEM-HANDOFF-COMMONIZATION-ORDER.md
  - docs/development/current/main/phases/phase-296x/296x-469-FMEM-TABLE-003A-FIELD-OFFSET-LINK.md
  - docs/development/current/main/design/fastmem-layout-table-contract-v0-ssot.md
  - src/mir/fastmem_access_plan.rs
  - src/mir/fastmem_layout_contract.rs
---

# 296x-471 FMEM-TABLE-003B Overflow Proof

## Decision

Add a verifier-owned overflow proof for FastMemory `TableIndex` access plans.

The proof consumes only already-verified memory-profile facts:

```text
table length fact
range bounds proof
table stride
table element size
TableIndex.result -> FieldLoad/Store.base link
linked field byte_offset
linked field size
target usize width
```

No backend lowering opens in this row.

## Proof Rule

`overflow_proof_valid=1` only when all earlier proof bits are true:

```text
table_length_resolved=1
bounds_proof_valid=1
stride_resolved=1
field_offset_resolved=1
alignment_valid=1
element_layout_verified=1
```

Then the verifier checks:

```text
length * stride <= target usize max
field_offset + field_size <= element_size
field_offset + field_size <= target usize max
```

The table byte-range proof and element field-range proof stay separate in the
implementation, but the published proof string is one access-proof row:

```text
usize_mul_add_no_overflow+offset_within_object:...
```

## Metadata Added

Field plans and table-field links now publish:

```text
field_size
```

Table plans now publish:

```text
element_size
```

These are layout-contract facts from `fastmem_layout_contract`, not values
computed by MIRBuilder or lowering.

## Boundary

Allowed:

```text
set overflow_proof_valid from verified proof inputs
clear verified-table-access-proof-incomplete when every proof bit is complete
leave LLVM TableIndex lowering closed
emit field_size / element_size in MIR JSON metadata
```

Forbidden:

```text
infer field size in lowering
query Type ABI / Provider ABI
open LLVM GEP/load/store for TableIndex
open product activation, hook install, global allocator, or winner claim
```

## Acceptance

```bash
cargo test -q fastmem_access_plan --lib
cargo test -q fastmem_layout_contract --lib
cargo test -q fastmem_metadata --lib
cargo test -q mir_json_emit --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
FMEM-TABLE-004:
  add JSON/report/check rejection for incomplete table access proofs

MIR-FMEM-008C:
  open LLVM lowering only for complete VerifiedTableAccess rows
```
