---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-086.
Related:
  - docs/development/current/main/phases/phase-296x/296x-584-MIM-PORT-FMEM-085-OWNER-SLOT-REUSE-PREFLIGHT-REFRESH.md
  - docs/development/current/main/phases/phase-296x/296x-559-MIM-PORT-FMEM-061-OWNER-SLOT-REUSE-PRODUCER-PILOT.md
---

# 296x-585 MIM-PORT-FMEM-086 Owner Slot Reuse Producer Refresh

## Purpose

Reopen owner slot reuse producer evidence on top of the refreshed terminal
ladder and TLS backing transfer producer rows. This row should enable owner
slot reuse evidence, including generation-bump proof, while keeping abandoned
reclaim and product activation closed.

## Required Boundaries

```text
abandoned reclaim behavior remains closed
product activation remains closed
hook installation remains closed
global allocator claim remains closed
winner claim remains closed
no new MemOp kind
no diagnostic Python-template C bridge retirement
```

## Acceptance Sketch

```text
replacement_front_selected_route=owner_slot_reuse_producer_refresh
replacement_front_selected_memop_family=owner_slot_reuse
replacement_front_selected_memop_kinds=OwnerSlotReuse
replacement_front_next_producer_slice=abandoned_reclaim_preflight_refresh

terminal_ladder_refresh_open=1
page_local_route_body_join_open=1
tls_backing_transfer_enabled=1
allocator_owner_slot_reuse_selected=1
allocator_owner_slot_reuse_enabled=1
allocator_owner_generation_bump_count=1
allocator_owner_reuse_without_generation_bump_count=0

abandoned_reclaim_enabled=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
```

## Non-goals

```text
abandoned reclaim
allocator activation
global allocator replacement
winner claim
```
