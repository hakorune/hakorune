# 293x-1001 MIMAP-380A Provider Activation Modeled Open Pilot

Status: landed
Date: 2026-05-21

## Purpose

Open provider activation in model space after the explicit input bundle and
dry-run unsupported behavior rows. This row records activation-open evidence
without executing provider calls or process allocator replacement.

## Scope

- Add `provider_activation_modeled_open_pilot_box.hako`.
- Consume `HakoAllocProviderActivationDryRunUnsupportedBehaviorReport`.
- Accept only explicit, accepted dry-run outcomes.
- Record:
  - `provider_activation_modeled_open = 1`
  - `provider_activation_model_active = 1`
  - `provider_activation_inactive = 0`
  - `would_activate_provider = 1`
- Keep provider calls, host replacement, hooks, backend matcher additions, and
  worker/thread execution closed.

## Stop Lines

- No provider API calls.
- No hidden env, implicit discovery, or process-global activation config.
- No host allocator replacement, hooks, or `#[global_allocator]`.
- No backend `.inc` matcher by app, box, owner, or row name.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Evidence

```bash
bash tools/checks/k2_wide_hako_alloc_provider_activation_modeled_open_pilot_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed. MIMAP-381A is selected as the next row-selection card.
