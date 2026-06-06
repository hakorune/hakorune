---
Status: Active
Date: 2026-06-06
Scope: FMEM-TABLE-001 proof payload fields for FastMemory TableIndex plans.
Related:
  - docs/development/current/main/design/mir-proof-envelope-v0-ssot.md
  - docs/development/current/main/design/fastmem-layout-table-contract-v0-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-460-VERIFIED-TABLE-ACCESS-PROOF-DECISION.md
  - src/mir/fastmem_access_plan.rs
---

# 296x-464 FastMemory Table Access Proof Payload

## Decision

`FastMemTableAccessPlan` now carries an explicit `FastMemTableAccessProof`
payload, but `TableIndex` lowering remains closed.

This implements the first VerifiedTableAccessProof row without selecting a page
map strategy or opening LLVM GEP/load/store for table indexes.

## Payload Fields

```text
table_length_resolved
bounds_proof_valid
stride_resolved
field_offset_resolved
overflow_proof_valid
alignment_valid
element_layout_verified
table_length_policy
bounds_proof
overflow_proof
failure_reason
```

Current `page_table` state:

```text
element_layout_verified=1
stride_resolved=1
alignment_valid=1
table_length_resolved=0
bounds_proof_valid=0
field_offset_resolved=0
overflow_proof_valid=0
lowerable=0
failure_reason=table-length-unresolved
```

## Boundary

Allowed:

```text
add proof payload fields
emit proof fields in MIR JSON metadata
keep current table plan rejected / non-lowerable
```

Forbidden:

```text
choose page-map strategy
mark TableIndex lowerable
infer bounds in lowering
call Type ABI / Provider ABI
open product activation
```

## Acceptance

```bash
cargo test -q fastmem_access_plan --lib
cargo test -q mir_json_emit --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
FMEM-TABLE-002:
  consume RangeIndexFact-style input as BoundsProof::RangeFact only when a
  FastMemory-owned table length fact exists

FMEM-TABLE-003:
  add OverflowProof for index * stride + field_offset
```
