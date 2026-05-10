---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M81 allocator provider activation safety gate diagnostic shape.
Related:
  - docs/development/current/main/design/allocator-provider-activation-entry-contract-ssot.md
  - docs/development/current/main/design/allocator-provider-readiness-preflight-ssot.md
  - docs/development/current/main/design/allocator-provider-combined-dry-run-ssot.md
  - docs/development/current/main/design/allocator-provider-registry-snapshot-ssot.md
  - docs/development/current/main/design/allocator-provider-selection-decision-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-ssot.md
  - docs/development/current/main/design/allocator-provider-rollback-preflight-ssot.md
  - docs/development/current/main/design/allocator-hook-plan-v0-ssot.md
  - docs/development/current/main/design/allocator-hook-activation-preflight-shape-ssot.md
  - docs/development/current/main/design/allocator-hook-activation-proof-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-safety-gate-v0.toml
---

# Allocator Provider Activation Safety Gate (SSOT)

## Goal

Define the reserved allocator provider activation safety gate diagnostic shape
that sits after rollback preflight data and before any allocator hook
activation row.

M81 is a docs/fixture/guard row. It gathers the activation evidence bundle into
one gate contract, but the gate remains closed. M81 does not add runtime
registry code, provider selection, proof consumption, rollback preparation,
hook activation, process allocator replacement, implicit discovery, or
allocator replacement attributes.

## Decision

The reserved activation safety gate fixture is:

```text
docs/development/current/main/design/allocator-provider-activation-safety-gate-v0.toml
```

The safety gate shape is constructed only from explicit diagnostic inputs:

```text
activation entry contract
provider readiness preflight
combined hook/provider dry-run
registry snapshot
selection decision
provider proof bundle
rollback preflight
hook plan
hook activation preflight
hook activation proof
rollback target
```

No environment override, implicit file discovery, fallback provider, `.inc`
name matching, runner-owned behavior, hook installation, or production
activation is allowed.

## Safety Gate Contract

The reserved safety gate is closed before activation:

```text
active = false
activation_safety_gate = "inactive"
safety_status = "reserved_gate_closed"
activation_gate_open = false
would_open_activation_gate = false
would_activate_hook = false
would_activate = false
activation = "future_row_required"
```

The fixture uses `native_mimalloc` only to lock the reserved activation target
shape. M81 does not select, consume, prepare rollback, activate, register,
install, replace, or roll back that provider.

## Diagnostic Contract

The stable activation safety diagnostics are:

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

Activation remains blocked in M81:

```text
provider_selection = "inactive"
proof_bundle_consumption = "inactive"
rollback_preflight = "inactive"
hook_activation = "inactive"
activation_safety_gate = "inactive"
safety_status = "reserved_gate_closed"
activation_gate_open = false
would_build_registry = false
would_select_provider = false
would_consume_proof_bundle = false
would_prepare_rollback = false
would_open_activation_gate = false
would_activate_hook = false
would_activate = false
activation = "future_row_required"
```

## Required Facts

A future implementation row must fail fast when these facts are missing:

```text
activation_entry_contract_ready
provider_readiness_preflight_ready
combined_dry_run_ready
registry_snapshot_ready
selection_decision_ready
selected_provider_id_absent
proof_bundle_ready
rollback_preflight_ready
hook_plan_ready
hook_activation_preflight_ready
activation_proof_ready
rollback_target_explicit
activation_target_provider_id_explicit
safety_gate_policy_named
activation_gate_closed
fail_fast_activation_safety_diagnostic_named
no_hidden_environment_toggle
no_implicit_manifest_discovery
no_implicit_proof_discovery
no_app_or_facade_name_matching
no_inc_name_matching
no_runtime_registry_implementation
no_provider_selection_implementation
no_proof_consumption_implementation
no_rollback_preparation_implementation
no_hook_activation_implementation
no_global_allocator_attribute
no_global_alloc_trait
no_process_allocator_replacement
no_route_widening
```

## Stop Line

M81 keeps these inactive:

- runtime provider registry implementation;
- provider selection implementation;
- proof consumption implementation;
- provider rollback preparation implementation;
- activation safety gate implementation;
- hook activation implementation;
- hook activation CLI/env toggles;
- activation safety gate CLI/env toggles;
- implicit runtime file-system manifest discovery;
- implicit provider proof discovery;
- implicit hook plan discovery;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

## Next Row

A later row may consume this fixture only as an explicit diagnostic input to a
fail-fast runtime diagnostic implementation. That row must still keep the gate
closed until a separate activation row explicitly opens it.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_activation_safety_gate_guard.sh
bash tools/checks/k2_wide_allocator_provider_rollback_preflight_guard.sh
git diff --check
```
