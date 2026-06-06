---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-027.
Related:
  - docs/development/current/main/phases/phase-296x/296x-524-MIM-PORT-FMEM-026-REMOTE-FREE-SUBSTRATE-SELECTION.md
  - src/mir/instruction.rs
  - src/mir/builder/fastmem.rs
---

# 296x-525 MIM-PORT-FMEM-027 AtomicRemoteHead Vocabulary Source Preflight

## Purpose

Add the first source/MIR vocabulary for remote-owner free publication without
opening CAS lowering or route behavior.

Remote owner free must not use ordinary `remote_head` FieldStore. It requires a
dedicated FastMemory MemOp so verifier plans, memory ordering, and report
evidence can stay explicit.

## Planned Source Shape

```text
mem.atomicRemoteHeadPush(page, block)
```

## Acceptance

```text
MemOpKind has a transport-only AtomicRemoteHeadPush vocabulary entry
source fastmem can name mem.atomicRemoteHeadPush(page, block)
AST/MIR inventory reports atomic_remote_head_push count
MIR-to-LLVM producer fails closed with unsupported-kind or missing verified plan
no CAS lowering opens
remote route behavior remains closed
branch CFG lowering remains closed
TLS transfer remains closed
product activation / hook / global allocator / winner claims remain 0
```

## Still Closed

```text
AtomicRemoteHead verifier plans
AtomicRemoteHead LLVM CAS lowering
remote owner routing
fastmem branch CFG lowering
TLS backing transfer
process allocator replacement
hook installation
global allocator claim
winner claim
```
