---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-088.
Related:
  - docs/development/current/main/phases/phase-296x/296x-586-MIM-PORT-FMEM-087-ABANDONED-RECLAIM-PREFLIGHT-REFRESH.md
  - docs/development/current/main/phases/phase-296x/296x-561-MIM-PORT-FMEM-063-ABANDONED-RECLAIM-PRODUCER-PILOT.md
---

# 296x-587 MIM-PORT-FMEM-088 Abandoned Reclaim Producer Refresh

## Purpose

Reopen abandoned reclaim producer evidence on the refreshed owner/TLS ladder.
This row should enable abandoned reclaim evidence while keeping product
activation, hooks, global allocator claim, and winner claim closed.

## Required Boundaries

```text
product activation remains closed
hook installation remains closed
global allocator claim remains closed
winner claim remains closed
no new MemOp kind
no diagnostic Python-template C bridge retirement
```

## Acceptance Sketch

```text
replacement_front_selected_route=abandoned_reclaim_producer_refresh
replacement_front_selected_memop_family=abandoned_reclaim
replacement_front_selected_memop_kinds=AbandonedReclaim
replacement_front_next_producer_slice=product_activation_preflight_refresh

terminal_ladder_refresh_open=1
page_local_route_body_join_open=1
tls_backing_transfer_enabled=1
allocator_owner_slot_reuse_enabled=1
allocator_owner_generation_bump_count=1
abandoned_reclaim_selected=1
abandoned_reclaim_enabled=1
page_reclaimed_with_remote_candidates=0

product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
```

## Non-goals

```text
allocator activation
global allocator replacement
winner claim
```
