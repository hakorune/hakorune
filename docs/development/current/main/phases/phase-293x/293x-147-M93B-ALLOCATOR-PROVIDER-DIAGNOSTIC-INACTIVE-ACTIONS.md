# 293x-147 M93B Allocator Provider Diagnostic Inactive Actions

Status: landed

## Goal

Make the inactive output contract shared across allocator provider diagnostic
reports before adding the M94 CLI surface.

## Scope

- Add `src/runtime/allocator_provider_diagnostic_inactive.rs` as the code-side
  SSOT for diagnostic-only false outputs.
- Route M83 activation safety, M89 activation decision, and M93 registry
  snapshot reports through that inactive-action source.
- Keep report public fields and behavior unchanged.
- Remove the M93 guard's current-card pin so future rows are not blocked.

## Must Not Add

- provider selection;
- proof consumption;
- rollback preparation;
- activation gate opening;
- hook activation;
- process allocator replacement;
- implicit file discovery or environment toggles.

## Verification

```bash
bash tools/checks/k2_wide_allocator_provider_diagnostic_inactive_actions_guard.sh
cargo test -q allocator_provider_inactive -- --nocapture
cargo test -q activation_safety -- --nocapture
cargo test -q activation_decision -- --nocapture
cargo test -q registry_snapshot -- --nocapture
```
