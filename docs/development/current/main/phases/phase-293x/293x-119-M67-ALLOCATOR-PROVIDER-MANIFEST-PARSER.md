---
Status: Completed
Decision: accepted
Date: 2026-05-10
Scope: M67 allocator provider manifest diagnostic parser.
Related:
  - docs/development/current/main/design/allocator-provider-manifest-diagnostic-parser-ssot.md
  - src/runtime/allocator_provider_manifest.rs
  - tools/checks/k2_wide_allocator_provider_manifest_parser_guard.sh
---

# 293x-119 M67 Allocator Provider Manifest Parser

## Goal

Implement M67 as a diagnostic-only parser/report for caller-provided provider
manifest TOML text.

## Result

Runtime now exposes:

- `AllocatorProviderManifestStatus`
- `AllocatorProviderManifestReport`
- `parse_allocator_provider_manifest_text(...)`

The parser validates reserved manifest/provider-row facts and always keeps
`would_select_provider = false`.

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
bash tools/checks/k2_wide_allocator_provider_manifest_parser_guard.sh
bash tools/checks/k2_wide_allocator_provider_task_breakdown_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
