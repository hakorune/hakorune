---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-039.
Related:
  - docs/development/current/main/phases/phase-296x/296x-536-MIM-PORT-FMEM-038-ATOMIC-REMOTE-HEAD-DRAIN-EXCHANGE-SELECTION.md
  - src/mir/fastmem_access_plan.rs
  - src/llvm_py/instructions/memop.py
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
---

# 296x-537 MIM-PORT-FMEM-039 AtomicRemoteHead Drain Exchange Lowering Producer Pilot

## Purpose

Open the first `AtomicRemoteHeadDrain` lowering producer pilot after MIM-038
selected the exchange shape.

## Candidate Shape

```text
AtomicRemoteHeadDrain(page):
  old = atomic_exchange(page.remote_head, 0, acquire)
  result = remote_free_list_token(old)
```

## Required Boundaries

```text
drain-to-local/free routing remains closed
remote-owner branch routing remains closed
same/remote free full body route remains closed
TLS backing transfer remains closed
owner slot reuse remains closed
abandoned reclaim behavior remains closed
process allocator replacement remains closed
hook installation remains closed
global allocator claim remains closed
winner claim remains closed
full .hako mimalloc algorithm claim remains closed
```

## Acceptance Sketch

```text
fastmem_atomic_remote_head_drain_exchange_producer_pilot=1
atomic_remote_head_drain_exchange_selected=1
atomic_remote_head_drain_open=1
atomic_remote_head_drain_lowered_count>0
atomic_remote_head_drain_exchange_order=acquire
atomic_remote_head_drain_result_kind=remote_free_list_token
atomic_remote_head_drain_to_local_route_open=0
remote_owner_branch_routing_open=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
global_allocator_claim=0
winner_claim=0
```
