---
Status: Active
Date: 2026-06-06
Scope: FastMemory/TableIndex next-task order and safe commonization handoff.
Related:
  - docs/development/current/main/design/contract-region-v0-ssot.md
  - docs/development/current/main/design/mir-fastmem-memop-dialect-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-460-VERIFIED-TABLE-ACCESS-PROOF-DECISION.md
  - docs/development/current/main/phases/phase-296x/296x-469-FMEM-TABLE-003A-FIELD-OFFSET-LINK.md
  - src/mir/verification/fastmem.rs
  - src/mir/escape_barrier.rs
---

# 296x-470 FMEM Handoff / Commonization Order

## Decision

Keep the FastMemory proof/lowering lane on the main line. Treat verifier
commonization as BoxShape sidecar work, not as a prerequisite for the next
TableIndex proof row.

Short form:

```text
main line:
  finish the concrete memory proof row

sidecar:
  clean duplicated verifier mechanics without adding accepted shapes
```

## Current Main Line

The current TableIndex proof chain is:

```text
FMEM-TABLE-002A:
  FastMemTableLengthFact carrier

FMEM-TABLE-002B:
  RangeIndexFact consumed as bounds proof only after a matching length fact

FMEM-TABLE-003A:
  TableIndex.result -> FieldLoad/Store.base link supplies field_offset_resolved
```

Still closed:

```text
overflow_proof_valid
TableIndex lowerable
LLVM TableIndex lowering
product activation / hook install / global allocator claim / winner claim
```

## Next Main Tasks

1. `FMEM-TABLE-003B`: add overflow proof from:

```text
index
stride
linked field_offset
field access size
table length / bounds proof
target usize width
```

Acceptance:

```text
overflow_proof_valid=1 only when mul-add and object-range proof both hold
TableIndex remains lowerable=0 until every proof bit is complete
no Type ABI / Provider ABI query
no lowering
```

2. `FMEM-TABLE-004`: add JSON/report/check rejection for incomplete table
   access proofs.

Acceptance:

```text
missing length / bounds / field-offset / overflow remains visible
fastmem_table_index_unchecked_count=0
fastmem_unverified_layout_access_count=0
```

3. `MIR-FMEM-008C`: open LLVM lowering only for complete
   `VerifiedTableAccess`.

Acceptance:

```text
lowerer consumes verified plan only
lowerer recomputed layout/table facts count = 0
inbounds GEP only when all required proof bits are true
```

## Worker Handoff Order

Use workers only on disjoint BoxShape sidecars. Do not delegate the immediate
TableIndex overflow proof unless the worker owns the whole row and its tests.

Recommended order:

```text
Main implementer:
  FMEM-TABLE-003B overflow proof

Worker A:
  verifier traversal utility inventory / patch
  owned files: src/mir/verification/utils.rs plus narrow verifier call sites
  constraint: behavior-preserving only, no new verification rule

Worker B:
  FastMem single-input PHI alias cleanup
  owned files: src/mir/verification/fastmem.rs plus existing phi helper imports
  constraint: reuse existing passthrough-PHI/value-origin helper concepts only

Worker C:
  MIR verifier test fixture helper inventory / patch
  owned files: verifier test support only
  constraint: test code cleanup only

Worker D:
  escape-analysis design note
  owned files: docs only unless explicitly promoted
  constraint: do not replace fastmem no-escape gates yet
```

## Commonization Assessment

### Already Commonized

`MirInstruction::extern_name()` already exists:

```text
src/mir/instruction/methods.rs
```

New verifier code should use that method instead of local `extern_name`
helpers.

FastMemory escape checking already reuses the shared escape-barrier classifier:

```text
src/mir/escape_barrier.rs
src/mir/verification/fastmem.rs
src/mir/passes/escape.rs
```

The current FastMemory verifier adds memory-profile rules on top of the shared
classifier. That is the right shape for now.

Basic instruction iteration also has partial support already:

```text
BasicBlock::all_spanned_instructions()
BasicBlock::all_spanned_instructions_enumerated()
MirFunction::block_ids()
```

The missing piece is not another instruction iterator on `BasicBlock`; it is a
small verifier-side helper that uses deterministic function block order and
passes block id, instruction index, and spanned instruction together.

### Worth Doing Soon

Verifier traversal helper:

```text
for each block
  for each spanned instruction
```

This pattern appears in several verifier modules. A small helper or walker is
reasonable when it remains behavior-preserving and keeps block id,
instruction index, span, and instruction together.

Start with the clearest call sites:

```text
src/mir/verification/ssa.rs
src/mir/verification/barrier.rs
src/mir/verification/fastmem.rs
```

Do not force indexed neighbor-window checks into the helper if it makes them
less readable.

FastMem single-input PHI aliasing:

```text
src/mir/verification/fastmem.rs currently carries local alias propagation
```

This may be aligned with the existing passthrough-PHI helper concepts used by
escape/barrier analysis, but only as behavior-preserving BoxShape cleanup.
Keep FastMemory's escape policy in the FastMemory verifier.

Test fixture helper:

```text
create small MirFunction / BasicBlock / metadata fixtures
```

This should be introduced only in test support files. Do not let fixture
helpers become hidden semantic defaults.

### Defer

Generic `AllowlistGate<Domain>`:

```text
MemOpKind allowlist
MIR instruction kept tags
backend support gates
call dialect gates
```

These are similar shapes but different domains. Keep their SSOT tables
separate until duplication causes a real bug.

Owner concept unification:

```text
AllocOwnerId
Page owner
language "1 box = 1 owner"
```

These names ask a similar question, but they live in different layers and have
different invariants. Keep them separate; cross-link docs only.

Full generic escape-analysis merge:

```text
src/mir/passes/escape.rs:
  optimization pass for removable local-box barriers

src/mir/verification/fastmem.rs:
  fail-fast verifier contract for MemOp result escape
```

They share escape-barrier vocabulary, but their outputs and responsibilities
are different. Do not merge them while FMEM-TABLE-003B is the main line.

Broad verifier test fixture cleanup:

```text
local test helpers are currently domain-shaped
```

Extract only when adding the next verifier tests would otherwise duplicate the
same fixture construction again.

## DirectArray Boundary

Do not merge FastMemory `TableIndex` proof machinery with DirectArray access
plans.

Commonize only:

```text
ProofEnvelopeV0-style status fields
RangeIndexFact as one possible bounds proof input
```

Keep separate:

```text
DirectArrayExtentFact
DirectArrayAccessPlan
FastMemTableLengthFact
FastMemTableAccessProof
```

`DirectArray` is collection/receiver capacity semantics. `TableIndex` is a
FastMemory table/layout access proof.

## No-Go

```text
mix BoxShape verifier cleanup with FMEM-TABLE-003B proof semantics
make TableIndex lowerable before overflow proof
turn shared escape classifier into broad lifetime analysis in this row
genericize allowlist gates before a concrete second use needs it
rename FastMemRegion to ContractRegion across the repo now
```
