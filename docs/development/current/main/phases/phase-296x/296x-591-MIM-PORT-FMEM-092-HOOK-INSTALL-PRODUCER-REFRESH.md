---
Status: Active
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

## Non-goals

```text
global allocator replacement
winner claim
real hook installation side effect
```
