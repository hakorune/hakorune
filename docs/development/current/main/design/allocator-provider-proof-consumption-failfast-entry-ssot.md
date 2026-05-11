---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M101 allocator provider proof consumption fail-fast entry.
Related:
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-entry-contract-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-diagnostic-report-ssot.md
  - src/runtime/allocator_provider_activation.rs
  - tools/checks/k2_wide_allocator_provider_proof_consumption_failfast_entry_guard.sh
---

# Allocator Provider Proof Consumption Fail-Fast Entry (SSOT)

## Goal

Move from diagnostic-only rows into the first small runtime behavior row without
activating allocator replacement.

M101 creates the reserved M100 entry as a fail-fast runtime API:

```text
allocator_provider_proof_bundle_consumption_attempt(...)
```

The entry reads an already-built proof-bundle consumption diagnostic report. It
does not read environment variables, discover files, select a provider, consume
proofs, prepare rollback, open an activation gate, install hooks, activate a
native allocator, or replace the process allocator.

## Runtime Owner

The owner is:

```text
src/runtime/allocator_provider_activation.rs
```

The diagnostic report owner remains:

```text
src/runtime/allocator_provider_proof_bundle_consumption.rs
```

The M99 CLI remains diagnostic-only and is not a behavior entry.

## Behavior Contract

When the proof-bundle report is ready but `selected_provider_id` is absent or
`none_reserved`, the attempt returns:

```text
status=BlockedMissingSelectedProvider
diagnostic=[allocator-provider/proof-bundle-consumption-selected-provider-missing]
selected_provider_required=true
selected_provider_id_absent=true
proof_bundle_valid_for_requested_provider=true
proof_bundle_consumed=false
would_select_provider=false
would_consume_proof_bundle=false
would_prepare_rollback=false
would_open_activation_gate=false
would_install_hook=false
would_replace_process_allocator=false
would_activate=false
```

Malformed or incomplete proof-bundle diagnostic reports return:

```text
status=BlockedMissingProofBundleReport
diagnostic=[allocator-provider/proof-bundle-consumption-report-missing]
proof_bundle_valid_for_requested_provider=false
proof_bundle_consumed=false
```

## Stop Line

M101 keeps these inactive:

- active runtime provider registry construction;
- provider selection;
- proof consumption;
- rollback preparation or execution;
- activation gate opening;
- hook activation or native activation;
- provider/proof environment toggles;
- implicit runtime file-system manifest/report/proof discovery;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

## Next Row

M102 may validate a caller-provided selected provider precondition. It still
must not implement provider selection or consume proofs.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_proof_consumption_failfast_entry_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
