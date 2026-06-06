---
Status: Active
Date: 2026-06-06
Scope: MIR-FMEM-008C implementation handoff order after complete FastMemory TableIndex proof checks.
Related:
  - docs/development/current/main/phases/phase-296x/296x-472-FMEM-TABLE-004-INCOMPLETE-PROOF-CHECK.md
  - docs/development/current/main/phases/phase-296x/296x-460-VERIFIED-TABLE-ACCESS-PROOF-DECISION.md
  - docs/development/current/main/design/mir-fastmem-memop-dialect-ssot.md
  - docs/development/current/main/design/fastmem-layout-table-contract-v0-ssot.md
  - src/llvm_py/instructions/memop.py
  - src/llvm_py/builders/function_metadata.py
  - src/llvm_py/resolver.py
---

# 296x-473 MIR-FMEM-008C Handoff Order

## Decision

`FMEM-TABLE-004` closed the verifier/report side of complete TableIndex proof
evidence. The next implementation row is `MIR-FMEM-008C`, but it starts with a
lowering preflight before opening any new LLVM behavior.

Short form:

```text
proof complete:
  yes, from FastMemory verifier/report/check

LLVM lowering:
  open only after confirming the Python LLVM producer can consume verified
  access plans without recomputing layout/table facts
```

## Main-Line Order

### 1. `MIR-FMEM-008C-PRE`: LLVM producer preflight

Read and document the current state of:

```text
src/llvm_py/builders/function_metadata.py
src/llvm_py/resolver.py
src/llvm_py/instructions/memop.py
src/llvm_py/builders/instruction_lower.py
```

Acceptance:

```text
access-plan rows are available by function/block/instruction site
field_size / element_size are loaded if lowering needs them
lowerer has a clear way to find the matching verified plan
no Type ABI / Provider ABI query is introduced
```

Stop for design consultation if any of these are unclear:

```text
table operand pointer representation
TableIndex result value representation
pointer_to_element versus inline_element lowering shape
LLVM type needed for FieldLoad / FieldStore
where to record lowering evidence counters
```

### 2. `MIR-FMEM-008C-A`: verified TableIndex element-reference lowering

Open only complete `VerifiedTableAccess` rows.

Allowed:

```text
TableIndex with all proof bits complete becomes lowerable
PointerElementTable lowers as slot GEP + pointer load
InlineElementTable lowers as element GEP only if contract says inline_element
no inbounds GEP unless bounds and overflow proof are complete
```

Forbidden:

```text
lowering table-length-unresolved rows
lowering incomplete proof rows
asking Type ABI / Provider ABI for table metadata
guessing element representation from operands
calling the Python-template C bridge
```

### 3. `MIR-FMEM-008C-B`: linked FieldLoad / FieldStore lowering

Open field access only when the base is a verifier-linked element reference.

Allowed:

```text
GEP/load/store from verified field byte_offset / field type / alignment
owner_id compatibility alias is already canonicalized to owner_worker_id
```

Forbidden:

```text
field-only lowering that secretly lowers page_table[index].field without a
VerifiedTableAccess
plain FieldStore(remote_head)
lowerer-side field offset recomputation
```

### 4. `MIR-FMEM-008C-C`: producer-neutral report/check evidence

Add or extend report fields only after the behavior is real.

Expected stop-line evidence:

```text
fastmem_lowering_recomputed_layout_offset_count=0
fastmem_table_index_unchecked_count=0
fastmem_atomic_field_plain_store_count=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Worker Handoff Order

Use workers for sidecar inventory or disjoint patches. Keep the immediate
lowering seam local unless the worker owns a whole subrow.

### Worker A: LLVM Producer Inventory

Task:

```text
Inspect the LLVM producer metadata and MemOp lowering path.
Report whether function metadata loads every field needed by complete
VerifiedTableAccess lowering.
```

Read-only files:

```text
src/llvm_py/builders/function_metadata.py
src/llvm_py/resolver.py
src/llvm_py/instructions/memop.py
src/llvm_py/builders/instruction_lower.py
src/llvm_py/tests/
```

Output:

```text
missing metadata fields
existing test hooks
design-stop risks
```

### Worker B: Report/Check Gap Inventory

Task:

```text
Inspect hako_check / report schema for existing lowering counters and identify
the smallest fields needed for MIR-FMEM-008C-C.
```

Read-only files:

```text
tools/hako_check/
tools/allocator/
docs/development/current/main/phases/phase-296x/*FMEM*
```

Output:

```text
fields that already exist
fields that should be added only after behavior exists
smoke scripts that can be extended instead of adding new scripts
```

### Worker C: Verifier BoxShape Sidecar

Task:

```text
Optional after the next lowering slice: verifier traversal helper / test
fixture cleanup inventory.
```

Allowed only as BoxShape:

```text
no new verification rule
no accepted shape change
no TableIndex proof vocabulary change
```

Preferred after:

```text
MIR-FMEM-008C-A or 008C-B lands
```

### Worker D: Design-Consult Pack

Task:

```text
If preflight finds ambiguous pointer/value representation, prepare the
ChatGPT Pro consultation pack.
```

The pack should ask about:

```text
TableIndex result representation in Python LLVM lowering
VerifiedTableAccess to LLVM GEP/load/store mapping
whether FieldLoad/Store should lower in the same row or after element-ref
whether report counters belong in the lowerer or hako_check adapter first
```

## Local Owner

The main implementer owns:

```text
preflight decision
MIR-FMEM-008C card creation
any design stop
the first behavior-changing lowering row
```

Do not delegate the critical path if the next step depends on the answer.

## No-Go

```text
mix verifier cleanup with lowering behavior
lower incomplete TableIndex proofs
recompute layout/table facts in the lowerer
turn Python-template C bridge back into a hidden execution fallback
open CurrentAllocOwnerId / OwnerEq in this row
open AtomicRemoteHead / TLS transfer / owner slot reuse
open product activation, hook install, global allocator, or winner claim
```

## Next

```text
MIR-FMEM-008C-PRE:
  inspect the LLVM producer seam and stop for design consultation if pointer
  representation or lowering evidence placement is ambiguous.
```
