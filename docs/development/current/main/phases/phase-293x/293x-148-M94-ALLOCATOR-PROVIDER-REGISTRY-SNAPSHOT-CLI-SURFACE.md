# 293x-148 M94 Allocator Provider Registry Snapshot CLI Surface

Status: landed

## Goal

Expose the M93 allocator provider registry snapshot diagnostic report through
an explicit CLI surface.

## Scope

- Add `hakorune --allocator-provider-registry-snapshot <REGISTRY_SNAPSHOT_TOML>`.
- Read only the caller-provided TOML path.
- Print stable key/value output for the inactive registry snapshot report.
- Reuse the allocator diagnostic CLI conflict guard.

## Must Not Add

- active registry construction;
- provider selection;
- proof consumption;
- rollback preparation;
- activation gate opening;
- hook activation;
- process allocator replacement;
- hidden environment toggles;
- implicit manifest/report/proof discovery.

## Verification

```bash
bash tools/checks/k2_wide_allocator_provider_registry_snapshot_cli_surface_guard.sh
cargo test -q allocator_provider_registry_snapshot -- --nocapture
```
