---
Status: Active
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

## Non-goals

```text
global allocator replacement
winner claim
real product activation or hook installation behavior
```
