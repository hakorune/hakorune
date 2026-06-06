---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-043.
Related:
  - docs/development/current/main/phases/phase-296x/296x-540-MIM-PORT-FMEM-042-ATOMIC-REMOTE-HEAD-DRAIN-LOCAL-LIST-MUTATION-PREFLIGHT.md
  - src/mir/fastmem_access_plan.rs
  - src/llvm_py/instructions/memop.py
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
---

# 296x-541 MIM-PORT-FMEM-043 AtomicRemoteHead Drain Local List Mutation Proof

## Purpose

Define the verifier/report proof surface required before a drained
`remote_free_list_token` may mutate owner-local list state.

## Candidate Shape

```text
remote = atomic_exchange(page.remote_head, 0, acquire)
token = remote_free_list_token(remote)

proof:
  token is non-escaping
  target owner-local list head class is resolved
  publication order remains verifier-owned
  mutation is still closed
```

## Required Boundaries

```text
local list mutation remains closed until the proof row lands
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
fastmem_atomic_remote_head_drain_local_list_mutation_proof=1
atomic_remote_head_drain_to_local_route_open=1
atomic_remote_head_drain_local_list_mutation_selected=1
atomic_remote_head_drain_local_list_mutation_open=0
atomic_remote_head_drain_local_list_token_escape_count=0
atomic_remote_head_drain_local_list_head_class_resolved=1
remote_owner_branch_routing_open=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
global_allocator_claim=0
winner_claim=0
```

This row should not perform the owner-local list mutation. If mutation requires
a dedicated MemOp, split vocabulary/source observation before producer lowering.
