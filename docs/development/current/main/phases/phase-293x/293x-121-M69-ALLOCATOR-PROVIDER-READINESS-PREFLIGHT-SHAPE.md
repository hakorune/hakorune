---
Status: Completed
Date: 2026-05-10
Scope: M69 allocator provider readiness preflight shape.
Related:
  - docs/development/current/main/design/allocator-provider-readiness-preflight-ssot.md
  - docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md
  - src/runtime/allocator_provider_manifest.rs
  - tools/checks/k2_wide_allocator_provider_readiness_preflight_guard.sh
---

# 293x-121 M69 Allocator Provider Readiness Preflight Shape

## Summary

M69 adds a diagnostic-only provider readiness preflight report that ties the
M67 provider manifest parser to the M63 allocator hook activation preflight
report.

The new runtime shape reports:

- provider manifest readiness;
- hook activation preflight readiness;
- reserved provider id set presence;
- stable missing-fact names;
- `would_select_provider = false`;
- `would_activate = false`.

## Boundary

This card does not add provider selection, a provider registry, implicit
manifest discovery, hook activation, or process allocator replacement. It only
creates the facts M70 can compose into a combined hook/provider dry-run report.

## Files

- `src/runtime/allocator_provider_manifest.rs`
- `docs/development/current/main/design/allocator-provider-readiness-preflight-ssot.md`
- `tools/checks/k2_wide_allocator_provider_readiness_preflight_guard.sh`

## Verification

```bash
cargo test -q allocator_provider
bash tools/checks/k2_wide_allocator_provider_readiness_preflight_guard.sh
bash tools/checks/k2_wide_allocator_provider_manifest_cli_surface_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
