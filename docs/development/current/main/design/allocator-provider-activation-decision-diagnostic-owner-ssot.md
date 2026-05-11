---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M88 allocator provider activation decision diagnostic owner.
Related:
  - docs/development/current/main/design/allocator-provider-activation-decision-surface-proposal-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-decision-v0.toml
  - docs/development/current/main/design/allocator-provider-lightweight-doc-sync-policy-ssot.md
---

# Allocator Provider Activation Decision Diagnostic Owner (SSOT)

## Goal

Name the runtime diagnostic owner for the allocator provider activation decision
surface before adding report implementation.

M88 is an owner/guard hygiene row. It does not parse activation decisions in
runtime code, expose a CLI route, select a provider, consume proof bundles,
prepare rollback, open the activation gate, activate hooks, discover files
implicitly, or replace the process allocator.

## Decision

Allocator provider activation decision diagnostics belong to a separate runtime
diagnostic owner:

```text
src/runtime/allocator_provider_activation_decision.rs
```

Reserved future API shape:

```text
validate_allocator_provider_activation_decision(decision_bundle)
  -> AllocatorProviderActivationDecisionReport
```

This owner may observe only the caller-provided M87 activation decision fixture
shape and explicit diagnostic paths. It must keep:

```text
activation_decision_allowed = false
would_select_provider = false
would_consume_proof = false
would_prepare_rollback = false
would_open_activation_gate = false
would_install_hook = false
would_replace_process_allocator = false
would_activate = false
```

## Ownership Rules

May own future diagnostic report construction for:

- activation decision bundle presence;
- explicit diagnostic path presence;
- operator intent validation;
- missing input diagnostic selection;
- blocked activation decision status;
- `would_* = false` reporting.

Must not own:

- provider registry construction;
- provider selection;
- proof bundle consumption;
- rollback preparation or execution;
- activation gate opening;
- hook installation;
- process allocator replacement;
- environment variable opt-in;
- implicit file-system discovery;
- `.inc` provider/hook/facade matching.

## Current M88 State

```text
runtime diagnostic owner:
  named

runtime owner file:
  future row

activation decision report implementation:
  future row

activation decision CLI:
  future row

activation:
  absent
```

## Past Guard Hygiene

M88 makes the M87 fixture guard future-compatible:

- M87 may keep proving the reserved fixture fields and inactive `would_*`
  values;
- M87 must not require `decision_surface_owner = "future_row_required"` once
  the owner is named;
- M87 must not block future diagnostic owner/type names in `src/`;
- M87 must not block a future explicit CLI route once a later CLI row lands.

## Stop Line

M88 keeps these inactive:

- runtime activation decision report implementation;
- activation decision CLI route;
- activation gate opening;
- runtime provider selection implementation;
- runtime proof consumption implementation;
- provider rollback preparation/execution;
- hook activation implementation;
- implicit runtime file-system manifest/report/proof discovery;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

## Next Row

M89 may add the diagnostic-only activation decision report in the named runtime
owner. It must consume caller-provided TOML text only, keep activation blocked,
and preserve all `would_*` activation fields as false.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_activation_decision_diagnostic_owner_guard.sh
bash tools/checks/k2_wide_allocator_provider_activation_decision_fixture_contract_guard.sh
git diff --check
```
