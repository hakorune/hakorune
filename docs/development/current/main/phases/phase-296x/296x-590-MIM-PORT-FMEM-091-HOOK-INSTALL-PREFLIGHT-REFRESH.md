---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-091.
Related:
  - docs/development/current/main/phases/phase-296x/296x-589-MIM-PORT-FMEM-090-PRODUCT-ACTIVATION-PRODUCER-REFRESH.md
  - docs/development/current/main/phases/phase-296x/296x-564-MIM-PORT-FMEM-066-HOOK-INSTALL-PREFLIGHT.md
  - docs/development/current/main/phases/phase-296x/296x-588-596-MIM-PORT-FMEM-REFRESH-LADDER-TASK-ORDER.md
---

# 296x-590 MIM-PORT-FMEM-091 Hook Install Preflight Refresh

## Purpose

Refresh the hook install preflight so it consumes refreshed product activation
producer evidence. This row should select hook install while keeping hook
installation behavior, global allocator claim, and winner claim closed.

## Required Boundaries

```text
hook installation behavior remains closed
global allocator claim remains closed
winner claim remains closed
no new MemOp kind
no diagnostic Python-template C bridge retirement
```

## Acceptance Sketch

```text
replacement_front_selected_route=hook_install_preflight_refresh
replacement_front_selected_memop_family=hook_install
replacement_front_selected_memop_kinds=HookInstall
replacement_front_next_producer_slice=hook_install_producer_refresh

terminal_ladder_refresh_open=1
page_local_route_body_join_open=1
product_activation=1
hook_install_selected=1
hook_install=0

global_allocator_claim=0
winner_claim=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
```

## Non-goals

```text
hook install producer behavior
global allocator replacement
winner claim
```
