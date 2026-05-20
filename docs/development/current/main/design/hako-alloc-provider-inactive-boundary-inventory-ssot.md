---
Status: SSOT
Decision: accepted
Date: 2026-05-21
Row: MIMAP-352A
Scope: provider/host integration inactive boundary inventory after worker/TLS.
Related:
  - docs/development/current/main/phases/phase-293x/293x-968-MIMAP-352A-PROVIDER-INACTIVE-BOUNDARY-INVENTORY.md
  - lang/src/hako_alloc/memory/provider_inactive_boundary_inventory_box.hako
  - apps/hako-alloc-provider-inactive-boundary-inventory-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_provider_inactive_boundary_inventory_guard.sh
---

# Hako Alloc Provider Inactive Boundary Inventory

## Decision

MIMAP-352A records that provider activation, host allocator replacement,
hooks, `#[global_allocator]`, and backend owner-name matchers remain inactive
after the MIMAP-350A worker/TLS pilot.

This is an inventory row. It consumes a bounded
`HakoAllocWorkerTlsPilotReport` and publishes scalar facts that the provider
ladder is still closed. It does not select, activate, or call a provider.

## Owner

`HakoAllocProviderInactiveBoundaryInventory` owns this row.

It may:

- require an accepted worker/TLS report
- require an explicit positive provider boundary token
- copy worker/TLS, OSVM/page-source, atomic bitmap, segment-map mutation,
  pointer lookup, and arena handle tokens as inherited facts
- publish inactive provider/host/hook/backend matcher facts
- reject inherited reports that request provider activation, backend matchers,
  worker scheduling, or an invalid boundary token

It must not:

- activate an allocator provider
- replace the host allocator
- install hooks or `#[global_allocator]`
- add backend `.inc` matchers by app, box, owner, or row name
- expose source-level worker-local or concurrency syntax
- execute release/recycle behavior
- introduce cross-function `Result` direct ABI or runtime sum materialization

## Reasons

```text
0 = accepted
1 = missing worker/TLS fact
2 = rejected worker/TLS fact
3 = inherited provider activation request
4 = inherited backend matcher request
5 = inherited worker scheduling request
6 = invalid provider boundary token
```

## Validation

Daily validation is L2:

```bash
bash tools/checks/k2_wide_hako_alloc_provider_inactive_boundary_inventory_guard.sh --level L2
```

L3/L4 evidence is deferred to a provider boundary closeout or the first
explicit provider-facing ladder row.
