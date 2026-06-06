---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-048.
Related:
  - docs/development/current/main/phases/phase-296x/296x-545-MIM-PORT-FMEM-047-REMOTE-OWNER-BRANCH-ROUTING-PREFLIGHT.md
  - src/mir/fastmem_access_plan.rs
  - src/llvm_py/instructions/memop.py
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-546 MIM-PORT-FMEM-048 Remote-Owner Branch Routing Lowering Preflight

## Purpose

Prepare the first lowering row for remote-owner branch routing after MIM-047
selected the route. This row should define the verifier/report obligations for
opening branch routing, but should not yet lower a full same/remote free body.

## Required Boundaries

```text
remote-owner branch lowering may be selected/preflighted
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
replacement_front_selected_route=remote_owner_branch_routing_lowering_preflight
remote_owner_branch_routing_selected=1
remote_owner_branch_routing_open=0
remote_owner_branch_routing_lowering_selected=1
remote_owner_branch_routing_lowered_count=0
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
branch CFG lowering implementation
same/remote free full body
remote-heavy benchmark claim
TLS backing transfer
abandoned reclaim
allocator activation
```
