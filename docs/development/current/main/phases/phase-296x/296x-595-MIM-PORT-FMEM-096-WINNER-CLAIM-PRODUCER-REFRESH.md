---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-096.
Related:
  - docs/development/current/main/phases/phase-296x/296x-594-MIM-PORT-FMEM-095-WINNER-CLAIM-PREFLIGHT-REFRESH.md
  - docs/development/current/main/phases/phase-296x/296x-569-MIM-PORT-FMEM-071-WINNER-CLAIM-PRODUCER-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-588-596-MIM-PORT-FMEM-REFRESH-LADDER-TASK-ORDER.md
---

# 296x-595 MIM-PORT-FMEM-096 Winner Claim Producer Refresh

## Purpose

Reopen winner claim producer evidence on the refreshed ladder and close the
refreshed producer chain with `replacement_front_next_producer_slice=complete`.

## Required Boundaries

```text
global allocator product claim remains closed
no real product allocator replacement
no hook installation side effect
no new MemOp kind
```

## Acceptance Sketch

```text
replacement_front_selected_route=winner_claim_producer_refresh
replacement_front_selected_memop_family=winner_claim
replacement_front_selected_memop_kinds=WinnerClaim
replacement_front_next_producer_slice=complete

product_activation=1
hook_install=1
global_allocator_claim=1
global_allocator_product_claim=0
winner_claim_selected=1
winner_claim=1
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
```

## Non-goals

```text
real global allocator replacement
real product activation or hook installation behavior
performance winner validation
```
