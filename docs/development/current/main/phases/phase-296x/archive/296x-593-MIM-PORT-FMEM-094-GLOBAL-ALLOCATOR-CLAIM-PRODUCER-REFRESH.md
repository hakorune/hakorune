---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-094.
Related:
  - docs/development/current/main/phases/phase-296x/296x-592-MIM-PORT-FMEM-093-GLOBAL-ALLOCATOR-CLAIM-PREFLIGHT-REFRESH.md
  - docs/development/current/main/phases/phase-296x/296x-567-MIM-PORT-FMEM-069-GLOBAL-ALLOCATOR-CLAIM-PRODUCER-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-588-596-MIM-PORT-FMEM-REFRESH-LADDER-TASK-ORDER.md
---

# 296x-593 MIM-PORT-FMEM-094 Global Allocator Claim Producer Refresh

## Purpose

Reopen global allocator claim producer evidence on the refreshed ladder while
keeping winner claim and real product allocator behavior closed.

## Required Boundaries

```text
winner claim remains closed
global allocator product claim remains closed
no hook installation side effect
no real product allocator replacement
no new MemOp kind
```

## Acceptance Sketch

```text
replacement_front_selected_route=global_allocator_claim_producer_refresh
replacement_front_selected_memop_family=global_allocator_claim
replacement_front_selected_memop_kinds=GlobalAllocatorClaim
replacement_front_next_producer_slice=winner_claim_preflight_refresh

product_activation=1
hook_install=1
hook_installed=0
global_allocator_claim_selected=1
global_allocator_claim=1
global_allocator_product_claim=0
winner_claim=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
```

## Landed Evidence

```text
replacement_front_selected_route=global_allocator_claim_producer_refresh
replacement_front_selected_memop_family=global_allocator_claim
replacement_front_selected_memop_kinds=GlobalAllocatorClaim
replacement_front_next_producer_slice=winner_claim_preflight_refresh

fastmem_global_allocator_claim_producer_refresh=1
terminal_ladder_refresh_open=1
page_local_route_body_join_open=1
product_activation=1
hook_install=1
hook_installed=0
global_allocator_claim_selected=1
global_allocator_claim=1
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
296x-594 MIM-PORT-FMEM-095 winner claim preflight refresh.
```

## Non-goals

```text
winner claim
real global allocator replacement
real product activation or hook installation behavior
```
