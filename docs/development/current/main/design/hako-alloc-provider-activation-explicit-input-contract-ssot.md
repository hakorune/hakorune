---
Status: SSOT
Decision: accepted
Date: 2026-05-21
Row: MIMAP-374A
Scope: provider activation explicit-input contract with activation closed.
Related:
  - docs/development/current/main/phases/phase-293x/293x-990-MIMAP-374A-PROVIDER-ACTIVATION-EXPLICIT-INPUT-CONTRACT.md
  - tools/checks/k2_wide_hako_alloc_provider_activation_explicit_input_contract_guard.sh
---

# Hako Alloc Provider Activation Explicit-Input Contract

## Decision

MIMAP-374A fixes the provider activation input boundary before any provider
activation first-pattern row. Activation inputs must be explicit and row-owned.
Hidden environment discovery, implicit process configuration, provider calls,
host allocator replacement, hooks, and `#[global_allocator]` remain closed.

This row is a planning/contract row. It does not add runtime provider
activation behavior.

## Required Input Shape

A future activation first-pattern row must provide an explicit input bundle:

- selected provider candidate token
- provider kind
- unsupported-outcome closeout evidence
- activation request reason
- closed-state proof for host replacement, hooks, backend matcher, and worker
  behavior

The input bundle must be produced by allocator-owned rows. It must not be
constructed by reading hidden env vars, process-global config, filesystem
discovery, or backend owner-name matchers.

## Stop Lines

- No provider activation or provider calls.
- No hidden env, implicit discovery, or process-global activation config.
- No host allocator replacement.
- No hooks or `#[global_allocator]`.
- No backend `.inc` matcher by app, box, owner, or row name.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_provider_activation_explicit_input_contract_guard.sh
```
