---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M93 allocator provider registry snapshot diagnostic report.
Related:
  - docs/development/current/main/design/allocator-provider-registry-snapshot-ssot.md
  - docs/development/current/main/design/allocator-provider-registry-snapshot-v0.toml
  - docs/development/current/main/design/allocator-provider-activation-implementation-entry-contract-ssot.md
  - tools/checks/k2_wide_allocator_provider_registry_snapshot_diagnostic_report_guard.sh
---

# Allocator Provider Registry Snapshot Diagnostic Report (SSOT)

## Goal

Add a diagnostic-only runtime report for the M77 registry snapshot fixture.

M93 parses caller-provided registry snapshot TOML text only. It does not expose
an implicit CLI route, discover files implicitly, build an active runtime
registry, select a provider, consume proofs, prepare rollback, open the
activation gate, install hooks, or replace the process allocator.

## Runtime Owner

```text
src/runtime/allocator_provider_registry.rs
```

Public diagnostic entry:

```text
validate_allocator_provider_registry_snapshot_from_text(text)
  -> AllocatorProviderRegistrySnapshotReport
```

## Output Contract

For a complete reserved fixture, the report is ready but inactive:

```text
status = ReadyInactive
diagnostic = [allocator-provider/registry-snapshot-inactive]
provider_count = 4
active_registry_built = false
would_build_registry = false
would_select_provider = false
would_consume_proof = false
would_prepare_rollback = false
would_open_activation_gate = false
would_install_hook = false
would_replace_process_allocator = false
would_activate = false
```

For missing or malformed input:

```text
status = MissingFacts
diagnostic = [allocator-provider/registry-snapshot-missing]
parse_error = Some(...) only for malformed TOML
missing_facts = stable fact names
missing_diagnostics = stable diagnostic tags
```

Reserved diagnostics:

```text
[allocator-provider/registry-snapshot-missing]
[allocator-provider/registry-provider-missing]
[allocator-provider/registry-capability-missing]
[allocator-provider/registry-snapshot-inactive]
```

## Stop Line

M93 itself keeps these inactive:

- active runtime provider registry construction;
- implicit registry snapshot CLI route or runtime file discovery;
- provider selection;
- provider proof consumption;
- rollback preparation/execution;
- activation gate opening;
- hook activation implementation;
- implicit runtime file-system manifest/report/proof discovery;
- provider environment toggles;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

## Next Row

M94 may expose this report through an explicit CLI diagnostic surface:

```text
hakorune --allocator-provider-registry-snapshot <REGISTRY_SNAPSHOT_TOML>
```

It must remain explicit-input only and keep all registry/activation fields
false.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_registry_snapshot_diagnostic_report_guard.sh
cargo test -q registry_snapshot -- --nocapture
git diff --check
```
