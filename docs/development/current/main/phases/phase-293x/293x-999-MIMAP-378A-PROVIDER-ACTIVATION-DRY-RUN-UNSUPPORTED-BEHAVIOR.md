# 293x-999 MIMAP-378A Provider Activation Dry-Run Unsupported Behavior

Status: landed
Date: 2026-05-21

## Purpose

Consume the explicit provider activation input bundle from MIMAP-376A and
produce a dry-run unsupported activation outcome. This is a scalar/model
behavior row only; provider activation remains closed.

## Scope

- Add `provider_activation_dry_run_unsupported_behavior_box.hako`.
- Consume `HakoAllocProviderActivationInputBundleInventoryReport`.
- Accept only explicit, accepted input bundles.
- Publish a scalar report that records `dry_run_attempted = 1` and
  `unsupported_outcome_present = 1` for the accepted row.
- Keep provider activation, provider calls, host replacement, hooks, backend
  matcher additions, and thread execution inactive.

## Stop Lines

- No provider activation or provider calls.
- No hidden env, implicit discovery, or process-global activation config.
- No host allocator replacement, hooks, or `#[global_allocator]`.
- No backend `.inc` matcher by app, box, owner, or row name.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Evidence

```bash
bash tools/checks/k2_wide_hako_alloc_provider_activation_dry_run_unsupported_behavior_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed. MIMAP-379A is selected as the next row-selection card.
