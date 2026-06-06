---
Status: Active
Date: 2026-06-06
Scope: FMEM-TABLE-003A TableIndex result to FieldLoad/Store base link for field-offset proof.
Related:
  - docs/development/current/main/phases/phase-296x/296x-468-FMEM-TABLE-003-DESIGN-CONSULT.md
  - docs/development/current/main/design/fastmem-layout-table-contract-v0-ssot.md
  - src/mir/fastmem_access_plan.rs
  - src/mir/fastmem_layout_contract.rs
---

# 296x-469 FMEM-TABLE-003A Field Offset Link

## Decision

Add a verifier-owned FastMemory link row between:

```text
TableIndex.result
  -> FieldLoad/FieldStore.base
```

This is the first explicit source for `field_offset_resolved`.

## V0 Link Rule

The v0 link is deliberately narrow:

```text
same function
same FastMemory region
same basic block
field instruction appears after table instruction
table.result == field.base
field plan is verified
field byte_offset is resolved by fastmem_layout_contract
```

The link row records the canonical field id and byte offset. It does not prove
overflow by itself.

## Proof Behavior

With a link:

```text
field_offset_resolved=1
overflow_proof_valid=0
lowerable=0
```

Without a link:

```text
field_offset_resolved=0
overflow_proof_valid=0
lowerable=0
```

## Boundary

Allowed:

```text
add fastmem_table_field_access_links[] metadata
set field_offset_resolved only from a verified link
source field_offset from resolve_fastmem_field_contract()
keep overflow_proof_valid=0
keep TableIndex non-lowerable
```

Forbidden:

```text
guess field_offset
use field aliases after canonicalization
perform CFG dominance reasoning in v0
add overflow proof
open LLVM TableIndex lowering
query Type ABI / Provider ABI
open product activation, hook install, global allocator, or winner claim
```

## Acceptance

```bash
cargo test -q fastmem_access_plan --lib
cargo test -q fastmem_metadata --lib
cargo test -q mir_json_emit --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
FMEM-TABLE-003B:
  add OverflowProof from index * stride + linked field_offset

FMEM-TABLE-004:
  add JSON/report/check rejection for incomplete proofs
```
