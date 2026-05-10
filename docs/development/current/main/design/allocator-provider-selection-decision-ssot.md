---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: M78 allocator provider selection decision diagnostic shape.
Related:
  - docs/development/current/main/design/allocator-provider-activation-entry-contract-ssot.md
  - docs/development/current/main/design/allocator-provider-registry-boundary-ssot.md
  - docs/development/current/main/design/allocator-provider-registry-snapshot-ssot.md
  - docs/development/current/main/design/allocator-provider-selection-decision-v0.toml
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-ssot.md
---

# Allocator Provider Selection Decision (SSOT)

## Goal

Define the reserved selection request/decision diagnostic shape that sits after
the registry snapshot and before provider proof bundle consumption.

M78 is a docs/fixture/guard row. It fixes deterministic selection facts without
adding runtime provider selection code, selecting a provider, activating a hook,
or replacing the process allocator.

## Decision

The reserved selection decision fixture is:

```text
docs/development/current/main/design/allocator-provider-selection-decision-v0.toml
```

The selection decision shape is constructed only from explicit diagnostic
inputs:

```text
registry snapshot
caller-provided selection request
```

No environment override, file discovery, fallback provider, `.inc` name
matching, runner-owned behavior, or production activation is allowed.

## Request Contract

The request shape is explicit and caller-provided:

```text
requested_provider_id
requested_operations
selection_policy = "explicit_provider_id_required_reserved"
selection_status = "reserved_no_selection"
selected_provider_id = "none_reserved"
deterministic_provider_order = "registry_snapshot_order"
```

The fixture uses `native_mimalloc` as a reserved requested provider id only to
fix the data shape. M78 does not select it.

## Decision Contract

The reserved decision remains inactive:

```text
decision_status = "reserved"
selection_status = "reserved_no_selection"
provider_selection = "inactive"
selected_provider_id = "none_reserved"
would_build_registry = false
would_select_provider = false
would_activate = false
activation = "future_row_required"
```

The reserved diagnostics are:

```text
[allocator-provider/selection-decision-missing]
[allocator-provider/selection-registry-missing]
[allocator-provider/selection-request-missing]
[allocator-provider/selection-unsupported-provider]
[allocator-provider/selection-capability-missing]
[allocator-provider/selection-ambiguous]
```

## Required Facts

A future implementation row must fail fast when these facts are missing:

```text
registry_snapshot_ready
selection_request_caller_provided
requested_provider_id_explicit
requested_operations_nonempty
candidate_provider_ids_reserved_set
deterministic_provider_order_named
selection_policy_named
fail_fast_selection_diagnostic_named
no_selected_provider_id
no_hidden_environment_toggle
no_implicit_manifest_discovery
no_app_or_facade_name_matching
no_runtime_registry_implementation
no_runtime_hook_activation
no_global_allocator_attribute
no_activation_without_later_row
```

## Stop Line

M78 keeps these inactive:

- runtime provider registry implementation;
- provider selection implementation;
- provider selection CLI;
- provider selection environment toggles;
- implicit runtime file-system manifest discovery;
- runtime hook install/uninstall body;
- provider proof bundle consumption;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

## Next Row

M79 may consume this reserved decision only as a diagnostic input to a provider
proof bundle shape. M79 must still keep no selected provider, no runtime proof
consumption, no activation, and no process allocator replacement.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_selection_decision_guard.sh
bash tools/checks/k2_wide_allocator_provider_registry_snapshot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
