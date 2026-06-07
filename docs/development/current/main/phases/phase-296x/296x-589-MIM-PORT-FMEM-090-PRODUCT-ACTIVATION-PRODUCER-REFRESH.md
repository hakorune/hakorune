---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-090.
Related:
  - docs/development/current/main/phases/phase-296x/296x-588-MIM-PORT-FMEM-089-PRODUCT-ACTIVATION-PREFLIGHT-REFRESH.md
  - docs/development/current/main/phases/phase-296x/296x-563-MIM-PORT-FMEM-065-PRODUCT-ACTIVATION-PRODUCER-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-588-596-MIM-PORT-FMEM-REFRESH-LADDER-TASK-ORDER.md
---

# 296x-589 MIM-PORT-FMEM-090 Product Activation Producer Refresh

## Purpose

Reopen product activation producer evidence on the refreshed ladder while
keeping hook installation, global allocator claim, and winner claim closed.

## Required Boundaries

```text
hook installation remains closed
global allocator claim remains closed
winner claim remains closed
no new MemOp kind
no diagnostic Python-template C bridge retirement
```

## Acceptance Sketch

```text
replacement_front_selected_route=product_activation_producer_refresh
replacement_front_selected_memop_family=product_activation
replacement_front_selected_memop_kinds=ProductActivation
replacement_front_next_producer_slice=hook_install_preflight_refresh

terminal_ladder_refresh_open=1
page_local_route_body_join_open=1
tls_backing_transfer_enabled=1
allocator_owner_slot_reuse_enabled=1
abandoned_reclaim_enabled=1
product_activation_selected=1
product_activation=1

hook_install=0
global_allocator_claim=0
winner_claim=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
```

## Non-goals

```text
hook installation
global allocator replacement
winner claim
```

## Landed Evidence

```text
replacement_front_selected_route=product_activation_producer_refresh
replacement_front_selected_memop_family=product_activation
replacement_front_selected_memop_kinds=ProductActivation
replacement_front_next_producer_slice=hook_install_preflight_refresh
fastmem_product_activation_producer_refresh=1
terminal_ladder_refresh_open=1
page_local_route_body_join_open=1
tls_backing_transfer_enabled=1
allocator_owner_slot_reuse_enabled=1
abandoned_reclaim_enabled=1
product_activation_selected=1
product_activation=1
hook_install=0
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
296x-590 MIM-PORT-FMEM-091 hook install preflight refresh.
```
