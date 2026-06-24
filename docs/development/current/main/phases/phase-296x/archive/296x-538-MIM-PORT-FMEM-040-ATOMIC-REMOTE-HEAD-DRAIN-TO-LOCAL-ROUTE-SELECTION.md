---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-040.
Related:
  - docs/development/current/main/phases/phase-296x/296x-537-MIM-PORT-FMEM-039-ATOMIC-REMOTE-HEAD-DRAIN-EXCHANGE-LOWERING-PRODUCER-PILOT.md
  - src/llvm_py/instructions/memop.py
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
---

# 296x-538 MIM-PORT-FMEM-040 AtomicRemoteHead Drain-To-Local Route Selection

## Purpose

Select the next slice after `AtomicRemoteHeadDrain(page)` lowers to an acquire
exchange and produces a `remote_free_list_token`.

## Candidate Shape

```text
remote = atomic_exchange(page.remote_head, 0, acquire)
token = remote_free_list_token(remote)

next selected route:
  token -> owner-local drain candidate
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
fastmem_atomic_remote_head_drain_to_local_route_selection=1
atomic_remote_head_drain_open=1
atomic_remote_head_drain_lowered_count>0
atomic_remote_head_drain_to_local_route_selected=1
atomic_remote_head_drain_to_local_route_open=0
remote_owner_branch_routing_open=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
global_allocator_claim=0
winner_claim=0
```

This row should be selection/report/check only unless the route requires a
smaller preflight. It must not mutate local/free lists yet.

## Landed

MIM-040 selects the next route after `AtomicRemoteHeadDrain(page)` exchange
lowering. The selected next slice is:

```text
remote_free_list_token -> owner-local drain route producer pilot
```

This row is report/check selection only. It does not mutate `local_free`,
`free_head`, or any owner-local list.

## Evidence

```text
fastmem_atomic_remote_head_drain_to_local_route_selection=1
replacement_front_next_producer_slice=atomic_remote_head_drain_to_local_route_producer_pilot
atomic_remote_head_drain_open=1
atomic_remote_head_drain_lowered_count=1
atomic_remote_head_drain_to_local_route_selected=1
atomic_remote_head_drain_to_local_route_open=0
remote_owner_branch_routing_open=0
```

Verification:

```text
python3 -m py_compile tools/hako_check/fastmem_check.py tools/hako_check/fastmem_mir_to_llvm_producer_report.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Next

MIM-PORT-FMEM-041 should open the first producer pilot for consuming a
`remote_free_list_token` into owner-local drain evidence. It must keep
remote-owner branch routing, TLS transfer, abandoned reclaim, product
activation, hooks, global allocator claim, and winner claim closed.
