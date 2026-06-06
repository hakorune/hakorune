---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-049.
Related:
  - docs/development/current/main/phases/phase-296x/296x-546-MIM-PORT-FMEM-048-REMOTE-OWNER-BRANCH-ROUTING-LOWERING-PREFLIGHT.md
  - src/mir/fastmem_access_plan.rs
  - src/llvm_py/instructions/memop.py
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-547 MIM-PORT-FMEM-049 Remote-Owner Branch Routing Lowering Producer Pilot

## Purpose

Open the first narrow producer pilot for remote-owner branch routing. The row
should lower only the routing split needed to distinguish same-owner and
remote-owner candidates, using verified owner equality evidence and existing
remote-free/list-mutation evidence.

## Required Boundaries

```text
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
replacement_front_selected_route=remote_owner_branch_routing_lowering_producer_pilot
remote_owner_branch_routing_selected=1
remote_owner_branch_routing_open=1
remote_owner_branch_routing_lowering_selected=1
remote_owner_branch_routing_lowered_count>=1
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
same/remote free full body
remote-heavy benchmark claim
TLS backing transfer
abandoned reclaim
allocator activation
```

## Landed Evidence

```text
source:
  lang/src/hako_alloc/memory/page_meta_remote_owner_branch_routing_lowering_box.hako

report/check:
  fastmem_remote_owner_branch_routing_lowering_producer_pilot=1
  replacement_front_selected_route=remote_owner_branch_routing_lowering_producer_pilot
  remote_owner_branch_routing_open=1
  remote_owner_branch_routing_lowered_count=1
  memop_current_alloc_owner_id_lowered_count=1
  memop_owner_eq_lowered_count=1
  atomic_remote_head_drain_local_list_mutation_lowered_count=1

boundaries:
  page_local_free_route_cfg_lowering_enabled=0
  type_abi_hot_lookup_count=0
  provider_abi_hot_dispatch_count=0
  product_activation=0
  hook_install=0
  global_allocator_claim=0
  winner_claim=0
```

## Verification

```text
python3 -m py_compile tools/hako_check/fastmem_check.py tools/hako_check/fastmem_mir_to_llvm_producer_report.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```
