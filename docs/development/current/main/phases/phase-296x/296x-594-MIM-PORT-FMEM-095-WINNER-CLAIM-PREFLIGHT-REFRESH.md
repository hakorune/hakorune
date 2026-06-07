---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-095.
Related:
  - docs/development/current/main/phases/phase-296x/296x-593-MIM-PORT-FMEM-094-GLOBAL-ALLOCATOR-CLAIM-PRODUCER-REFRESH.md
  - docs/development/current/main/phases/phase-296x/296x-568-MIM-PORT-FMEM-070-WINNER-CLAIM-PREFLIGHT.md
  - docs/development/current/main/phases/phase-296x/296x-588-596-MIM-PORT-FMEM-REFRESH-LADDER-TASK-ORDER.md
---

# 296x-594 MIM-PORT-FMEM-095 Winner Claim Preflight Refresh

## Purpose

Select the refreshed winner claim preflight row after global allocator claim
producer evidence while keeping the actual winner claim closed.

## Required Boundaries

```text
winner claim remains closed
global allocator product claim remains closed
no real product allocator replacement
no hook installation side effect
no new MemOp kind
```

## Acceptance Sketch

```text
replacement_front_selected_route=winner_claim_preflight_refresh
replacement_front_selected_memop_family=winner_claim
replacement_front_selected_memop_kinds=WinnerClaim
replacement_front_next_producer_slice=winner_claim_producer_refresh

product_activation=1
hook_install=1
global_allocator_claim=1
global_allocator_product_claim=0
winner_claim_selected=1
winner_claim=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
```

## Landed Evidence

```text
replacement_front_selected_route=winner_claim_preflight_refresh
replacement_front_selected_memop_family=winner_claim
replacement_front_selected_memop_kinds=WinnerClaim
replacement_front_next_producer_slice=winner_claim_producer_refresh

fastmem_winner_claim_preflight_refresh=1
terminal_ladder_refresh_open=1
page_local_route_body_join_open=1
product_activation=1
hook_install=1
global_allocator_claim=1
global_allocator_product_claim=0
winner_claim_selected=1
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
296x-595 MIM-PORT-FMEM-096 winner claim producer refresh.
```

## Non-goals

```text
winner claim producer evidence
real global allocator replacement
real product activation or hook installation behavior
```
