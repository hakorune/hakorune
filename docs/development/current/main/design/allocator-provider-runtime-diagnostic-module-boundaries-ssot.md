---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M98B allocator provider runtime diagnostic module boundaries.
Related:
  - src/runtime/allocator_provider_registry.rs
  - src/runtime/allocator_provider_registry_snapshot.rs
  - src/runtime/allocator_provider_selection_decision.rs
  - src/runtime/allocator_provider_proof_bundle_consumption.rs
  - src/runtime/allocator_provider_activation_safety.rs
  - src/runtime/allocator_provider_registry_common.rs
  - src/runtime/allocator_provider_registry_facade_tests.rs
  - tools/checks/k2_wide_allocator_provider_runtime_diagnostic_module_boundaries_guard.sh
---

# Allocator Provider Runtime Diagnostic Module Boundaries (SSOT)

## Goal

Keep allocator provider diagnostic runtime code split by report owner, with
`src/runtime/allocator_provider_registry.rs` acting only as the stable facade for
the historical API path.

## Boundaries

| Module | Owns |
| --- | --- |
| `allocator_provider_registry.rs` | public re-exports for the historical registry API path |
| `allocator_provider_registry_facade_tests.rs` | regression tests that prove the historical facade path still reaches each diagnostic report |
| `allocator_provider_registry_snapshot.rs` | registry snapshot facts, report, diagnostics, and TOML reader |
| `allocator_provider_selection_decision.rs` | selection decision facts, report, diagnostics, and TOML reader |
| `allocator_provider_proof_bundle_consumption.rs` | proof-bundle consumption facts, report, diagnostics, and TOML reader |
| `allocator_provider_activation_safety.rs` | activation safety gate facts, report, diagnostics, and TOML reader |
| `allocator_provider_registry_common.rs` | owner path, reserved provider-id set, and shared TOML string-list readers |

## Invariants

- `allocator_provider_registry.rs` remains a thin facade under 200 lines.
- Regression tests do not live in the facade; they live in
  `allocator_provider_registry_facade_tests.rs`.
- Owner strings remain `src/runtime/allocator_provider_registry.rs` until a
  separate fixture/docs migration changes that contract.
- Public callers may keep importing through
  `crate::runtime::allocator_provider_registry::*`.
- The split must not add active registry construction, provider selection, proof
  consumption, rollback preparation, activation gate opening, hook activation,
  native allocator activation, or process allocator replacement.
- Guards that validate implementation internals must point at the owning module,
  not the facade.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_runtime_diagnostic_module_boundaries_guard.sh
git diff --check
```
