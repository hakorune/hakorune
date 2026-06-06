---
Status: Active
Date: 2026-06-06
Scope: Design stop for FMEM-TABLE-002 table length / bounds proof ownership.
Related:
  - docs/development/current/main/phases/phase-296x/296x-464-FASTMEM-TABLE-ACCESS-PROOF-PAYLOAD.md
  - docs/development/current/main/design/mir-proof-envelope-v0-ssot.md
  - docs/development/current/main/design/fastmem-layout-table-contract-v0-ssot.md
  - src/mir/fastmem_access_plan.rs
  - src/mir/range_index_fact.rs
---

# 296x-465 FMEM-TABLE-002 Design Consult

## Current State

`FastMemTableAccessPlan` now has proof payload slots:

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

Current `page_table` proof state:

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

## Design Question

Before `FMEM-TABLE-002` can be implemented, decide the owner and shape of the
FastMemory table length fact.

Question:

```text
Who owns page_table length truth for FastMemory TableIndex proof?
```

Candidates:

```text
A. Contract-owned const length
   PageMapV0 declares page_table length as a constant table contract.

B. Builder/MIR fact-owned length
   MIRBuilder or semantic refresh emits a memory-profile table length fact
   separate from the layout contract.

C. Strategy-owned length
   page-map strategy selection owns table length, e.g. one-level mask table,
   two-level table, or page-base-mask bridge.
```

## Current Recommendation

Prefer **B now, C later**, with one tightening:

```text
B now means:
  MIR semantic metadata / FastMemory verifier-owned fact

B now does not mean:
  MIRBuilder invents table length
```

MIRBuilder should only preserve symbolic ids and provenance. The length fact
should be a FastMemory memory-profile fact consumed by `FastMemAccessPlan`.

Implementation direction:

```text
FMEM-TABLE-002:
  introduce FastMemory-owned table length fact vocabulary,
  but keep page-map strategy deferred.

FMEM-TABLE-009 later:
  map one-level / two-level / mask strategies into the same table length fact
  surface after the proof API is stable.
```

Reason:

```text
A:
  easy, but risks baking a benchmark table shape into the layout contract.

B:
  keeps layout facts separate from access proof facts and allows RangeIndexFact
  to be consumed only after FastMemory owns a length fact.

C:
  ultimately needed, but selecting strategy now would mix proof-surface work
  with page-map/product-shape decisions.
```

## Accepted Owner Split

If this design is accepted, use this file ownership:

```text
src/mir/function/types.rs:
  FastMemTableLengthFact
  FastMemTableLengthPolicyKind
  FunctionMetadata.fastmem_table_length_facts

src/mir/fastmem_table_length_fact.rs:
  FastMemory-specific semantic refresh owner
  emits facts only from explicit memory-profile evidence
  does not choose page-map strategy

src/mir/mod.rs:
  module registration

src/mir/semantic_refresh.rs:
  refresh table-length facts before refresh_function_fastmem_access_plans

src/mir/fastmem_access_plan.rs:
  consumes matching facts by region + table_id + table_value
  sets length / table_length_resolved / table_length_policy
  keeps bounds_proof_valid=false
  keeps TableIndex non-lowerable

src/mir/fastmem_layout_contract.rs:
  non-owner for length truth
  keeps table representation / stride / alignment / element-layout facts
```

JSON/report surface:

```text
src/runner/mir_json_emit/metadata.rs:
  emits fastmem_table_length_facts[]

tools/hako_check/*:
  later check/report surface, only after Rust metadata shape lands
```

Recommended fact carrier:

```text
FastMemTableLengthFact:
  fact_id
  region
  table_id
  table_value
  length_value
  resolved_length: Option<u64>
  policy
```

`length_value` is the primary proof identity. `resolved_length` is optional
reporting / later overflow input only.

## Required Invariants

Any accepted design must preserve:

```text
layout verified != access verified

RangeIndexFact is only an input:
  it does not itself prove FastMemory table length

TableIndex lowerable requires:
  table_length_resolved=1
  bounds_proof_valid=1
  stride_resolved=1
  field_offset_resolved=1
  overflow_proof_valid=1
  alignment_valid=1
  element_layout_verified=1

Lowering must not:
  query Type ABI
  dispatch through Provider ABI
  infer length/bounds by itself
  mark inbounds GEP without proof
```

## Worker Delegation Order

After design agreement:

```text
1. FMEM-TABLE-002A worker
   Add FastMemory table length fact carrier.
   Emit no facts unless explicit evidence exists.
   Teach access plans to consume injected/explicit facts.
   No bounds proof consumption yet.

2. FMEM-TABLE-002B worker
   Connect RangeIndexFact as BoundsProof::RangeFact only when:
     index_value matches TableIndex index
     upper_exclusive_value matches FastMemory table length fact
     step == 1
     end_exclusive == true
     index_body_read_only == true
     loop_carried_writes_supported == false

3. FMEM-TABLE-003 worker
   Add OverflowProof for index * stride + field_offset.

4. FMEM-TABLE-004 worker
   Add JSON/report/check rejection for incomplete proofs.

5. FMEM-TABLE-005 worker
   Open LLVM lowering only for fully verified table access.
```

## Stop Line

Do not implement `FMEM-TABLE-002` until the table length fact owner is accepted.

Specifically do not:

```text
set page_table length to an arbitrary constant
mark page_table lowerable
consume RangeIndexFact without a FastMemory length fact
choose one-level vs two-level page-map strategy
open LLVM TableIndex lowering
query Type ABI / Provider ABI / runtime provider for length
```
