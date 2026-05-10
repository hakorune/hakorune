---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: M69 allocator provider readiness preflight shape.
Related:
  - docs/development/current/main/design/allocator-provider-manifest-diagnostic-parser-ssot.md
  - docs/development/current/main/design/allocator-provider-manifest-cli-surface-ssot.md
  - docs/development/current/main/design/allocator-hook-activation-preflight-ssot.md
  - src/runtime/allocator_provider_manifest.rs
---

# Allocator Provider Readiness Preflight (SSOT)

## Goal

Tie provider manifest readiness to the existing allocator hook activation
preflight diagnostics without selecting a provider, activating a hook, or
replacing the process allocator.

M69 is a diagnostic data-shape row. It is not a provider registry row and it is
not an activation row.

## Decision

The accepted runtime surface is:

```text
validate_allocator_provider_readiness_preflight(
  provider_manifest_report,
  activation_preflight_report,
)

validate_allocator_provider_readiness_preflight_from_manifest_texts(
  provider_manifest_toml,
  hook_plan_toml,
  activation_proof_toml,
)
```

The report observes only caller-provided TOML text and already-formed
diagnostic reports. It does not read files, discover manifests, read
environment variables, install a registry, select a provider, or activate a
replacement allocator.

## Output Contract

The M69 report has stable provider-readiness facts:

- `provider_manifest_ready`
- `activation_preflight_ready`
- `provider_ids_reserved_set`

Ready diagnostic:

```text
diagnostic = "[allocator-provider/readiness-preflight-ready]"
missing_facts = []
would_select_provider = false
would_activate = false
```

Missing diagnostic:

```text
diagnostic = "[allocator-provider/readiness-preflight-missing]"
missing_facts = ["provider_manifest_ready", ...]
would_select_provider = false
would_activate = false
```

The report also preserves the provider manifest diagnostic and hook activation
preflight diagnostic so M70 can compose a combined dry-run report without
re-parsing or re-deciding readiness.

## Ownership

- `src/runtime/allocator_provider_manifest.rs` owns provider manifest parsing
  and provider readiness preflight reporting.
- `src/runtime/allocator_hook_dry_run.rs` owns hook activation preflight facts.
- M69 may reference the hook activation preflight report but must not widen the
  hook activation path.
- CLI composition is deferred to M70.

## Stop Line

M69 keeps these inactive:

- runtime provider registry;
- provider selection;
- provider selection environment toggles;
- implicit runtime file-system manifest discovery;
- runtime hook install/uninstall body;
- process allocator replacement;
- `#[global_allocator]`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_readiness_preflight_guard.sh
bash tools/checks/k2_wide_allocator_provider_manifest_cli_surface_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
