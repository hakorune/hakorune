# 293x-146 M93 Allocator Provider Registry Snapshot Diagnostic Report

Status: Completed
Date: 2026-05-11

## Scope

M93 adds the diagnostic-only runtime report for the reserved allocator provider
registry snapshot fixture.

## Landed

- Added `AllocatorProviderRegistrySnapshotFacts`,
  `AllocatorProviderRegistrySnapshotReport`, and
  `AllocatorProviderRegistrySnapshotStatus` in
  `src/runtime/allocator_provider_registry.rs`.
- Added
  `validate_allocator_provider_registry_snapshot_from_text(...)` for
  caller-provided TOML text.
- Fixed complete, empty, missing-capability, and malformed TOML tests for the
  registry snapshot report.
- Added the M93 SSOT and dedicated guard.

## Stop Line

M93 does not build an active runtime registry, select a provider, consume
proofs, prepare rollback, open the activation gate, install hooks, replace the
process allocator, add environment discovery, add implicit file discovery, or
add a CLI route.

## Verification

```bash
cargo test -q registry_snapshot -- --nocapture
bash tools/checks/k2_wide_allocator_provider_registry_snapshot_diagnostic_report_guard.sh
git diff --check
```
