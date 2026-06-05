---
Status: Done
Date: 2026-06-06
Scope: lock the MIRBuilder FastMemRegion/MemOp representation boundary before implementing FastMemory execution lowering.
Blocker: MIR-FMEM-001
Related:
  - docs/development/current/main/design/mir-fastmem-memop-dialect-ssot.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - src/mir/contracts/README.md
---

# 296x-437 MIRBuilder FastMem MemOp Dialect Decision

## Purpose

`MIR-FMEM-001` turns the design consultation result into the current MIR
boundary before code is added.

The key risk was representing `fastmem` regions as executable begin/end marker
instructions. That would pollute CFG, JSON, VM, and LLVM allowlists with
non-executable tags. This row rejects that shape.

## Decision

```text
MirInstruction::MemOp:
  single executable MIR instruction for fast memory dialect operations

MemOpKind:
  dialect vocabulary

FastMemRegion:
  metadata side table / region truth

MemOp.region:
  carries FastMemRegionId
```

Rejected:

```text
FastMemRegionBegin as a normal MIR instruction
FastMemRegionEnd as a normal MIR instruction
MIRBuilder choosing page-map strategy
MIRBuilder choosing C vs LLVM
MIRBuilder choosing fast vs slow route
MIRBuilder making product activation / keeper claims
```

## V0 Scope

Initial `MemOpKind` vocabulary stays narrow:

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

Atomic operations remain later work.

## Acceptance

```text
design SSOT added:
  docs/development/current/main/design/mir-fastmem-memop-dialect-ssot.md

capability gap SSOT updated:
  old FastMemRegionBegin/End plan is replaced by side-table metadata decision

behavior_change=0
mir_enum_change=0
json_change=0
verifier_change=0
lowering_change=0
product_activation=0
hook_installed=0
global_allocator_product_claim=0
winner_claim=0
```

## Follow-Up

```text
MIR-FMEM-002:
  add `MemOp` to mir/contracts instruction vocabulary and create a MemOpKind
  allowlist surface.
```

## Stop Line

- do not implement MemOp lowering in this row
- do not add atomic MemOps in this row
- do not add route/backend selection to MIRBuilder
- do not retire the Python template C bridge in this row
- do not open product allocator activation
