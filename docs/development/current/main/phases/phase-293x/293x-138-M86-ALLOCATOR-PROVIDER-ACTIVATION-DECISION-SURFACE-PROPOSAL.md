---
Status: Completed
Date: 2026-05-11
Scope: M86 allocator provider activation decision surface proposal.
Related:
  - docs/development/current/main/design/allocator-provider-activation-decision-surface-proposal-ssot.md
  - tools/checks/k2_wide_allocator_provider_activation_decision_surface_proposal_guard.sh
---

# 293x-138 M86 Allocator Provider Activation Decision Surface Proposal

## Summary

M86 defines the future allocator provider activation decision surface as a
docs-first proposal.

The proposed surface is explicit input only:
`hakorune --allocator-provider-activation-decision <ACTIVATION_DECISION_TOML>`.
M86 does not implement that CLI route. It only fixes the vocabulary and the
required gate-closed output shape for a later fixture/parser row.

## Boundary

M86 is proposal-only. It does not add runtime parsing, CLI routing, environment
toggles, provider selection, proof consumption, rollback preparation, activation
gate opening, hook activation, `#[global_allocator]`, process allocator
replacement, route widening, or `.inc` name matching.

## Verification

```bash
bash -n tools/checks/k2_wide_allocator_provider_activation_decision_surface_proposal_guard.sh
bash tools/checks/k2_wide_allocator_provider_activation_decision_surface_proposal_guard.sh
git diff --check
```
