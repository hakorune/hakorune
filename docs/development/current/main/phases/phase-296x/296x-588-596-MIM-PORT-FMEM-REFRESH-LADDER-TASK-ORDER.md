---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-089..097.
Related:
  - docs/development/current/main/phases/phase-296x/296x-588-MIM-PORT-FMEM-089-PRODUCT-ACTIVATION-PREFLIGHT-REFRESH.md
  - docs/development/current/main/phases/phase-296x/296x-587-MIM-PORT-FMEM-088-ABANDONED-RECLAIM-PRODUCER-REFRESH.md
---

# 296x-588..596 MIM-PORT-FMEM Refresh Ladder Task Order

## Purpose

Keep the remaining FastMemory mimalloc replacement-front refresh ladder in one
small task-order card before reopening product activation, hook install, global
allocator claim, and winner claim rows.

This card is planning-only. It does not change the active blocker by itself.

## Decision

Continue the refresh ladder before source cleanup or real product integration.

The remaining rows are report/check ladder refresh rows. They should not add new
FastMemory MemOps, change `.hako` hako_alloc algorithm bodies, restore the
Python-template C bridge as semantic producer, or open real product allocator
behavior.

## Task Order

```text
296x-588 / MIM-PORT-FMEM-089:
  product_activation_preflight_refresh

296x-589 / MIM-PORT-FMEM-090:
  product_activation_producer_refresh

296x-590 / MIM-PORT-FMEM-091:
  hook_install_preflight_refresh

296x-591 / MIM-PORT-FMEM-092:
  hook_install_producer_refresh

296x-592 / MIM-PORT-FMEM-093:
  global_allocator_claim_preflight_refresh

296x-593 / MIM-PORT-FMEM-094:
  global_allocator_claim_producer_refresh

296x-594 / MIM-PORT-FMEM-095:
  winner_claim_preflight_refresh

296x-595 / MIM-PORT-FMEM-096:
  winner_claim_producer_refresh

296x-596 / MIM-PORT-FMEM-097:
  refreshed winner closeout audit
```

## Expected Profile Names

```text
product-activation-preflight-refresh
product-activation-producer-refresh
hook-install-preflight-refresh
hook-install-producer-refresh
global-allocator-claim-preflight-refresh
global-allocator-claim-producer-refresh
winner-claim-preflight-refresh
winner-claim-producer-refresh
```

## Expected Report Flags

```text
fastmem_product_activation_preflight_refresh
fastmem_product_activation_producer_refresh
fastmem_hook_install_preflight_refresh
fastmem_hook_install_producer_refresh
fastmem_global_allocator_claim_preflight_refresh
fastmem_global_allocator_claim_producer_refresh
fastmem_winner_claim_preflight_refresh
fastmem_winner_claim_producer_refresh
```

## Expected Next-Slice Chain

```text
product_activation_preflight_refresh
  -> product_activation_producer_refresh
  -> hook_install_preflight_refresh
  -> hook_install_producer_refresh
  -> global_allocator_claim_preflight_refresh
  -> global_allocator_claim_producer_refresh
  -> winner_claim_preflight_refresh
  -> winner_claim_producer_refresh
  -> complete
```

## Per-Row Boundary

Each row should add exactly one refreshed route profile:

```text
one profile name
one selected_route
one selected_memop_family
one selected_memop_kinds value
one next_producer_slice
one expected zero set
one expected positive set
one source-syntax smoke block
one fastmem-check terminal rule branch
```

Rows may update `CURRENT_STATE.toml` and the active card at closeout following
the current docs update policy.

## Always Closed During 588..596

```text
new MemOp kind=0
Python-template C bridge restoration=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
hook_installed=0
global_allocator_product_claim=0
```

## Deferred Until After 596

```text
real product activation implementation
real hook install behavior
global allocator replacement behavior
winner/perf claim validation
additional .hako hako_alloc body migration
Python-template C bridge retirement/delete decision
source/docs cleanup that is not needed for the refresh ladder
```

## Cleanup Timing

Do not mix cleanup with the refresh ladder rows.

After 296x-596, do a short closeout and cleanup planning pass before opening
real activation or bridge retirement work. That pass should decide whether to
clean report/check duplication, refresh reference docs, or move directly into
the next implementation ladder.
