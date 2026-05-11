---
Status: Completed
Date: 2026-05-11
Scope: M96 allocator provider selection decision diagnostic report.
Related:
  - docs/development/current/main/design/allocator-provider-selection-decision-diagnostic-report-ssot.md
  - docs/development/current/main/design/allocator-provider-selection-decision-v0.toml
  - src/runtime/allocator_provider_registry.rs
  - tools/checks/k2_wide_allocator_provider_selection_decision_diagnostic_report_guard.sh
---

# 293x-150 M96 Allocator Provider Selection Decision Diagnostic Report

## Result

M96 adds the diagnostic-only runtime parser/report for caller-provided
`allocator_provider_selection_decision_v0` TOML text.

The new runtime surface is:

```text
validate_allocator_provider_selection_decision_from_text(...)
AllocatorProviderSelectionDecisionFacts
AllocatorProviderSelectionDecisionReport
AllocatorProviderSelectionDecisionStatus
```

It reports complete, missing, invalid, and malformed selection decision input
without selecting a provider.

## Inactive Contract

M96 keeps:

- `active_registry_built=false`
- `would_build_registry=false`
- `would_select_provider=false`
- `would_consume_proof=false`
- `would_prepare_rollback=false`
- `would_open_activation_gate=false`
- `would_install_hook=false`
- `would_replace_process_allocator=false`
- `would_activate=false`

M96 does not add a CLI route, environment toggle, implicit discovery path,
provider selection implementation, activation gate opening, hook activation, or
process allocator replacement.

## Verification

```bash
cargo test -q selection_decision -- --nocapture
bash tools/checks/k2_wide_allocator_provider_selection_decision_diagnostic_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
