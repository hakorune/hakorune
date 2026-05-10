---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: M77 allocator provider registry snapshot diagnostic shape.
Related:
  - docs/development/current/main/design/allocator-provider-activation-entry-contract-ssot.md
  - docs/development/current/main/design/allocator-provider-manifest-v0-ssot.md
  - docs/development/current/main/design/allocator-provider-readiness-preflight-ssot.md
  - docs/development/current/main/design/allocator-provider-registry-snapshot-v0.toml
  - docs/development/current/main/design/allocator-provider-selection-decision-ssot.md
---

# Allocator Provider Registry Snapshot (SSOT)

## Goal

Define the registry snapshot diagnostic shape that a later runtime registry
owner may implement, without adding registry code, selecting a provider, or
activating allocator replacement.

M77 is a docs/fixture/guard row. It fixes the data contract that sits between
provider readiness and future provider selection.

## Decision

The reserved registry snapshot fixture is:

```text
docs/development/current/main/design/allocator-provider-registry-snapshot-v0.toml
```

The snapshot is constructed only from explicit diagnostic inputs:

```text
provider manifest report
provider readiness preflight report
```

No file discovery, environment override, `.inc` name matching, or runner-owned
provider behavior is allowed.

## Snapshot Contract

The registry snapshot contains provider entries copied from the provider
manifest vocabulary:

```text
provider_id
provider_kind
role
operations
state = "reserved"
activation = "future_row_required"
```

The snapshot remains inactive:

```text
provider_selection = "inactive"
would_build_registry = false
would_select_provider = false
would_activate = false
activation = "future_row_required"
```

The reserved diagnostics are:

```text
[allocator-provider/registry-snapshot-missing]
[allocator-provider/registry-provider-missing]
[allocator-provider/registry-capability-missing]
```

## Required Facts

A future implementation row must fail fast when these facts are missing:

```text
provider_manifest_ready
provider_readiness_preflight_ready
provider_entries_nonempty
provider_ids_reserved_set
provider_operations_nonempty
registry_owner_named
no_hidden_environment_toggle
no_implicit_manifest_discovery
no_app_or_facade_name_matching
```

## Stop Line

M77 keeps these inactive:

- runtime provider registry implementation;
- provider selection implementation;
- provider selection CLI;
- provider selection environment toggles;
- implicit runtime file-system manifest discovery;
- runtime hook install/uninstall body;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

## Next Row

M78 may consume this snapshot only as an explicit diagnostic input to a
reserved selection request/decision shape. M78 must still keep
`would_select_provider = false`, no selected provider id, no activation, and no
runtime registry implementation.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_registry_snapshot_guard.sh
bash tools/checks/k2_wide_allocator_provider_activation_entry_contract_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
