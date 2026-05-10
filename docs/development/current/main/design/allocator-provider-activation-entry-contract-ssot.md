---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: M76 allocator provider activation entry contract.
Related:
  - docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md
  - docs/development/current/main/design/allocator-provider-registry-boundary-ssot.md
  - docs/development/current/main/design/allocator-provider-combined-dry-run-ssot.md
  - docs/development/current/main/design/allocator-hook-activation-proof-ssot.md
  - docs/development/current/main/design/allocator-hook-activation-preflight-shape-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-entry-contract-v0.toml
  - docs/development/current/main/design/allocator-provider-selection-decision-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-ssot.md
  - docs/development/current/main/design/allocator-provider-rollback-preflight-ssot.md
---

# Allocator Provider Activation Entry Contract (SSOT)

## Goal

Raise the post-M75 activation work into an explicit entry contract before any
provider registry implementation, provider selection implementation, hook
activation, or process allocator replacement row exists.

M76 is a docs/fixture/guard row. It introduces the activation entry ownership
and required facts, but it does not implement the registry, select a provider,
consume proof at runtime, activate a hook, or replace the process allocator.

## Decision

The activation entry contract is represented by this reserved fixture:

```text
docs/development/current/main/design/allocator-provider-activation-entry-contract-v0.toml
```

The contract names the future owners:

```text
registry_owner = "src/runtime/allocator_provider_registry.rs"
selection_owner = "src/runtime/allocator_provider_registry.rs"
activation_preflight_owner = "src/runtime/allocator_hook_dry_run.rs"
provider_manifest_owner = "src/runtime/allocator_provider_manifest.rs"
```

M76 does not create the registry owner file. The owner names are contract
anchors for later rows.

## Activation Entry Facts

A later activation row must consume explicit diagnostics and fail fast when any
required fact is missing. The M76 reserved fact list is:

```text
registry_selection_owner_named
explicit_provider_manifest_fact
provider_readiness_preflight_ready
combined_dry_run_ready
activation_proof_consumed
native_provider_proof_consumed
fail_fast_selection_diagnostic_named
rollback_behavior_named
no_hidden_environment_toggle
no_implicit_manifest_discovery
no_app_or_facade_name_matching
no_global_allocator_attribute
no_process_allocator_replacement_without_later_row
```

The reserved fail-fast diagnostic root is:

```text
[allocator-provider/activation-entry-contract-missing]
```

## Layer Contract

The future activation entry must be data-shaped:

```text
provider manifest parser
  -> provider readiness preflight
  -> combined hook/provider dry-run
  -> registry snapshot
  -> selection decision
  -> provider proof bundle consumption
  -> activation proof consumption
  -> rollback preflight
  -> later activation row
```

No layer may read environment variables, discover manifest files implicitly,
match `.inc` names, or silently select a provider.

## Stop Line

M76 keeps these inactive:

- runtime provider registry implementation;
- provider selection implementation;
- provider selection CLI;
- provider selection environment toggles;
- implicit runtime file-system manifest discovery;
- production mimalloc activation;
- runtime hook install/uninstall body;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

All M76 reports and fixtures keep:

```text
provider_selection = "inactive"
would_select_provider = false
would_activate = false
activation = "future_row_required"
```

## Next Ladder

M76 opens the post-M75 activation contract ladder:

| Row | Task | Output | Must Not Add |
| --- | --- | --- | --- |
| M76 | activation entry contract | this SSOT + reserved fixture + guard | runtime registry code |
| M77 | registry snapshot diagnostic shape | explicit registry snapshot data shape | provider selection |
| M78 | selection decision diagnostic shape | deterministic selection request/decision facts | activation |
| M79 | provider proof bundle consumption | explicit provider proof validation handoff | `#[global_allocator]` |
| M80 | rollback preflight contract | rollback facts before activation | process allocator replacement |

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_activation_entry_contract_guard.sh
bash tools/checks/k2_wide_allocator_provider_native_mimalloc_proof_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
