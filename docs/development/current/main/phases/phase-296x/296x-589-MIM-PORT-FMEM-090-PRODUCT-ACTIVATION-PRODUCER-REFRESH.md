---
Status: Active
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
