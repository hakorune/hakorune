---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M82 allocator provider activation safety gate diagnostic owner.
Related:
  - docs/development/current/main/design/allocator-provider-activation-safety-gate-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-safety-gate-v0.toml
  - docs/development/current/main/design/allocator-provider-registry-boundary-ssot.md
  - docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md
---

# Allocator Provider Activation Safety Diagnostic Owner (SSOT)

## Goal

Name the runtime diagnostic owner for the allocator provider activation safety
gate before adding any report implementation.

M82 is an owner/guard hygiene row. It does not open the activation gate, select
a provider, consume proof bundles, prepare rollback, activate hooks, discover
files implicitly, or replace the process allocator.

## Decision

Allocator provider activation safety diagnostics belong to the provider
registry owner:

```text
src/runtime/allocator_provider_registry.rs
```

Reserved future API shape:

```text
validate_allocator_provider_activation_safety_gate(evidence)
  -> AllocatorProviderActivationSafetyReport
```

The future report may observe only explicit diagnostic inputs from the M81
activation safety fixture. It must keep:

```text
activation_gate_open = false
would_open_activation_gate = false
would_activate_hook = false
would_activate = false
```

## Ownership Rules

### Provider Registry Owner

May own future diagnostic report construction for:

- activation safety input presence;
- missing input diagnostic selection;
- activation target presence;
- gate-closed status;
- `would_* = false` reporting.

Must not own:

- provider manifest parsing;
- hook plan parsing;
- hook activation proof parsing;
- provider proof semantics;
- rollback execution;
- hook installation;
- process allocator replacement;
- environment variable opt-in;
- `.inc` provider/hook/facade matching.

### Existing Diagnostic Owners

`src/runtime/allocator_provider_manifest.rs` keeps provider manifest,
readiness, and combined dry-run diagnostics.

`src/runtime/allocator_hook_dry_run.rs` keeps hook dry-run, activation proof,
and activation preflight diagnostics.

### Runner, Apps, and `.inc`

Runner code, real apps, and `.inc` shims must not become allocator provider
activation safety owners.

## Current M82 State

```text
runtime diagnostic owner:
  named

runtime owner file:
  not required by M82

activation safety report implementation:
  future row

activation gate opening:
  absent

hook activation:
  absent

process allocator replacement:
  absent
```

## Past Guard Hygiene

M82 makes older provider guards future-compatible:

- past guards may keep blocking active provider selection, environment toggles,
  hook activation, rollback execution, and process allocator replacement;
- past guards must not require the future provider registry owner file to stay
  absent;
- past guards must not reject diagnostic owner/type names such as registry
  snapshot, selection decision, proof bundle, rollback preflight, or activation
  safety report names.

This keeps the ladder one-way: M83 can add diagnostic report code in the named
owner without rewriting unrelated guards again.

## Stop Line

M82 keeps these inactive:

- activation safety report implementation;
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

M83 may add the diagnostic-only activation safety report in the named runtime
owner. It must consume explicit inputs only, keep the gate closed, and preserve
all `would_*` activation fields as false.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_owner_guard.sh
bash tools/checks/k2_wide_allocator_provider_activation_safety_gate_guard.sh
git diff --check
```
