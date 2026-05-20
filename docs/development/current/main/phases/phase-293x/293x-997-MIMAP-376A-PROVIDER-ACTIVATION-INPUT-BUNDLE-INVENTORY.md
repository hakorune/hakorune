# 293x-997 MIMAP-376A Provider Activation Input Bundle Inventory

Status: landed
Date: 2026-05-21

## Purpose

Create the explicit provider activation input bundle required before any
provider activation first-pattern behavior row. This row only inventories the
activation input contract; it does not activate a provider or call provider
APIs.

## Scope

- Add `provider_activation_input_bundle_inventory_box.hako`.
- Consume the accepted MIMAP-370A unsupported-outcome ledger report.
- Require explicit row-owned inputs:
  - provider candidate token
  - provider kind
  - activation request token
  - activation mode
- Publish a scalar report that keeps provider activation unsupported/inactive
  and all execution flags at zero.

## Stop Lines

- No provider activation or provider calls.
- No hidden env, implicit discovery, or process-global activation config.
- No host allocator replacement, hooks, or `#[global_allocator]`.
- No backend `.inc` matcher by app, box, owner, or row name.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Evidence

```bash
bash tools/checks/k2_wide_hako_alloc_provider_activation_input_bundle_inventory_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed. MIMAP-377A is selected as the next row-selection card.
