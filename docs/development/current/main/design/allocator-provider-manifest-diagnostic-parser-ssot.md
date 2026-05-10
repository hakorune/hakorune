---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: M67 allocator provider manifest diagnostic parser.
Related:
  - docs/development/current/main/design/allocator-provider-manifest-v0-ssot.md
  - docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md
  - src/runtime/allocator_provider_manifest.rs
---

# Allocator Provider Manifest Diagnostic Parser (SSOT)

## Goal

Accept caller-provided `allocator_provider_manifest_v0` TOML text as a
diagnostic parser input and report reserved provider facts without selecting a
provider.

## Decision

M67 adds a runtime diagnostic parser/report surface:

```text
parse_allocator_provider_manifest_text(manifest_toml)
```

The parser is diagnostic-only and always reports:

- `would_select_provider = false`.

## Report Shape

The runtime report includes:

- `status` (`ready` / `missing_or_invalid`);
- stable diagnostic tags (`[allocator-provider/manifest-ready]`,
  `[allocator-provider/manifest-missing]`);
- parsed `provider_ids`;
- `missing_facts` for invalid/missing reserved fields.

## Reserved Validation Contract

The parser validates:

- top-level `schema_version = "allocator_provider_manifest_v0"`;
- `status = "reserved"`;
- `active = false`;
- `provider_selection = "inactive"`;
- `activation = "future_row_required"`;
- non-empty `providers` rows;
- per-provider `provider_id` / `provider_kind` / `role`;
- per-provider `state = "reserved"` and
  `activation = "future_row_required"`;
- per-provider non-empty `operations`;
- provider-id set matches the reserved M64 ids.

## Stop Line

M67 keeps these inactive:

- runtime provider registry;
- provider selection;
- provider selection environment toggles;
- process allocator replacement;
- `#[global_allocator]`;
- implicit runtime file-system manifest discovery;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_manifest_parser_guard.sh
bash tools/checks/k2_wide_allocator_provider_task_breakdown_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
