---
Status: Landed
Decision: accepted
Date: 2026-05-11
Scope: M97B allocator provider diagnostic helper cleanup.
Related:
  - docs/development/current/main/design/allocator-provider-diagnostic-helper-cleanup-ssot.md
  - src/runtime/allocator_provider_toml_helpers.rs
  - src/runtime/allocator_provider_registry.rs
  - src/runtime/allocator_provider_activation_decision.rs
  - tools/checks/k2_wide_allocator_provider_diagnostic_helper_cleanup_guard.sh
---

# 293x-152 M97B Allocator Provider Diagnostic Helper Cleanup

## Result

M97B extracts duplicated allocator provider diagnostic TOML helper predicates
and the per-report fact-check shape into:

```text
src/runtime/allocator_provider_toml_helpers.rs
```

The cleanup is behavior-preserving. It keeps the existing diagnostic report and
CLI output contracts flat and unchanged.

## Inactive Contract

M97B does not add active provider behavior:

- no active registry construction;
- no provider selection;
- no proof consumption;
- no rollback preparation;
- no activation gate opening;
- no hook installation;
- no process allocator replacement;
- no native activation.

## Deferred Work

The repeated `would_*` report fields remain flat in this row. That cleanup has
a wider public test and CLI-output blast radius and should be handled as a
separate behavior-preserving API cleanup.
