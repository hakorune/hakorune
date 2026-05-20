# 293x-988 MIMAP-372A Provider Activation Unsupported Outcome Closeout

Status: landed
Date: 2026-05-21

## Decision

Close out the provider activation unsupported outcome ledger before any
provider activation first-pattern row is considered. Provider activation,
provider calls, host allocator replacement, hooks, and backend matchers remain
closed.

## Scope

- Verify MIMAP-368A / 370A cards are landed.
- Verify MIMAP-370A proof app is manifest-backed.
- Run MIMAP-370A at L2.
- Keep activation and host-facing replacement/hook behavior closed.

## Stop Lines

- No provider activation or provider calls.
- No host allocator replacement, hooks, or `#[global_allocator]`.
- No backend `.inc` matcher by app, box, owner, or row name.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_provider_activation_unsupported_outcome_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed. MIMAP-373A is selected as the next row-selection card.
