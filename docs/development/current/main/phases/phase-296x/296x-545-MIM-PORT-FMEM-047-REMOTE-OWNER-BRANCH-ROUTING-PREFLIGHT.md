---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-047.
Related:
  - docs/development/current/main/phases/phase-296x/296x-544-MIM-PORT-FMEM-046-ATOMIC-REMOTE-HEAD-DRAIN-LOCAL-LIST-MUTATION-LOWERING-PRODUCER-PILOT.md
  - src/mir/fastmem_access_plan.rs
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-545 MIM-PORT-FMEM-047 Remote-Owner Branch Routing Preflight

## Purpose

Select the next producer slice after owner-local remote-list drain mutation:
remote-owner branch routing preflight.

The row should make the intended same-owner / remote-owner split observable
without opening a full free body, TLS backing transfer, abandoned reclaim, or
allocator activation. The output is route evidence and fail-fast gates, not a
product allocator claim.

## Required Boundaries

```text
remote-owner branch routing may be selected/preflighted
remote-owner branch lowering remains closed unless explicitly opened by a later card
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
replacement_front_selected_route=remote_owner_branch_routing_preflight
replacement_front_next_producer_slice=remote_owner_branch_routing
remote_owner_branch_routing_selected=1
remote_owner_branch_routing_open=0
atomic_remote_head_drain_local_list_mutation_lowered_count>=1
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Non-goals

```text
branch CFG lowering
same/remote free full route body
remote-heavy benchmark claim
TLS backing transfer
abandoned reclaim
allocator activation
```
