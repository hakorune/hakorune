---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-093.
Related:
  - docs/development/current/main/phases/phase-296x/296x-591-MIM-PORT-FMEM-092-HOOK-INSTALL-PRODUCER-REFRESH.md
  - docs/development/current/main/phases/phase-296x/296x-566-MIM-PORT-FMEM-068-GLOBAL-ALLOCATOR-CLAIM-PREFLIGHT.md
  - docs/development/current/main/phases/phase-296x/296x-588-596-MIM-PORT-FMEM-REFRESH-LADDER-TASK-ORDER.md
---

# 296x-592 MIM-PORT-FMEM-093 Global Allocator Claim Preflight Refresh

## Purpose

Select the refreshed global allocator claim preflight row after hook install
producer evidence while keeping the actual global allocator claim and winner
claim closed.

## Required Boundaries

```text
winner claim remains closed
global allocator claim remains closed
global allocator product claim remains closed
no hook installation side effect
no new MemOp kind
```

## Acceptance Sketch

```text
replacement_front_selected_route=global_allocator_claim_preflight_refresh
replacement_front_selected_memop_family=global_allocator_claim
replacement_front_selected_memop_kinds=GlobalAllocatorClaim
replacement_front_next_producer_slice=global_allocator_claim_producer_refresh

product_activation=1
hook_install_selected=1
hook_install=1
hook_installed=0
global_allocator_claim_selected=1
global_allocator_claim=0
global_allocator_product_claim=0
winner_claim=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
```

## Landed Evidence

```text
replacement_front_selected_route=global_allocator_claim_preflight_refresh
replacement_front_selected_memop_family=global_allocator_claim
replacement_front_selected_memop_kinds=GlobalAllocatorClaim
replacement_front_next_producer_slice=global_allocator_claim_producer_refresh

fastmem_global_allocator_claim_preflight_refresh=1
terminal_ladder_refresh_open=1
page_local_route_body_join_open=1
product_activation=1
hook_install_selected=1
hook_install=1
hook_installed=0
global_allocator_claim_selected=1
global_allocator_claim=0
global_allocator_product_claim=0
winner_claim=0
```

## Verification

```bash
python3 -m py_compile tools/hako_check/fastmem_route_profiles.py tools/hako_check/fastmem_check_profile_functions.py tools/hako_check/fastmem_check_terminal_rules.py tools/hako_check/fastmem_mir_to_llvm_producer_report_rows.py tools/hako_check/fastmem_mir_to_llvm_producer_report_route_rows.py tools/hako_check/fastmem_mir_to_llvm_producer_report_body.py tools/hako_check/fastmem_mir_to_llvm_producer_report_tail_rows.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Next

```text
296x-593 MIM-PORT-FMEM-094 global allocator claim producer refresh.
```

## Non-goals

```text
global allocator replacement
winner claim
real product activation or hook installation behavior
```
