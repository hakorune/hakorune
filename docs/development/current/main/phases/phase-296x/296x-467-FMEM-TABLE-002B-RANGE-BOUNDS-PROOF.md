---
Status: Active
Date: 2026-06-06
Scope: FMEM-TABLE-002B RangeIndexFact consumption as FastMemory TableIndex bounds proof.
Related:
  - docs/development/current/main/phases/phase-296x/296x-466-FMEM-TABLE-002A-LENGTH-FACT-CARRIER.md
  - docs/development/current/main/design/mir-proof-envelope-v0-ssot.md
  - src/mir/fastmem_access_plan.rs
  - src/mir/range_index_fact.rs
---

# 296x-467 FMEM-TABLE-002B Range Bounds Proof

## Decision

`RangeIndexFact` may become `BoundsProof::RangeFact` for a FastMemory
`TableIndex` only after a matching `FastMemTableLengthFact` exists.

This keeps the order fixed:

```text
length fact first
bounds proof second
overflow proof later
TableIndex lowering last
```

## Match Conditions

A range fact is consumed only when all conditions hold:

```text
range.index_value == TableIndex.index
range.upper_exclusive_value == FastMemTableLengthFact.length_value
range.body_bb == TableIndex.block
range.step == 1
range.end_exclusive == true
range.index_body_read_only == true
range.loop_carried_writes_supported == false
```

If any condition is missing, `bounds_proof_valid` remains false.

## Proof Behavior

With a matching length fact but no matching range proof:

```text
table_length_resolved=1
bounds_proof_valid=0
lowerable=0
failure_reason=verified-table-access-proof-incomplete
```

With both a matching length fact and matching range proof:

```text
table_length_resolved=1
bounds_proof_valid=1
bounds_proof=range_fact:<fact_id>
field_offset_resolved=0
overflow_proof_valid=0
lowerable=0
failure_reason=verified-table-access-proof-incomplete
```

## Boundary

Allowed:

```text
consume existing RangeIndexFact as a bounds proof
emit bounds_proof in existing MIR JSON access-plan rows
keep TableIndex non-lowerable
```

Forbidden:

```text
create a table length from RangeIndexFact
consume RangeIndexFact without a FastMemory length fact
add overflow proof
open LLVM TableIndex lowering
mark inbounds GEP
choose page-map strategy
query Type ABI / Provider ABI
open product activation, hook install, global allocator, or winner claim
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
FMEM-TABLE-003:
  add OverflowProof for index * stride + field_offset

FMEM-TABLE-004:
  add JSON/report/check rejection for incomplete verified table proofs
```
