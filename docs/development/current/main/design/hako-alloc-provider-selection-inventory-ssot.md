---
Status: SSOT
Decision: accepted
Date: 2026-05-21
Row: MIMAP-364A
Scope: provider selection inventory with provider activation closed.
Related:
  - docs/development/current/main/phases/phase-293x/293x-980-MIMAP-364A-PROVIDER-SELECTION-INVENTORY.md
  - lang/src/hako_alloc/memory/provider_selection_inventory_box.hako
  - apps/hako-alloc-provider-selection-inventory-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_provider_selection_inventory_guard.sh
---

# Hako Alloc Provider Selection Inventory

## Decision

MIMAP-364A records a provider selection candidate after provider readiness
preflight while keeping provider activation closed.

This row consumes `HakoAllocProviderReadinessPreflightReport` and records a
provider candidate token and provider kind as scalar facts. It does not call,
activate, or install a provider.

## Reasons

```text
0 = accepted
1 = missing readiness preflight
2 = rejected readiness preflight
3 = invalid readiness token
4 = invalid provider candidate token
5 = invalid provider kind
6 = closed execution request
```

## Stop Lines

- No provider activation.
- No host allocator replacement.
- No hooks or `#[global_allocator]`.
- No backend `.inc` matcher by app, box, owner, or row name.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_provider_selection_inventory_guard.sh --level L2
```
