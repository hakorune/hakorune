---
Status: Active
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

## Non-goals

```text
winner claim
real global allocator replacement
real product activation or hook installation behavior
```
