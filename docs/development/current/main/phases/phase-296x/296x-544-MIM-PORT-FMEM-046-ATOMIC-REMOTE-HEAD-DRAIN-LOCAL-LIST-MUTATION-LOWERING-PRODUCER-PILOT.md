---
Status: Done
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

## Closeout

`DrainRemoteListToLocal` is now a lowerable verified access-plan row. The plan
resolves the owner-local `local_free_head` target and the
`FreeBlockNodeLayoutV0.next` link used to splice the drained remote list.

The MIR-to-LLVM producer consumes only that verified plan. It lowers
`mem.drainRemoteListToLocal(page, remote_free_list_token)` by treating an empty
token as a no-op, otherwise walking the drained remote list to its tail,
linking that tail to the previous owner-local head, and publishing the token as
the new `local_free_head`.

Remote-owner branch routing, TLS backing transfer, owner slot reuse, abandoned
reclaim behavior, product activation, hooks, global allocator claim, winner
claim, and full `.hako` mimalloc algorithm claim remain closed.

## Verification

```text
python3 -m py_compile \
  tools/hako_check/fastmem_check.py \
  tools/hako_check/fastmem_mir_to_llvm_producer_report.py \
  tools/hako_check/fastmem_capability_inventory_common.py \
  src/llvm_py/instructions/memop.py
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
cargo test -q --lib fastmem_
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
