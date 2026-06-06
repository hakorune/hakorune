---
Status: Active
Date: 2026-06-06
Scope: FMEM-TABLE-002A FastMemory table length fact carrier and access-plan consumption.
Related:
  - docs/development/current/main/phases/phase-296x/296x-465-FMEM-TABLE-002-DESIGN-CONSULT.md
  - docs/development/current/main/design/mir-proof-envelope-v0-ssot.md
  - docs/development/current/main/design/fastmem-layout-table-contract-v0-ssot.md
  - src/mir/fastmem_table_length_fact.rs
  - src/mir/fastmem_access_plan.rs
---

# 296x-466 FMEM-TABLE-002A Table Length Fact Carrier

## Decision

`FastMemTableLengthFact` is the FastMemory-owned semantic metadata carrier for
TableIndex length truth.

This accepts the `B now / C later` split from 296x-465:

```text
now:
  memory-profile table length facts exist in FunctionMetadata
  access plans may consume matching explicit facts

later:
  RangeIndexFact bounds proof
  overflow proof
  page-map strategy selection
  LLVM TableIndex lowering
```

MIRBuilder still only preserves symbolic table ids and provenance. It does not
invent table lengths.

## Implemented Surface

```text
FunctionMetadata.fastmem_table_length_facts
FastMemTableLengthFact
FastMemTableLengthPolicyKind::ExplicitConstLen
fastmem_table_length_fact refresh module
fastmem_access_plan consumption by region + table_id + table_value
MIR JSON fastmem_table_length_facts[]
```

The refresh owner currently normalizes explicit facts already present on the
function. It emits no new fact without explicit memory-profile evidence.

## Proof Behavior

Without a matching length fact:

```text
table_length_resolved=0
failure_reason=table-length-unresolved
lowerable=0
```

With a matching explicit length fact:

```text
table_length_resolved=1
table_length_policy=explicit_const_len
length=<resolved_length when available>
bounds_proof_valid=0
overflow_proof_valid=0
lowerable=0
failure_reason=verified-table-access-proof-incomplete
```

`resolved_length` is optional reporting/overflow input. `length_value` remains
the primary proof identity for the later RangeIndexFact match.

## Boundary

Allowed:

```text
add FastMemory table length facts
preserve and normalize explicit facts
emit facts in MIR JSON
set table_length_resolved when a matching fact exists
keep TableIndex non-lowerable
```

Forbidden:

```text
infer table length from layout contracts
choose one-level / two-level / mask page-map strategy
consume RangeIndexFact as bounds proof
add overflow proof
open LLVM TableIndex lowering
query Type ABI / Provider ABI
open product activation, hook install, global allocator, or winner claim
```

## Acceptance

```bash
cargo test -q fastmem_table_length_fact --lib
cargo test -q fastmem_access_plan --lib
cargo test -q fastmem_metadata --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
FMEM-TABLE-002B:
  connect RangeIndexFact as BoundsProof::RangeFact only when:
    index_value matches TableIndex index
    upper_exclusive_value matches FastMemory length_value
    step == 1
    end_exclusive == true
    index_body_read_only == true
    loop_carried_writes_supported == false

FMEM-TABLE-003:
  add OverflowProof for index * stride + field_offset
```
