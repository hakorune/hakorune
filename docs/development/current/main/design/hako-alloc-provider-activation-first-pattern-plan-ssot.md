---
Status: SSOT
Decision: accepted
Date: 2026-05-21
Row: MIMAP-368A
Scope: provider activation first-pattern planning with activation still closed.
Related:
  - docs/development/current/main/phases/phase-293x/293x-984-MIMAP-368A-PROVIDER-ACTIVATION-FIRST-PATTERN-PLAN.md
  - tools/checks/k2_wide_hako_alloc_provider_activation_first_pattern_plan_guard.sh
---

# Hako Alloc Provider Activation First-Pattern Plan

## Decision

MIMAP-368A plans the provider activation first-pattern boundary after the
provider-facing ladder closeout. It does not activate a provider, replace the
host allocator, install hooks, or open `#[global_allocator]`.

The next provider-facing behavior row should prove unsupported activation
outcomes first. Actual provider activation requires a later explicit
first-pattern row with representative L3 evidence.

## Planned Order

1. provider activation unsupported outcome ledger
2. unsupported outcome observer / diagnostics, if the ledger row cannot carry
   the complete reason vocabulary itself
3. provider activation unsupported outcome closeout
4. provider activation first-pattern row, only after explicit selection
5. host allocator replacement / hooks / `#[global_allocator]` as separate
   optional ladders

## First-Pattern Gate

Provider activation may only open when all of these are true:

- provider boundary diagnostic vocabulary, readiness preflight, selection
  inventory, and provider-facing ladder closeout are landed
- unsupported activation outcomes are ledgered and fail-fast
- activation inputs are explicit; no hidden env or implicit discovery
- backend consumes route metadata and does not match app, box, owner, or row
  names
- L3 exact-MIR evidence is required for the first row that activates behavior

## Stop Lines

- No provider activation.
- No host allocator replacement.
- No hooks or `#[global_allocator]`.
- No backend `.inc` matcher by app, box, owner, or row name.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_provider_activation_first_pattern_plan_guard.sh
```
