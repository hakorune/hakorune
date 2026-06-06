---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-046.
Related:
  - docs/development/current/main/phases/phase-296x/296x-543-MIM-PORT-FMEM-045-ATOMIC-REMOTE-HEAD-DRAIN-LOCAL-LIST-MUTATION-VERIFIER-PRECONDITIONS.md
  - src/mir/fastmem_access_plan.rs
  - src/llvm_py/instructions/memop.py
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
---

# 296x-544 MIM-PORT-FMEM-046 AtomicRemoteHead Drain Local List Mutation Lowering Producer Pilot

## Purpose

Open the first producer-side lowering pilot for:

```text
mem.drainRemoteListToLocal(page, remote_free_list_token)
```

The lowering must consume only the MIM-045 verifier-owned
`DrainRemoteListToLocal` plan. It may mutate an owner-local list head according
to the verified plan, but it must not open remote-owner branch routing or a full
same/remote free body.

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
fastmem_atomic_remote_head_drain_local_list_mutation_lowering_producer_pilot=1
replacement_front_selected_memop_kinds=DrainRemoteListToLocal
drain_remote_list_to_local_plan_count=1
drain_remote_list_to_local_token_provenance_valid=1
drain_remote_list_to_local_page_operand_valid=1
drain_remote_list_to_local_head_class_resolved=1
drain_remote_list_to_local_lowerable_count=1
atomic_remote_head_drain_local_list_mutation_lowered_count=1
atomic_remote_head_drain_local_list_mutation_open=1
remote_owner_branch_routing_open=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
global_allocator_claim=0
winner_claim=0
```

## Non-goals

```text
remote-owner branch CFG routing
same/remote free full route body
remote-heavy benchmark claim
TLS backing transfer
allocator activation
```
