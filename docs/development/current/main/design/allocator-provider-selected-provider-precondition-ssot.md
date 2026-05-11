---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M102 allocator provider selected-provider precondition.
Related:
  - docs/development/current/main/design/allocator-provider-post-m101-implementation-ladder-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-consumption-failfast-entry-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-diagnostic-report-ssot.md
  - src/runtime/allocator_provider_activation.rs
  - tools/checks/k2_wide_allocator_provider_selected_provider_precondition_guard.sh
---

# Allocator Provider Selected-Provider Precondition (SSOT)

## Goal

M102 advances from "selected provider absent" fail-fast to a caller-provided
selected-provider precondition. It checks whether an explicit selected provider
matches the requested provider in the proof-bundle diagnostic report.

M102 does not implement provider selection. The caller must provide the selected
provider id.

## Runtime Entry

The runtime owner remains:

```text
src/runtime/allocator_provider_activation.rs
```

M102 adds:

```text
allocator_provider_selected_provider_precondition_attempt(...)
```

The existing M101 entry remains:

```text
allocator_provider_proof_bundle_consumption_attempt(...)
```

The M101 entry continues to use the selected provider id in the diagnostic
report, so the reserved `none_reserved` fixture still blocks.

## Behavior Contract

If the proof-bundle report is incomplete or malformed, M102 returns:

```text
status=BlockedMissingProofBundleReport
diagnostic=[allocator-provider/proof-bundle-consumption-report-missing]
proof_bundle_consumed=false
```

If the caller-provided selected provider is absent, empty, or `none_reserved`,
M102 returns:

```text
status=BlockedMissingSelectedProvider
diagnostic=[allocator-provider/proof-bundle-consumption-selected-provider-missing]
selected_provider_required=true
selected_provider_id_absent=true
proof_bundle_consumed=false
```

If the selected provider differs from `requested_provider_id` or has no proof
entry in the report, M102 returns:

```text
status=BlockedSelectedProviderMismatch
diagnostic=[allocator-provider/proof-bundle-consumption-selected-provider-mismatch]
proof_bundle_consumed=false
```

If the selected provider matches `requested_provider_id` and has a proof entry,
M102 returns:

```text
status=ReadySelectedProviderPrecondition
diagnostic=[allocator-provider/proof-bundle-consumption-selected-provider-ready]
selected_provider_required=true
selected_provider_id_absent=false
proof_bundle_valid_for_requested_provider=true
proof_bundle_consumed=false
```

## Inactive Contract

M102 keeps all allocator activation behavior inactive:

```text
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

No provider is selected, no proof is consumed, no rollback is prepared, no gate
opens, no hook is installed, no native allocator is activated, and the process
allocator is not replaced.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_selected_provider_precondition_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
