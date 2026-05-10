---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: M70 combined hook/provider dry-run report.
Related:
  - docs/development/current/main/design/allocator-provider-readiness-preflight-ssot.md
  - docs/development/current/main/design/allocator-provider-manifest-cli-surface-ssot.md
  - docs/development/current/main/design/allocator-hook-dry-run-cli-surface-ssot.md
  - src/runtime/allocator_provider_manifest.rs
  - src/cli/allocator_provider_manifest.rs
---

# Allocator Provider Combined Dry-Run (SSOT)

## Goal

Compose the explicit allocator hook plan, allocator hook activation proof, and
allocator provider manifest diagnostics into one report without activating a
hook, selecting a provider, installing a registry, or replacing the process
allocator.

M70 is a diagnostic composition row. It is not a provider registry row and it
is not an allocator activation row.

## Decision

The accepted CLI shape reuses the existing explicit file arguments:

```text
hakorune \
  --allocator-hook-dry-run \
  --allocator-hook-plan <HOOK_PLAN_TOML> \
  --allocator-hook-proof <ACTIVATION_PROOF_TOML> \
  --allocator-provider-manifest <PROVIDER_MANIFEST_TOML>
```

The CLI reads only these explicit paths. It does not discover manifests, read
environment variables, select a provider, install a registry, or replace the
process allocator.

Standalone `--allocator-hook-dry-run` and standalone
`--allocator-provider-manifest` remain valid diagnostic surfaces.

## Output Contract

The combined report prints stable key/value diagnostics:

```text
diagnostic=[allocator-provider/combined-dry-run-ready]
combined_status=ready
hook_id=hako_alloc.production.v0
hook_dry_run_diagnostic=[allocator-hook/dry-run-ready]
activation_proof_diagnostic=[allocator-hook/activation-proof-ready]
activation_preflight_diagnostic=[allocator-hook/activation-preflight-ready]
provider_manifest_diagnostic=[allocator-provider/manifest-ready]
provider_readiness_diagnostic=[allocator-provider/readiness-preflight-ready]
provider_ids=native_system_malloc,native_mimalloc,hako_model_allocator,debug_guarded_allocator
missing_facts=
would_install=false
would_select_provider=false
would_activate=false
```

Exit code:

- `0`: hook dry-run, activation proof, activation preflight, provider manifest,
  and provider readiness preflight are all ready diagnostics.
- `2`: any required input file cannot be read, any report is missing facts, or
  any reserved TOML contract is invalid.

## Missing Facts

M70 reports only combined-readiness missing facts:

- `hook_dry_run_ready`
- `activation_proof_ready`
- `activation_preflight_ready`
- `provider_manifest_ready`
- `provider_readiness_preflight_ready`

Sub-report diagnostics stay visible as separate output keys. M70 does not
duplicate lower-level missing-fact lists into the combined fact list.

## Ownership

- `src/runtime/allocator_provider_manifest.rs` owns the combined report data
  shape and report composition.
- `src/cli/allocator_provider_manifest.rs` owns explicit file input and CLI
  output formatting for the combined provider/hook report.
- `src/main.rs` dispatches the combined report before the standalone hook or
  provider diagnostics so one invocation has one diagnostic owner.
- `src/runner/**` does not own this behavior.

## Stop Line

M70 keeps these inactive:

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
bash tools/checks/k2_wide_allocator_provider_combined_dry_run_guard.sh
bash tools/checks/k2_wide_allocator_provider_readiness_preflight_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
