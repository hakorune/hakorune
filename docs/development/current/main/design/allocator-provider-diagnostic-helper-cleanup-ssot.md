---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M97B allocator provider diagnostic helper cleanup.
Related:
  - src/runtime/allocator_provider_toml_helpers.rs
  - src/runtime/allocator_provider_registry.rs
  - src/runtime/allocator_provider_activation_decision.rs
  - tools/checks/k2_wide_allocator_provider_diagnostic_helper_cleanup_guard.sh
---

# Allocator Provider Diagnostic Helper Cleanup (SSOT)

## Goal

Keep allocator provider diagnostic TOML parsing helpers in one runtime-internal
owner:

```text
src/runtime/allocator_provider_toml_helpers.rs
```

M97B is a behavior-preserving BoxShape cleanup. It does not add a runtime
provider registry, provider selection, proof consumption, rollback preparation,
activation gate opening, hook activation, native activation, or process
allocator replacement.

## Shared Helpers

The shared owner provides:

```text
DiagnosticFactCheck
text_field_matches(...)
bool_field_false(...)
nonempty_text_field(...)
string_list_contains_all(...)
```

The consumers are:

```text
src/runtime/allocator_provider_registry.rs
src/runtime/allocator_provider_activation_decision.rs
```

These files must not reintroduce private copies of the shared TOML helpers or
the old per-report fact-check structs:

```text
RegistrySnapshotFactCheck
SelectionDecisionFactCheck
ActivationSafetyFactCheck
ActivationDecisionFactCheck
```

## Deferred Cleanup

The report structs still expose the `would_*` fields flatly because that is the
CLI contract today. Replacing those fields with an embedded
`AllocatorProviderDiagnosticInactiveActions` value is a larger API cleanup and
must be handled in a separate row with CLI output and test updates.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_diagnostic_helper_cleanup_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
