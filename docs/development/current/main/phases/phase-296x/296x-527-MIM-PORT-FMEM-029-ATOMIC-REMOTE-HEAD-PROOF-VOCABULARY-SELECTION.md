---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-029.
Related:
  - docs/development/current/main/phases/phase-296x/296x-526-MIM-PORT-FMEM-028-ATOMIC-REMOTE-HEAD-VERIFIER-PRECONDITIONS.md
  - src/mir/fastmem_access_plan.rs
  - src/mir/builder/fastmem.rs
---

# 296x-527 MIM-PORT-FMEM-029 AtomicRemoteHead Proof Vocabulary Selection

## Purpose

Select the next proof vocabulary needed before AtomicRemoteHead CAS lowering.

MIM-028 made `AtomicRemoteHeadPush` visible as a rejected verifier-owned
precondition row. The current gap is not code generation; it is the proof
surface that can prove a remote-owner publication route and a writable
remote-free block-next field without using ordinary `remote_head` FieldStore.

## Options

```text
A. Remote-owner proof first
   Add a source/MIR fact that proves page.owner != current AllocOwnerId for
   AtomicRemoteHeadPush.

B. Remote-free block-next proof first
   Add a source/MIR fact dedicated to remote-free publication nodes instead of
   reusing local/free-head block-next assumptions.

C. Pair both as a single proof vocabulary row
   Add both facts together, but keep AtomicRemoteHeadPush lowerable=0 until the
   next LLVM CAS row.
```

## Selection Criteria

```text
must:
  keep AtomicRemoteHead LLVM CAS lowering closed
  keep remote owner routing behavior closed
  keep fastmem branch CFG lowering closed
  avoid ordinary remote_head FieldStore
  report missing proof counts independently

prefer:
  one durable proof vocabulary slice
  source syntax that reads like a remote-free publication contract
  MIR facts that later lowering can consume without re-deciding owner policy
```

## Decision

```text
selected:
  C. Pair both as a single proof vocabulary row

next:
  MIM-PORT-FMEM-030 AtomicRemoteHead proof vocabulary source preflight
```

AtomicRemoteHead publication needs both facts to avoid a misleading half-open
route:

```text
remote-owner proof:
  proves the page is not same-owner local mutation

remote-free block-next proof:
  proves the block node can carry the remote publication link
```

The next implementation row should add source/MIR fact vocabulary for both and
keep `AtomicRemoteHeadPush` `lowerable=0`. CAS lowering and remote branch
routing remain deferred until a later producer row consumes those facts.

## Still Closed

```text
AtomicRemoteHead LLVM CAS lowering
remote owner branch routing
TLS backing transfer
process allocator replacement
hook installation
global allocator claim
winner claim
```
