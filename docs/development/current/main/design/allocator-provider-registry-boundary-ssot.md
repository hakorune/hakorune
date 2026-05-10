---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: M71 allocator provider registry boundary docs.
Related:
  - docs/development/current/main/design/allocator-provider-boundary-v0-ssot.md
  - docs/development/current/main/design/allocator-provider-manifest-v0-ssot.md
  - docs/development/current/main/design/allocator-provider-combined-dry-run-ssot.md
---

# Allocator Provider Registry Boundary (SSOT)

## Goal

Name the future provider registry ownership and API boundary before any active
registry implementation exists.

M71 is a docs-only boundary row. It does not add runtime provider registry
code, provider selection, provider environment toggles, implicit manifest
discovery, hook activation, or process allocator replacement.

## Decision

The future registry owner is the runtime allocator provider boundary. If a
later row implements it, the owner path should be:

```text
src/runtime/allocator_provider_registry.rs
```

That file must remain absent in M71.

The future registry may only be introduced after a later row defines:

- registry construction input;
- explicit provider manifest source;
- selection policy owner;
- fail-fast behavior for missing provider ids;
- activation-preflight dependency;
- rollback behavior;
- test and guard ownership.

## Future API Shape

The future registry API must be explicit and data-shaped. The boundary names
these future concepts without implementing them:

```text
ProviderRegistryEntry
ProviderRegistrySnapshot
ProviderRegistryBuildInput
ProviderSelectionRequest
ProviderSelectionDecision
```

The registry must not read files or environment variables. Its inputs must be
caller-provided values that already passed manifest parsing and readiness
preflight diagnostics.

## Layer Contract

The boundary is:

```text
provider manifest parser
  -> provider readiness preflight
  -> combined dry-run report
  -> future registry build input
  -> future selection decision
  -> future activation row
```

M71 stops before `future registry build input` becomes code.

## Selection Contract

Provider selection remains forbidden in M71. A future selection row must define
all of these before code exists:

- deterministic provider ordering;
- required operation set;
- unsupported provider diagnostic;
- missing capability diagnostic;
- no hidden environment override;
- no `.inc` name matching;
- no process allocator replacement without an activation row.

Until that row lands, every diagnostic report must keep:

```text
would_select_provider = false
would_activate = false
```

## Stop Line

M71 keeps these inactive:

- runtime provider registry implementation;
- provider selection implementation;
- provider selection CLI;
- provider selection environment toggles;
- implicit runtime file-system manifest discovery;
- runtime hook install/uninstall body;
- process allocator replacement;
- `#[global_allocator]`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_registry_boundary_guard.sh
bash tools/checks/k2_wide_allocator_provider_combined_dry_run_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
