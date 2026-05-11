---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M100 allocator provider proof bundle consumption implementation entry contract.
Related:
  - docs/development/current/main/design/allocator-provider-activation-implementation-entry-contract-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-cli-surface-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-entry-contract-v0.toml
  - tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_entry_contract_guard.sh
---

# Allocator Provider Proof Bundle Consumption Entry Contract (SSOT)

## Goal

Name one future proof-bundle consumption owner and one future entry before any
proof consumption behavior is implemented.

M100 is docs/fixture/guard only. It does not consume proofs, select a provider,
build an active provider registry, prepare rollback, open an activation gate,
install allocator hooks, activate a native allocator, or replace the process
allocator.

## Decision

The future proof-bundle consumption behavior owner is the activation
orchestration owner reserved by M92:

```text
src/runtime/allocator_provider_activation.rs
```

The future entry name is:

```text
allocator_provider_proof_bundle_consumption_attempt
```

The diagnostic report owners remain diagnostic-only:

- `src/runtime/allocator_provider_registry.rs` owns inactive registry snapshot,
  selection decision, proof-bundle consumption, and activation safety reports.
- `src/runtime/allocator_provider_activation_decision.rs` owns inactive
  activation decision reports.
- `src/cli/allocator_provider_proof_bundle_consumption.rs` owns only the M99
  explicit-path CLI diagnostic surface.
- `src/runtime/allocator_provider_activation.rs` will own future behavior only
  after a later guarded row creates it.

## Reserved Fixture

The contract is represented by:

```text
docs/development/current/main/design/allocator-provider-proof-bundle-consumption-entry-contract-v0.toml
```

Required fixture facts:

```text
activation_owner_named
proof_bundle_consumption_owner_named
proof_bundle_consumption_entry_named
activation_decision_report_explicit
registry_snapshot_report_explicit
selection_decision_report_explicit
proof_bundle_consumption_report_explicit
selected_provider_required_before_consumption
provider_proof_bundle_explicit
fail_fast_proof_bundle_entry_diagnostic_named
no_hidden_environment_toggle
no_implicit_manifest_discovery
no_implicit_report_discovery
no_implicit_proof_discovery
no_provider_selection_implementation
no_proof_consumption_implementation
no_rollback_preparation_implementation
no_activation_gate_opening
no_hook_activation_implementation
no_process_allocator_replacement
```

The reserved fail-fast diagnostic root is:

```text
[allocator-provider/proof-bundle-consumption-entry-missing]
```

## Stop Line

M100 changes only the named owner/entry boundary. These remain inactive:

- active runtime provider registry construction;
- provider selection;
- proof consumption;
- rollback preparation or execution;
- activation gate opening;
- hook activation or native activation;
- provider/proof environment toggles, including `NYASH_ALLOCATOR_PROVIDER`,
  `HAKO_ALLOCATOR_PROVIDER`, and broad `ALLOCATOR_PROVIDER_*` names;
- implicit runtime file-system manifest/report/proof discovery;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

All M100 reports and fixtures keep:

```text
proof_bundle_consumption_implementation_status = "owner_entry_reserved"
selected_provider_requirement = "future_selected_provider_required"
proof_bundle_consumed = false
would_build_registry = false
would_select_provider = false
would_consume_proof_bundle = false
would_prepare_rollback = false
would_open_activation_gate = false
would_install_hook = false
would_replace_process_allocator = false
would_activate = false
activation = "future_row_required"
```

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_entry_contract_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
