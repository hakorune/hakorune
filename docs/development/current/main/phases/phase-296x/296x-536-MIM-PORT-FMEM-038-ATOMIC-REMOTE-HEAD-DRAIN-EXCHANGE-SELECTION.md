---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-038.
Related:
  - docs/development/current/main/phases/phase-296x/296x-535-MIM-PORT-FMEM-037-ATOMIC-REMOTE-HEAD-DRAIN-PREFLIGHT.md
  - src/mir/fastmem_access_plan.rs
  - src/llvm_py/instructions/memop.py
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
---

# 296x-536 MIM-PORT-FMEM-038 AtomicRemoteHead Drain Exchange Selection

## Purpose

Select the exact producer slice for lowering `AtomicRemoteHeadDrain` after
MIM-037 made the drain vocabulary and rejected access-plan row visible.

This row should still be a preflight/selection row unless it explicitly opens a
dedicated producer pilot.

## Candidate Shape

```text
AtomicRemoteHeadDrain(page):
  list = atomic_exchange(remote_head, 0, acquire)
  result kind = remote_free_list_token / pointer-sized scalar
```

## Still Closed

```text
drain-to-local/free routing
remote-owner branch routing
same/remote free full body route
TLS backing transfer
owner slot reuse
abandoned reclaim behavior
process allocator replacement
hook installation
global allocator claim
winner claim
full .hako mimalloc algorithm claim
```

## Acceptance Sketch

```text
atomic_remote_head_drain_exchange_selected=1
atomic_remote_head_drain_open=0
atomic_remote_head_drain_lowered_count=0
atomic_remote_head_drain_to_local_route_open=0
remote_owner_branch_routing_open=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
global_allocator_claim=0
winner_claim=0
```
