---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-045.
Related:
  - docs/development/current/main/phases/phase-296x/296x-542-MIM-PORT-FMEM-044-ATOMIC-REMOTE-HEAD-DRAIN-LOCAL-LIST-MUTATION-VOCABULARY-PREFLIGHT.md
  - src/mir/fastmem_access_plan.rs
  - src/mir/verification/fastmem.rs
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-543 MIM-PORT-FMEM-045 AtomicRemoteHead Drain Local List Mutation Verifier Preconditions

## Purpose

Add verifier-owned preconditions for the dedicated owner-local list mutation
operation:

```text
mem.drainRemoteListToLocal(page, remote_free_list_token)
```

This row should prove that the mutation operation consumes a
`remote_free_list_token` produced by `mem.atomicRemoteHeadDrain(page)` in the
same fastmem region and targets a verified owner-local list head class. It must
not open producer lowering yet.

## Required Boundaries

```text
local list mutation lowering remains closed
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
fastmem_atomic_remote_head_drain_local_list_mutation_verifier_preconditions=1
fastmem_memop_drain_remote_list_to_local_count=1
drain_remote_list_to_local_plan_count=1
drain_remote_list_to_local_token_provenance_valid=1
drain_remote_list_to_local_page_operand_valid=1
drain_remote_list_to_local_head_class_resolved=1
drain_remote_list_to_local_lowerable_count=0
atomic_remote_head_drain_local_list_mutation_open=0
remote_owner_branch_routing_open=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
global_allocator_claim=0
winner_claim=0
```

## Non-goals

```text
LLVM lowering for DrainRemoteListToLocal
remote-owner branch CFG routing
same/remote free full route body
AtomicRemoteHead remote-heavy benchmark claim
TLS backing transfer
allocator activation
```
