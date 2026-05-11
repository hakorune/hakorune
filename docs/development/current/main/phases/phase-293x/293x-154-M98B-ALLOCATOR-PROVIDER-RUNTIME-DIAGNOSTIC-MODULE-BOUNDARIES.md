# 293x-154 M98B Allocator Provider Runtime Diagnostic Module Boundaries

Status: complete
Date: 2026-05-11

## Summary

M98B splits the allocator provider runtime diagnostic reports out of the large
registry file while preserving the historical public API path.

## Changed

- Added report-owner modules:
  - `src/runtime/allocator_provider_registry_snapshot.rs`
  - `src/runtime/allocator_provider_selection_decision.rs`
  - `src/runtime/allocator_provider_proof_bundle_consumption.rs`
  - `src/runtime/allocator_provider_activation_safety.rs`
- Added shared contract/helpers:
  - `src/runtime/allocator_provider_registry_common.rs`
- Kept `src/runtime/allocator_provider_registry.rs` as a thin facade.
- Moved historical facade regression tests to
  `src/runtime/allocator_provider_registry_facade_tests.rs`.
- Updated older allocator-provider guards so implementation checks target the
  new owning modules instead of the facade.

## Stop Line

M98B is behavior-preserving cleanup only. It does not add active registry
construction, provider selection, proof consumption, rollback preparation,
activation gate opening, hook activation, native allocator activation, or
process allocator replacement.

## Verification

```bash
cargo test -q registry_snapshot -- --nocapture
cargo test -q selection_decision -- --nocapture
cargo test -q proof_bundle_consumption -- --nocapture
cargo test -q activation_safety -- --nocapture
bash tools/checks/k2_wide_allocator_provider_runtime_diagnostic_module_boundaries_guard.sh
```
