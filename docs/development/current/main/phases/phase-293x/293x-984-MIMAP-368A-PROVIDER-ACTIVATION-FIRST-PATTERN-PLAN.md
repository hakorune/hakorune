# 293x-984 MIMAP-368A Provider Activation First-Pattern Plan

Status: landed
Date: 2026-05-21

## Decision

Plan the provider activation first-pattern boundary after provider-facing
ladder closeout, while keeping provider activation, host allocator
replacement, hooks, and `#[global_allocator]` closed.

The next behavior row should ledger unsupported provider activation outcomes
before any activation behavior is opened.

## Scope

- Record the provider activation first-pattern gate.
- Require unsupported activation outcome ledgering before activation.
- Keep host allocator replacement / hooks / `#[global_allocator]` as separate
  optional ladders.
- Reuse the MIMAP-366A provider-facing ladder closeout as prerequisite
  evidence.

## Stop Lines

- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No backend `.inc` matcher by app, box, owner, or row name.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_provider_activation_first_pattern_plan_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed. MIMAP-369A is selected as the next row-selection card.
