---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-051.
Related:
  - docs/development/current/main/phases/phase-296x/296x-548-MIM-PORT-FMEM-050-REMOTE-OWNER-BRANCH-ROUTE-BODY-PREFLIGHT.md
  - tools/hako_check/fastmem_source_syntax_smoke.sh
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-549 MIM-PORT-FMEM-051 FastMemory Branch CFG Preflight

## Purpose

Select and document the first branch-CFG row required after the remote-owner
route body preflight. Current fastmem source branch syntax is intentionally
closed by `branch_cfg_closed`; this row should keep it closed while pinning the
next acceptance surface for opening a narrow branch CFG producer later.

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
replacement_front_selected_route=fastmem_branch_cfg_preflight
remote_owner_branch_route_body_selected=1
remote_owner_branch_route_body_open=0
fastmem_branch_cfg_selected=1
fastmem_branch_cfg_open=0
fastmem_branch_cfg_closed_guard=1
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
opening branch CFG lowering
same/remote free full body execution
remote-heavy benchmark claim
TLS backing transfer
abandoned reclaim
allocator activation
```

## Result

```text
fastmem_branch_cfg_preflight=1
replacement_front_selected_route=fastmem_branch_cfg_preflight
replacement_front_next_producer_slice=fastmem_branch_cfg_lowering_preflight
remote_owner_branch_route_body_selected=1
remote_owner_branch_route_body_open=0
fastmem_branch_cfg_selected=1
fastmem_branch_cfg_open=0
fastmem_branch_cfg_closed_guard=1
page_local_free_route_cfg_lowering_enabled=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
global_allocator_claim=0
winner_claim=0
```

The source-level branch CFG guard remains closed through
`[freeze:contract][fastmem/branch_cfg_closed]`; this card only pins the report
and check surface for the next producer row.
