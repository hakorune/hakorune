---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-042.
Related:
  - docs/development/current/main/phases/phase-296x/296x-539-MIM-PORT-FMEM-041-ATOMIC-REMOTE-HEAD-DRAIN-TO-LOCAL-ROUTE-PRODUCER-PILOT.md
  - src/llvm_py/instructions/memop.py
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
---

# 296x-540 MIM-PORT-FMEM-042 AtomicRemoteHead Drain Local List Mutation Preflight

## Purpose

Select the first proof row needed before mutating owner-local lists with the
`remote_free_list_token` produced by `AtomicRemoteHeadDrain(page)`.

## Candidate Shape

```text
remote = atomic_exchange(page.remote_head, 0, acquire)
token = remote_free_list_token(remote)
route = owner-local drain route

preflight:
  prove token can be appended/prepended to owner-local list state
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
fastmem_atomic_remote_head_drain_local_list_mutation_preflight=1
atomic_remote_head_drain_to_local_route_open=1
atomic_remote_head_drain_local_list_mutation_selected=1
atomic_remote_head_drain_local_list_mutation_open=0
remote_owner_branch_routing_open=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
global_allocator_claim=0
winner_claim=0
```

This row should not mutate owner-local lists yet unless the proof surface is
already strong enough. If needed, split token shape, list head class, and
publication order into smaller preflight rows.

## Landed

MIM-042 selects the local-list mutation preflight for the
`remote_free_list_token` emitted by `AtomicRemoteHeadDrain(page)`. The route
evidence from MIM-041 remains open, but owner-local list mutation itself stays
closed.

## Evidence

```text
fastmem_atomic_remote_head_drain_local_list_mutation_preflight=1
replacement_front_next_producer_slice=atomic_remote_head_drain_local_list_mutation_proof
atomic_remote_head_drain_to_local_route_open=1
atomic_remote_head_drain_local_list_mutation_selected=1
atomic_remote_head_drain_local_list_mutation_open=0
remote_owner_branch_routing_open=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
global_allocator_claim=0
winner_claim=0
```

Verification:

```text
python3 -m py_compile tools/hako_check/fastmem_check.py tools/hako_check/fastmem_mir_to_llvm_producer_report.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Next

MIM-PORT-FMEM-043 should define the proof surface for turning a drained
`remote_free_list_token` into owner-local list mutation eligibility. It must
still keep remote-owner branch routing, TLS transfer, abandoned reclaim,
product activation, hooks, global allocator claim, and winner claim closed.
