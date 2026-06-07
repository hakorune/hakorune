---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-092.
Related:
  - docs/development/current/main/phases/phase-296x/296x-590-MIM-PORT-FMEM-091-HOOK-INSTALL-PREFLIGHT-REFRESH.md
  - docs/development/current/main/phases/phase-296x/296x-565-MIM-PORT-FMEM-067-HOOK-INSTALL-PRODUCER-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-588-596-MIM-PORT-FMEM-REFRESH-LADDER-TASK-ORDER.md
---

# 296x-591 MIM-PORT-FMEM-092 Hook Install Producer Refresh

## Purpose

Reopen hook install producer evidence on the refreshed ladder while keeping
global allocator claim and winner claim closed.

## Required Boundaries

```text
global allocator claim remains closed
winner claim remains closed
hook_installed remains 0
no new MemOp kind
no diagnostic Python-template C bridge retirement
```

## Acceptance Sketch

```text
replacement_front_selected_route=hook_install_producer_refresh
replacement_front_selected_memop_family=hook_install
replacement_front_selected_memop_kinds=HookInstall
replacement_front_next_producer_slice=global_allocator_claim_preflight_refresh

product_activation=1
hook_install_selected=1
hook_install=1
hook_installed=0
global_allocator_claim=0
winner_claim=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
```

## Landed Evidence

```text
replacement_front_selected_route=hook_install_producer_refresh
replacement_front_selected_memop_family=hook_install
replacement_front_selected_memop_kinds=HookInstall
replacement_front_next_producer_slice=global_allocator_claim_preflight_refresh

fastmem_hook_install_producer_refresh=1
terminal_ladder_refresh_open=1
page_local_route_body_join_open=1
product_activation=1
hook_install_selected=1
hook_install=1
hook_installed=0
global_allocator_claim=0
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
296x-592 MIM-PORT-FMEM-093 global allocator claim preflight refresh.
```

## Non-goals

```text
global allocator replacement
winner claim
real hook installation side effect
```
