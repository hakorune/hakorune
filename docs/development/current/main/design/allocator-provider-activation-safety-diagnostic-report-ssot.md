---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M83 allocator provider activation safety diagnostic report.
Related:
  - docs/development/current/main/design/allocator-provider-activation-safety-diagnostic-owner-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-safety-gate-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-safety-gate-v0.toml
  - src/runtime/allocator_provider_registry.rs
  - tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_report_guard.sh
---

# Allocator Provider Activation Safety Diagnostic Report (SSOT)

## Goal

Add the diagnostic-only runtime report for the allocator provider activation
safety gate in the owner named by M82.

M83 consumes caller-provided TOML text only. It does not discover files, expose
CLI/env toggles, select a provider, consume proof bundles, prepare rollback,
open the activation gate, activate hooks, or replace the process allocator.

## Runtime Owner

```text
src/runtime/allocator_provider_registry.rs
```

Public diagnostic surface:

```text
AllocatorProviderActivationSafetyFacts
AllocatorProviderActivationSafetyReport
AllocatorProviderActivationSafetyStatus
validate_allocator_provider_activation_safety_gate(...)
validate_allocator_provider_activation_safety_gate_from_text(...)
```

The report validates the reserved M81 fixture shape and returns either:

```text
status = MissingFacts
diagnostic = [allocator-provider/activation-safety-gate-missing]
```

or:

```text
status = ReadyGateClosed
diagnostic = [allocator-provider/activation-safety-blocked]
```

## Closed Gate Contract

Even when all diagnostic evidence is present, the report is blocked and keeps:

```text
safety_status = "reserved_gate_closed"
activation_gate_open = false
would_open_activation_gate = false
would_activate_hook = false
would_activate = false
```

M83 has no activation success state.

## Required Diagnostics

The implementation owns these stable diagnostic constants:

```text
[allocator-provider/activation-safety-gate-missing]
[allocator-provider/activation-safety-entry-missing]
[allocator-provider/activation-safety-readiness-missing]
[allocator-provider/activation-safety-combined-dry-run-missing]
[allocator-provider/activation-safety-registry-missing]
[allocator-provider/activation-safety-selection-missing]
[allocator-provider/activation-safety-proof-bundle-missing]
[allocator-provider/activation-safety-rollback-missing]
[allocator-provider/activation-safety-hook-plan-missing]
[allocator-provider/activation-safety-preflight-missing]
[allocator-provider/activation-safety-proof-missing]
[allocator-provider/activation-safety-target-missing]
[allocator-provider/activation-safety-blocked]
```

## Stop Line

M83 keeps these inactive:

- activation gate opening;
- runtime provider selection implementation;
- runtime proof consumption implementation;
- provider rollback preparation/execution;
- hook activation implementation;
- activation safety CLI/env toggles;
- implicit runtime file-system manifest discovery;
- implicit provider proof discovery;
- implicit hook plan discovery;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

## Next Row

M84 may expose this report through an explicit CLI diagnostic surface. That row
must still require caller-provided paths/text only and must not add environment
discovery, implicit discovery, gate opening, hook activation, or process
allocator replacement.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_report_guard.sh
cargo test -q activation_safety -- --nocapture
git diff --check
```
