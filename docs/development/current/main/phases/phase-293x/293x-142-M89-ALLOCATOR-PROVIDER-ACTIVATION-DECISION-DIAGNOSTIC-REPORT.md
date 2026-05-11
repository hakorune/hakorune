---
Status: Completed
Date: 2026-05-11
Scope: M89 allocator provider activation decision diagnostic report.
Related:
  - docs/development/current/main/design/allocator-provider-activation-decision-diagnostic-report-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-decision-v0.toml
  - src/runtime/allocator_provider_activation_decision.rs
  - tools/checks/k2_wide_allocator_provider_activation_decision_diagnostic_report_guard.sh
---

# 293x-142 M89 Allocator Provider Activation Decision Diagnostic Report

## Summary

M89 adds the runtime diagnostic report for caller-provided
`allocator_provider_activation_decision_v0` TOML text.

The report can identify a complete reserved decision bundle, but the decision
remains blocked:

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

## Boundary

This is runtime diagnostic parsing only. It does not add CLI routing, provider
selection, proof consumption, rollback preparation, activation gate opening,
hook activation, `#[global_allocator]`, process allocator replacement,
environment discovery, route widening, or `.inc` name matching.

## Verification

```bash
bash tools/checks/k2_wide_allocator_provider_activation_decision_diagnostic_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
