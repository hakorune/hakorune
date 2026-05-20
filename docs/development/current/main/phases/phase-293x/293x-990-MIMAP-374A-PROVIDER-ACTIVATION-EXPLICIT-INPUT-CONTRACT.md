# 293x-990 MIMAP-374A Provider Activation Explicit-Input Contract

Status: landed
Date: 2026-05-21

## Decision

Select and document the explicit input contract required before any provider
activation first-pattern row. Activation remains closed; this row only fixes
the input boundary.

## Scope

- Require future activation input to be explicit and allocator-owned.
- Forbid hidden env, implicit discovery, process-global activation config, and
  backend owner-name matchers.
- Keep host allocator replacement, hooks, and `#[global_allocator]` as separate
  optional ladders.

## Stop Lines

- No provider activation or provider calls.
- No hidden env, implicit discovery, or process-global activation config.
- No host allocator replacement, hooks, or `#[global_allocator]`.
- No backend `.inc` matcher by app, box, owner, or row name.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_provider_activation_explicit_input_contract_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed. MIMAP-375A is selected as the next row-selection card.
