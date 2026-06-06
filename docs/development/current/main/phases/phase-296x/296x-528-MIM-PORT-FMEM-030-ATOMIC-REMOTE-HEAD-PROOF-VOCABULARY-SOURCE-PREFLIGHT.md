---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-030.
Related:
  - docs/development/current/main/phases/phase-296x/296x-527-MIM-PORT-FMEM-029-ATOMIC-REMOTE-HEAD-PROOF-VOCABULARY-SELECTION.md
  - src/mir/builder/fastmem.rs
  - src/mir/function/types.rs
  - src/mir/fastmem_access_plan.rs
---

# 296x-528 MIM-PORT-FMEM-030 AtomicRemoteHead Proof Vocabulary Source Preflight

## Purpose

Add source/MIR fact vocabulary for AtomicRemoteHead publication preconditions
without opening CAS lowering.

MIM-029 selected a paired proof vocabulary row. This row should make both
requirements observable in `.hako` source and MIR metadata:

```text
remote-owner proof:
  page is not same-owner local mutation

remote-free block-next proof:
  block.next can be used as the remote publication link
```

## Planned Evidence

```text
atomic_remote_head_push_plan_count=1
atomic_remote_head_push_lowerable_count=0
atomic_remote_head_remote_owner_required=1
atomic_remote_head_remote_owner_missing_count=0
atomic_remote_head_block_next_required=1
atomic_remote_head_block_next_missing_count=0
atomic_remote_head_memory_order_policy=closed
```

## Candidate Source Intrinsics

```text
mem.assumeRemoteOwner(page)
mem.assumeRemoteFreeBlockNext(block)
```

The exact names may change during implementation, but the resulting facts must
remain AtomicRemoteHead-specific. Do not reuse same-owner local-free facts for
remote publication.

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
