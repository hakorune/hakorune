---
Status: Completed
Date: 2026-05-11
Scope: M83 allocator provider activation safety diagnostic report.
Related:
  - docs/development/current/main/design/allocator-provider-activation-safety-diagnostic-report-ssot.md
  - src/runtime/allocator_provider_registry.rs
  - tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_report_guard.sh
---

# 293x-135 M83 Allocator Provider Activation Safety Diagnostic Report

## Summary

M83 adds the runtime-owned diagnostic report for the allocator provider
activation safety gate:

```text
src/runtime/allocator_provider_registry.rs
```

The report validates caller-provided activation safety TOML text and returns a
gate-closed diagnostic. It has no activation success state.

## Boundary

M83 does not add CLI/env toggles, implicit manifest/proof/hook-plan discovery,
provider selection, proof consumption, rollback preparation, activation gate
opening, hook activation, `#[global_allocator]`, `GlobalAlloc`, process
allocator replacement, route widening, or `.inc` name matching.

The implementation keeps:

```text
activation_gate_open = false
would_open_activation_gate = false
would_activate_hook = false
would_activate = false
```

## Verification

```bash
bash -n tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_report_guard.sh
bash tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_report_guard.sh
cargo test -q activation_safety -- --nocapture
git diff --check
```
