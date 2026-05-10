---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: M80 allocator provider rollback preflight diagnostic shape.
Related:
  - docs/development/current/main/design/allocator-provider-activation-entry-contract-ssot.md
  - docs/development/current/main/design/allocator-provider-registry-snapshot-ssot.md
  - docs/development/current/main/design/allocator-provider-selection-decision-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-ssot.md
  - docs/development/current/main/design/allocator-hook-plan-v0-ssot.md
  - docs/development/current/main/design/allocator-hook-activation-preflight-shape-ssot.md
  - docs/development/current/main/design/allocator-hook-activation-proof-ssot.md
  - docs/development/current/main/design/allocator-provider-rollback-preflight-v0.toml
  - docs/development/current/main/design/allocator-provider-activation-safety-gate-ssot.md
---

# Allocator Provider Rollback Preflight (SSOT)

## Goal

Define the reserved allocator provider rollback preflight diagnostic shape that
sits after provider proof bundle consumption data and before any allocator hook
activation row.

M80 is a docs/fixture/guard row. It fixes the handoff contract for future
rollback preparation without adding runtime registry code, provider selection,
proof consumption, hook activation, process allocator replacement, implicit
discovery, or allocator replacement attributes.

## Decision

The reserved rollback preflight fixture is:

```text
docs/development/current/main/design/allocator-provider-rollback-preflight-v0.toml
```

The rollback preflight shape is constructed only from explicit diagnostic
inputs:

```text
preflight input
activation entry contract
registry snapshot
selection decision
provider proof bundle
hook plan
hook activation preflight
hook activation proof
rollback target
```

No environment override, implicit file discovery, fallback provider, `.inc`
name matching, runner-owned behavior, hook installation, or production
activation is allowed.

## Rollback Contract

The reserved rollback facts are defined before activation:

```text
active = false
rollback_preflight = "inactive"
rollback_status = "reserved_no_rollback"
would_prepare_rollback = false
would_activate = false
activation = "future_row_required"
```

The fixture uses `native_mimalloc` only to lock the reserved rollback target
shape. M80 does not select, consume, activate, register, install, replace, or
roll back that provider.

## Diagnostic Contract

The stable rollback preflight diagnostics are:

```text
[allocator-provider/rollback-preflight-missing]
[allocator-provider/rollback-input-missing]
[allocator-provider/rollback-snapshot-missing]
[allocator-provider/rollback-selection-missing]
[allocator-provider/rollback-proof-bundle-missing]
[allocator-provider/rollback-hook-plan-missing]
[allocator-provider/rollback-activation-preflight-missing]
[allocator-provider/rollback-activation-proof-missing]
[allocator-provider/rollback-target-missing]
[allocator-provider/rollback-activation-blocked]
```

Activation remains blocked in M80:

```text
provider_selection = "inactive"
proof_bundle_consumption = "inactive"
hook_activation = "inactive"
rollback_preflight = "inactive"
rollback_status = "reserved_no_rollback"
would_build_registry = false
would_select_provider = false
would_consume_proof_bundle = false
would_prepare_rollback = false
would_activate_hook = false
would_activate = false
activation = "future_row_required"
```

## Required Facts

A future implementation row must fail fast when these facts are missing:

```text
preflight_input_caller_provided
activation_entry_contract_ready
registry_snapshot_ready
selection_decision_ready
proof_bundle_ready
hook_plan_ready
hook_activation_preflight_ready
activation_proof_ready
rollback_target_explicit
rollback_target_provider_id_explicit
previous_allocator_state_snapshot_required
rollback_status_reserved
rollback_preflight_inactive
fail_fast_rollback_diagnostic_named
activation_blocked_until_future_row
no_hidden_environment_toggle
no_implicit_manifest_discovery
no_implicit_proof_discovery
no_app_or_facade_name_matching
no_inc_name_matching
no_runtime_registry_implementation
no_provider_selection_implementation
no_proof_consumption_implementation
no_hook_activation_implementation
no_global_allocator_attribute
no_global_alloc_trait
no_process_allocator_replacement
```

## Stop Line

M80 keeps these inactive:

- runtime provider registry implementation;
- provider selection implementation;
- proof consumption implementation;
- provider rollback preparation implementation;
- hook activation implementation;
- hook activation CLI/env toggles;
- provider rollback CLI/env toggles;
- implicit runtime file-system manifest discovery;
- implicit provider proof discovery;
- implicit hook plan discovery;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

## Next Row

M81 may consume this fixture only as an explicit diagnostic input to the
activation safety gate shape. M81 still must not prepare rollback, open an
activation gate, activate a hook, replace the process allocator, or add
implicit discovery.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_rollback_preflight_guard.sh
bash tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_guard.sh
git diff --check
```
