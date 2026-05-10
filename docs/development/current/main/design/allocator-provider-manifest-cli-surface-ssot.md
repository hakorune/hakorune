---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: M68 allocator provider manifest CLI diagnostic surface.
Related:
  - docs/development/current/main/design/allocator-provider-manifest-diagnostic-parser-ssot.md
  - docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md
  - src/runtime/allocator_provider_manifest.rs
  - src/cli/allocator_provider_manifest.rs
---

# Allocator Provider Manifest CLI Surface (SSOT)

## Goal

Expose the M67 allocator provider manifest parser through an explicit
diagnostic CLI surface without adding provider selection, environment toggles,
implicit manifest discovery, or allocator replacement.

## Decision

The accepted CLI shape is:

```text
hakorune --allocator-provider-manifest <PROVIDER_MANIFEST_TOML>
```

The CLI reads only the path explicitly passed by the user. It does not search
default locations, read environment variables, select a provider, install a
registry, or replace the process allocator.

## Output Contract

The CLI prints stable key/value diagnostics:

```text
diagnostic=[allocator-provider/manifest-ready]
manifest_status=ready
provider_ids=native_system_malloc,native_mimalloc,hako_model_allocator,debug_guarded_allocator
missing_facts=
would_select_provider=false
```

Exit code:

- `0`: provider manifest is a ready diagnostic.
- `2`: file read error, invalid TOML, missing required manifest facts, or
  non-reserved provider rows.

## Ownership

- `src/runtime/allocator_provider_manifest.rs` owns manifest parsing and
  reserved provider facts.
- `src/cli/allocator_provider_manifest.rs` owns explicit file input and CLI
  output formatting.
- `src/main.rs` may early-exit after CLI parsing.
- `src/runner/**` does not own this surface.
- `.inc` remains a reader/emitter only and does not infer allocator provider
  behavior.

## Stop Line

M68 keeps these inactive:

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
bash tools/checks/k2_wide_allocator_provider_manifest_cli_surface_guard.sh
bash tools/checks/k2_wide_allocator_provider_manifest_parser_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
