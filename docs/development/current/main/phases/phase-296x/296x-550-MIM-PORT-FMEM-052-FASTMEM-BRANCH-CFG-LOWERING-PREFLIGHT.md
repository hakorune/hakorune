---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-052.
Related:
  - docs/development/current/main/phases/phase-296x/296x-549-MIM-PORT-FMEM-051-FASTMEM-BRANCH-CFG-PREFLIGHT.md
  - src/mir/builder/fastmem.rs
  - tools/hako_check/fastmem_source_syntax_smoke.sh
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-550 MIM-PORT-FMEM-052 FastMemory Branch CFG Lowering Preflight

## Purpose

Define the narrow acceptance surface for opening branch CFG lowering inside a
FastMemory region. MIM-PORT-FMEM-051 selected the branch CFG row but kept source
branch syntax closed; this card prepares the next producer slice without opening
the same/remote free body, TLS transfer, product activation, or allocator claim.

## Required Boundaries

```text
branch CFG lowering may only open through a dedicated preflight row
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
replacement_front_selected_route=fastmem_branch_cfg_lowering_preflight
fastmem_branch_cfg_selected=1
fastmem_branch_cfg_open=0
fastmem_branch_cfg_closed_guard=1
fastmem_branch_cfg_lowering_preflight=1
fastmem_branch_cfg_lowered_count=0
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
opening same/remote free full body execution
remote-heavy benchmark claim
TLS backing transfer
abandoned reclaim
allocator activation
```
