---
Status: SSOT
Decision: accepted
Date: 2026-05-21
Row: MIMAP-370A
Scope: provider activation unsupported outcome ledger with activation closed.
Related:
  - lang/src/hako_alloc/memory/provider_activation_unsupported_outcome_ledger_box.hako
  - apps/hako-alloc-provider-activation-unsupported-outcome-ledger-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_provider_activation_unsupported_outcome_ledger_guard.sh
---

# Hako Alloc Provider Activation Unsupported Outcome Ledger

## Decision

MIMAP-370A adds a scalar/model ledger for provider activation requests that
are still unsupported. It consumes the provider selection inventory report and
records that activation remains inactive.

This is not provider activation. It is the fail-fast ledger row before a later
activation first-pattern can be selected.

## Owner

`HakoAllocProviderActivationUnsupportedOutcomeLedger` owns the unsupported
activation outcome row.

It may:

- read `HakoAllocProviderSelectionInventoryReport`
- record one unsupported activation outcome
- classify missing selection, rejected selection, invalid candidate, invalid
  provider kind, and closed execution requests
- publish scalar counters and report fields

It must not:

- activate providers or call provider APIs
- replace the host allocator
- install hooks or `#[global_allocator]`
- add backend `.inc` matchers
- run worker/TLS behavior or expose source-level concurrency
- open cross-function `Result` direct ABI or runtime sum materialization

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_provider_activation_unsupported_outcome_ledger_guard.sh --level L2
```
