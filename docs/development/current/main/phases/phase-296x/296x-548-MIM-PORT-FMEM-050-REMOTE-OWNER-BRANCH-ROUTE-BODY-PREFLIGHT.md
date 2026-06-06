---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-050.
Related:
  - docs/development/current/main/phases/phase-296x/296x-547-MIM-PORT-FMEM-049-REMOTE-OWNER-BRANCH-ROUTING-LOWERING-PRODUCER-PILOT.md
  - lang/src/hako_alloc/memory/page_meta_remote_owner_branch_routing_lowering_box.hako
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-548 MIM-PORT-FMEM-050 Remote-Owner Branch Route Body Preflight

## Purpose

Select the next narrow row after MIM-PORT-FMEM-049: describe the body-shape
preflight needed before actual source branch CFG lowering or full same/remote
free route execution opens.

MIM-PORT-FMEM-049 proves that one `.hako fastmem` region can lower owner
equality evidence and the already-open remote drain/local-list mutation
evidence together. MIM-PORT-FMEM-050 should decide the next body boundary
without changing behavior.

## Required Boundaries

```text
source fastmem branch CFG lowering remains closed
same/remote free full body route remains closed
remote-heavy benchmark claim remains closed
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
replacement_front_selected_route=remote_owner_branch_route_body_preflight
remote_owner_branch_routing_open=1
remote_owner_branch_routing_lowered_count>=1
remote_owner_branch_route_body_selected=1
remote_owner_branch_route_body_open=0
page_local_free_route_cfg_lowering_enabled=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Non-goals

```text
opening source if/branch CFG lowering
same/remote free full body execution
remote-heavy benchmark claim
TLS backing transfer
abandoned reclaim
allocator activation
```

## Landed Evidence

```text
report/check:
  fastmem_remote_owner_branch_route_body_preflight=1
  replacement_front_selected_route=remote_owner_branch_route_body_preflight
  replacement_front_next_producer_slice=fastmem_branch_cfg_preflight
  remote_owner_branch_routing_open=1
  remote_owner_branch_routing_lowered_count=1
  remote_owner_branch_route_body_selected=1
  remote_owner_branch_route_body_open=0

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
