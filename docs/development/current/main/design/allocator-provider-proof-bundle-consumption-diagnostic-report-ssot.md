---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M98 allocator provider proof bundle consumption diagnostic report.
Related:
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-v0.toml
  - docs/development/current/main/design/allocator-provider-selection-decision-diagnostic-report-ssot.md
  - docs/development/current/main/design/allocator-provider-diagnostic-inactive-actions-ssot.md
  - tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_diagnostic_report_guard.sh
---

# Allocator Provider Proof Bundle Consumption Diagnostic Report (SSOT)

## Goal

Add a diagnostic-only runtime report for caller-provided
`allocator_provider_proof_bundle_consumption_v0` TOML text.

M98 promotes the M79 reserved proof-bundle consumption shape from fixture-only
to a runtime parser/report. It does not consume proofs, select a provider,
build an active provider registry, prepare rollback, open an activation gate,
install hooks, activate a native allocator, or replace the process allocator.

## Runtime Owner

The report owner remains:

```text
src/runtime/allocator_provider_registry.rs
```

The public diagnostic entry is:

```text
validate_allocator_provider_proof_bundle_consumption_from_text(...)
```

The report types are:

```text
AllocatorProviderProofBundleConsumptionFacts
AllocatorProviderProofBundleConsumptionReport
AllocatorProviderProofBundleConsumptionStatus
```

## Report Contract

Complete caller-provided proof-bundle consumption TOML reports:

```text
proof_bundle_consumption_status=ready_inactive
diagnostic=[allocator-provider/proof-bundle-consumption-inactive]
requested_provider_id=native_mimalloc
selected_provider_id=none_reserved
selected_provider_id_absent=true
requested_operations=alloc,realloc,free
candidate_provider_ids=native_system_malloc,native_mimalloc,hako_model_allocator,debug_guarded_allocator
provider_proof_ids=native_system_malloc,native_mimalloc,hako_model_allocator,debug_guarded_allocator
provider_proof_count=4
proof_bundle_consumed=false
active_registry_built=false
would_build_registry=false
would_select_provider=false
would_consume_proof_bundle=false
would_prepare_rollback=false
would_open_activation_gate=false
would_install_hook=false
would_replace_process_allocator=false
would_activate=false
```

Malformed TOML reports `parse_error`, `missing_facts=parse_toml`, and keeps all
inactive output booleans false.

Missing or invalid facts fail fast through stable diagnostics from the M79
shape:

```text
[allocator-provider/proof-bundle-consumption-missing]
[allocator-provider/proof-bundle-registry-missing]
[allocator-provider/proof-bundle-selection-missing]
[allocator-provider/proof-bundle-provider-proof-missing]
[allocator-provider/proof-bundle-provider-mismatch]
[allocator-provider/proof-bundle-capability-missing]
[allocator-provider/proof-bundle-activation-blocked]
```

## Stop Line

M98 keeps these inactive:

- active runtime provider registry construction;
- provider selection;
- proof consumption;
- rollback preparation or execution;
- activation gate opening;
- hook activation or native activation;
- proof-bundle environment toggles, including `NYASH_ALLOCATOR_PROVIDER`,
  `HAKO_ALLOCATOR_PROVIDER`, and broad `ALLOCATOR_PROVIDER_*` names;
- implicit runtime file-system manifest/report/proof discovery;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

M98 does not add a CLI route. A later row may expose this report through an
explicit CLI diagnostic surface.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_diagnostic_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
