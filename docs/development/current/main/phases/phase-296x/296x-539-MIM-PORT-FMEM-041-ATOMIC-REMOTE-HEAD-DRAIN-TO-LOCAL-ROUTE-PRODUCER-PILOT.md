---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-041.
Related:
  - docs/development/current/main/phases/phase-296x/296x-538-MIM-PORT-FMEM-040-ATOMIC-REMOTE-HEAD-DRAIN-TO-LOCAL-ROUTE-SELECTION.md
  - src/llvm_py/instructions/memop.py
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
---

# 296x-539 MIM-PORT-FMEM-041 AtomicRemoteHead Drain-To-Local Route Producer Pilot

## Purpose

Open the first producer pilot for consuming the `remote_free_list_token` emitted
by `AtomicRemoteHeadDrain(page)`.

## Candidate Shape

```text
remote = atomic_exchange(page.remote_head, 0, acquire)
token = remote_free_list_token(remote)

producer pilot:
  token -> owner-local drain evidence
```

## Required Boundaries

```text
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
fastmem_atomic_remote_head_drain_to_local_route_producer_pilot=1
atomic_remote_head_drain_open=1
atomic_remote_head_drain_lowered_count>0
atomic_remote_head_drain_to_local_route_selected=1
atomic_remote_head_drain_to_local_route_open=1
remote_owner_branch_routing_open=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
global_allocator_claim=0
winner_claim=0
```

This row should be conservative. If list mutation needs a separate
precondition/proof row, split it before opening behavior.
