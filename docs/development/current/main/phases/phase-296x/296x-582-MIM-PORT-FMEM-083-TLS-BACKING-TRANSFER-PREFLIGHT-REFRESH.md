---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-083.
Related:
  - docs/development/current/main/phases/phase-296x/296x-581-MIM-PORT-FMEM-082-TERMINAL-LADDER-REFRESH-PREFLIGHT.md
  - docs/development/current/main/phases/phase-296x/296x-556-MIM-PORT-FMEM-058-TLS-BACKING-TRANSFER-PREFLIGHT.md
  - docs/development/current/main/phases/phase-296x/296x-557-MIM-PORT-FMEM-059-TLS-BACKING-TRANSFER-PRODUCER-PILOT.md
---

# 296x-582 MIM-PORT-FMEM-083 TLS Backing Transfer Preflight Refresh

## Purpose

Refresh the TLS backing transfer preflight so it consumes the terminal ladder
refresh boundary instead of the old page-local free route CFG-only boundary.
This keeps the terminal ladder aligned with the newer page-local alloc/free
route body join evidence before TLS behavior reopens.

## Required Boundaries

```text
TLS backing transfer lowering remains closed
owner slot reuse remains closed
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
replacement_front_selected_route=tls_backing_transfer_preflight_refresh
replacement_front_selected_memop_family=tls_backing_transfer
replacement_front_selected_memop_kinds=TlsBackingTransfer
replacement_front_next_producer_slice=tls_backing_transfer_producer_refresh

fastmem_terminal_ladder_refresh_preflight=1
terminal_ladder_refresh_selected=1
terminal_ladder_refresh_open=1
page_local_route_body_join_selected=1
page_local_route_body_join_open=1
page_local_alloc_route_cfg_lowering_enabled=1
page_local_free_route_cfg_lowering_enabled=1

tls_backing_transfer_selected=1
tls_backing_transfer_enabled=0
allocator_owner_slot_reuse_enabled=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
```

## Non-goals

```text
opening TLS backing transfer lowering
owner slot reuse
abandoned reclaim
allocator activation
global allocator replacement
winner claim
```
