---
Status: Done
Date: 2026-06-06
Scope: add MIR FastMemory MemOp vocabulary to the contracts surface without opening transport or lowering.
Blocker: MIR-FMEM-002
Related:
  - docs/development/current/main/design/mir-fastmem-memop-dialect-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-437-MIRBUILDER-FASTMEM-MEMOP-DIALECT-DECISION.md
  - src/mir/instruction.rs
  - src/mir/contracts/backend_core_ops.rs
  - src/mir/contracts/fastmem_ops.rs
  - docs/reference/mir/INSTRUCTION_SET.md
---

# 296x-438 MIR Contracts FastMem MemOp Vocabulary

## Purpose

`MIR-FMEM-001` accepted the representation boundary. This row adds the first
code-side vocabulary surface:

```text
MirInstruction::MemOp
FastMemRegionId
MemOpKind
src/mir/contracts/fastmem_ops.rs
```

This is a vocabulary and contract row only.

## Decision

```text
memop_instruction_tag=MemOp
fastmem_v0_memop_kind_allowlist=1
fastmem_v0_memop_kind_count=10

mir_json_memop_supported=0
vm_memop_supported=0
llvm_json_memop_supported=0
llvm_native_memop_supported=0
c_artifact_memop_supported=0
```

`MemOp` is part of the kept MIR instruction vocabulary so later rows can attach
JSON, verifier, and lowering behavior to one instruction tag. It is not yet
accepted by MIR JSON, VM, or LLVM lowerers.

## Added V0 MemOpKind

```text
AddrOf
LogicalShr
BitAnd
Add
Sub
TableIndex
FieldLoad
FieldStore
CurrentAllocOwnerId
OwnerEq
```

Atomic MemOps remain closed for this row.

## Acceptance

```bash
cargo check -q
cargo test -q mir::contracts
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Expected behavior:

```text
instruction_tag(MemOp)=MemOp
instruction_diet_cohort(MemOp)=Kept
is_supported_mir_json_instruction(MemOp)=0
is_supported_vm_instruction(MemOp)=0
llvm_json_ops_for_instruction(MemOp)=[]
```

## Stop Line

- do not emit MemOp from MIRBuilder in this row
- do not add FastMemRegion side-table storage in this row
- do not serialize MemOp to MIR JSON in this row
- do not execute MemOp in VM in this row
- do not lower MemOp to LLVM/C in this row
- do not add atomic MemOps in this row
- do not open product allocator activation

## Follow-Up

```text
MIR-FMEM-003:
  MIRBuilder source lowering to FastMemRegion/MemOp metadata.
```
