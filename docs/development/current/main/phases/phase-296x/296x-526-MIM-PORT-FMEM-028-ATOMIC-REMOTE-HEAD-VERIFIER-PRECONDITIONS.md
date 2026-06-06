---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-028.
Related:
  - docs/development/current/main/phases/phase-296x/296x-525-MIM-PORT-FMEM-027-ATOMIC-REMOTE-HEAD-VOCABULARY-SOURCE-PREFLIGHT.md
  - src/mir/fastmem_access_plan.rs
---

# 296x-526 MIM-PORT-FMEM-028 AtomicRemoteHead Verifier Preconditions

## Purpose

Create verifier-owned AtomicRemoteHeadPush access-plan rows without opening CAS
lowering.

MIM-027 made the source/MIR vocabulary visible. The next row should make the
missing preconditions explicit so remote-free behavior can be opened later
through proof rows rather than ordinary `remote_head` FieldStore.

## Planned Evidence

```text
atomic_remote_head_push_plan_count
atomic_remote_head_push_lowerable_count=0
atomic_remote_head_remote_owner_required=1
atomic_remote_head_remote_owner_missing_count
atomic_remote_head_block_next_required=1
atomic_remote_head_block_next_missing_count
atomic_remote_head_memory_order_policy=<closed|acq_rel>
```

## Still Closed

```text
AtomicRemoteHead LLVM CAS lowering
remote owner routing
fastmem branch CFG lowering
TLS backing transfer
process allocator replacement
hook installation
global allocator claim
winner claim
```
