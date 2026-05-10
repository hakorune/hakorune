---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: M79 allocator provider proof bundle consumption diagnostic shape.
Related:
  - docs/development/current/main/design/allocator-provider-activation-entry-contract-ssot.md
  - docs/development/current/main/design/allocator-provider-registry-snapshot-ssot.md
  - docs/development/current/main/design/allocator-provider-selection-decision-ssot.md
  - docs/development/current/main/design/allocator-provider-native-system-proof-ssot.md
  - docs/development/current/main/design/allocator-provider-native-mimalloc-proof-ssot.md
  - docs/development/current/main/design/allocator-provider-hako-model-proof-ssot.md
  - docs/development/current/main/design/allocator-provider-debug-guarded-proof-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-v0.toml
---

# Allocator Provider Proof Bundle Consumption (SSOT)

## Goal

Define the reserved provider proof bundle consumption diagnostic shape that
sits after selection decision data and before any allocator provider
activation row.

M79 is a docs/fixture/guard row. It fixes the handoff contract for explicit
proof bundle data without adding runtime registry code, provider selection,
provider activation, process allocator replacement, or implicit discovery.

## Decision

The reserved proof bundle consumption fixture is:

```text
docs/development/current/main/design/allocator-provider-proof-bundle-consumption-v0.toml
```

The proof bundle consumption shape is constructed only from explicit
diagnostic inputs:

```text
registry snapshot
selection decision
caller-provided provider proof bundle
```

No environment override, implicit file discovery, fallback provider,
`.inc` name matching, runner-owned behavior, or production activation is
allowed.

## Consumption Contract

The proof bundle request is explicit and caller-provided:

```text
proof_bundle_source = "caller_provided_diagnostic_bundle"
requested_provider_id = "native_mimalloc"
selected_provider_id = "none_reserved"
consumption_status = "reserved_no_consumption"
proof_bundle_consumption = "inactive"
```

The fixture uses `native_mimalloc` only to lock the reserved handoff shape.
M79 does not select, consume, activate, register, or install that provider.

## Diagnostic Contract

The reserved diagnostic remains inactive:

```text
decision_status = "reserved"
selection_status = "reserved_no_selection"
provider_selection = "inactive"
proof_bundle_consumed = false
would_build_registry = false
would_select_provider = false
would_consume_proof_bundle = false
would_activate = false
activation = "future_row_required"
```

The reserved diagnostics are:

```text
[allocator-provider/proof-bundle-consumption-missing]
[allocator-provider/proof-bundle-registry-missing]
[allocator-provider/proof-bundle-selection-missing]
[allocator-provider/proof-bundle-provider-proof-missing]
[allocator-provider/proof-bundle-provider-mismatch]
[allocator-provider/proof-bundle-capability-missing]
[allocator-provider/proof-bundle-activation-blocked]
```

## Required Facts

A future implementation row must fail fast when these facts are missing:

```text
registry_snapshot_ready
selection_decision_ready
proof_bundle_caller_provided
requested_provider_id_explicit
selected_provider_id_absent
provider_proof_entries_nonempty
provider_proof_ids_reserved_set
provider_proof_operations_cover_request
proof_bundle_policy_named
fail_fast_proof_bundle_diagnostic_named
no_hidden_environment_toggle
no_implicit_manifest_discovery
no_app_or_facade_name_matching
no_inc_name_matching
no_runtime_registry_implementation
no_provider_selection_implementation
no_runtime_hook_activation
no_global_allocator_attribute
no_global_alloc_trait
no_activation_without_later_row
```

## Stop Line

M79 keeps these inactive:

- runtime provider registry implementation;
- provider selection implementation;
- provider proof bundle consumption implementation;
- provider selection CLI;
- provider proof bundle CLI;
- provider selection or proof bundle environment toggles;
- implicit runtime file-system manifest discovery;
- implicit provider proof discovery;
- runtime hook install/uninstall body;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

## Next Row

A later row may consume this fixture only as an explicit diagnostic input to a
fail-fast implementation. That later row must still choose a single owner for
registry/selection/proof consumption before any activation code is introduced.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_guard.sh
bash tools/checks/k2_wide_allocator_provider_selection_decision_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
