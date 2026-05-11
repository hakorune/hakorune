---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M96 allocator provider selection decision diagnostic report.
Related:
  - docs/development/current/main/design/allocator-provider-selection-decision-ssot.md
  - docs/development/current/main/design/allocator-provider-selection-decision-v0.toml
  - docs/development/current/main/design/allocator-provider-registry-snapshot-diagnostic-report-ssot.md
  - docs/development/current/main/design/allocator-provider-diagnostic-inactive-actions-ssot.md
  - tools/checks/k2_wide_allocator_provider_selection_decision_diagnostic_report_guard.sh
---

# Allocator Provider Selection Decision Diagnostic Report (SSOT)

## Goal

Add a diagnostic-only runtime report for caller-provided
`allocator_provider_selection_decision_v0` TOML text.

M96 promotes the M78 reserved selection decision shape from fixture-only to a
runtime parser/report. It does not select a provider, build an active provider
registry, consume proofs, prepare rollback, open an activation gate, install
hooks, activate a native allocator, or replace the process allocator.

## Runtime Owner

The report owner remains:

```text
src/runtime/allocator_provider_registry.rs
```

The public diagnostic entry is:

```text
validate_allocator_provider_selection_decision_from_text(...)
```

The report types are:

```text
AllocatorProviderSelectionDecisionFacts
AllocatorProviderSelectionDecisionReport
AllocatorProviderSelectionDecisionStatus
```

## Report Contract

Complete caller-provided selection decision TOML reports:

```text
selection_decision_status=ready_inactive
diagnostic=[allocator-provider/selection-decision-inactive]
requested_provider_id=native_mimalloc
selected_provider_id=none_reserved
selected_provider_id_absent=true
active_registry_built=false
would_build_registry=false
would_select_provider=false
would_consume_proof=false
would_prepare_rollback=false
would_open_activation_gate=false
would_install_hook=false
would_replace_process_allocator=false
would_activate=false
```

Malformed TOML reports `parse_error`, `missing_facts=parse_toml`, and keeps all
inactive output booleans false.

Missing or invalid facts fail fast through stable diagnostics from the M78
shape:

```text
[allocator-provider/selection-decision-missing]
[allocator-provider/selection-registry-missing]
[allocator-provider/selection-request-missing]
[allocator-provider/selection-unsupported-provider]
[allocator-provider/selection-capability-missing]
[allocator-provider/selection-ambiguous]
```

## Stop Line

M96 keeps these inactive:

- active runtime provider registry construction;
- provider selection;
- proof consumption;
- rollback preparation or execution;
- activation gate opening;
- hook activation or native activation;
- provider selection environment toggles, including `NYASH_ALLOCATOR_PROVIDER`,
  `HAKO_ALLOCATOR_PROVIDER`, and broad `ALLOCATOR_PROVIDER_*` names;
- implicit runtime file-system manifest/report/proof discovery;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

M96 does not add a CLI route. M97 may expose this report through an explicit CLI diagnostic surface.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_selection_decision_diagnostic_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
