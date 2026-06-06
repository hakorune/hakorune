---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-037.
Related:
  - docs/development/current/main/phases/phase-296x/296x-534-MIM-PORT-FMEM-036-ATOMIC-REMOTE-HEAD-RETRY-LOWERING-PRODUCER-PILOT.md
  - src/mir/fastmem_access_plan.rs
  - src/llvm_py/instructions/memop.py
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
---

# 296x-535 MIM-PORT-FMEM-037 AtomicRemoteHead Drain Preflight

## Purpose

Select and pin the first `AtomicRemoteHeadDrain` row after
`AtomicRemoteHeadPush` gained bounded retry lowering evidence.

This is a preflight row. It should make drain/exchange vocabulary and
report/check obligations explicit before changing remote-free behavior.

## Candidate Shape

```text
AtomicRemoteHeadDrain(page):
  list = atomic_exchange(remote_head, null, acquire)
  expose drain-selected evidence
  do not yet route drained nodes into local/free lists unless a later row opens it
```

## Still Closed

```text
remote-owner branch routing
same/remote free full body route
drain-to-local/free publication behavior
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
atomic_remote_head_drain_selected=1
atomic_remote_head_drain_open=0
atomic_remote_head_drain_lowered_count=0
remote_owner_branch_routing_open=0
atomic_remote_head_retry_policy_open=1
atomic_remote_head_retry_lowered_count=1
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
global_allocator_claim=0
winner_claim=0
```
