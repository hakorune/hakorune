---
Status: Completed
Decision: accepted
Date: 2026-05-10
Scope: M68 allocator provider manifest CLI diagnostic surface.
Related:
  - docs/development/current/main/design/allocator-provider-manifest-cli-surface-ssot.md
  - src/cli/allocator_provider_manifest.rs
  - tools/checks/k2_wide_allocator_provider_manifest_cli_surface_guard.sh
---

# 293x-120 M68 Allocator Provider Manifest CLI Surface

## Goal

Expose the M67 provider manifest parser through an explicit CLI diagnostic
surface.

## Result

CLI now accepts:

```text
hakorune --allocator-provider-manifest <PROVIDER_MANIFEST_TOML>
```

The route reads only the explicitly provided file, prints stable key/value
diagnostics, exits before runner execution, and keeps
`would_select_provider=false`.

## Non-Goals

This card does not add:

- runtime provider registry;
- provider selection;
- provider selection environment toggles;
- process allocator replacement;
- `#[global_allocator]`;
- implicit runtime file-system manifest discovery;
- `.inc` hook/provider/facade/policy name matching;
- allocator activation route widening.

## Verification

```bash
bash tools/checks/k2_wide_allocator_provider_manifest_cli_surface_guard.sh
bash tools/checks/k2_wide_allocator_provider_manifest_parser_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
