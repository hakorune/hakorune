---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M94 allocator provider registry snapshot diagnostic CLI surface.
Related:
  - docs/development/current/main/design/allocator-provider-registry-snapshot-diagnostic-report-ssot.md
  - docs/development/current/main/design/allocator-provider-registry-snapshot-v0.toml
  - src/runtime/allocator_provider_registry.rs
  - src/cli/allocator_provider_registry_snapshot.rs
  - tools/checks/k2_wide_allocator_provider_registry_snapshot_cli_surface_guard.sh
---

# Allocator Provider Registry Snapshot CLI Surface (SSOT)

## Goal

Expose the M93 registry snapshot report through an explicit diagnostic CLI
surface without building an active provider registry or selecting a provider.

## Decision

The accepted CLI shape is:

```text
hakorune --allocator-provider-registry-snapshot <REGISTRY_SNAPSHOT_TOML>
```

The CLI reads only the path explicitly passed by the caller. It does not search
default locations, read environment variables, build a runtime provider
registry, select providers, consume proofs, prepare rollback, open the
activation gate, activate hooks, install native allocator hooks, or replace the
process allocator.

Allocator diagnostic CLI modes are mutually exclusive. The one allowed
multi-flag mode is the combined hook/provider dry-run, where
`--allocator-provider-manifest` is paired with the allocator hook dry-run
plan/proof inputs. Any other combination of allocator diagnostic surfaces must
fail fast with:

```text
[allocator-diagnostic/cli-conflicting-modes]
```

## Output Contract

The CLI prints stable key/value diagnostics:

```text
diagnostic=[allocator-provider/registry-snapshot-inactive]
registry_snapshot_status=ready_inactive
parse_error=
missing_facts=
missing_diagnostics=
provider_ids=native_system_malloc,native_mimalloc,hako_model_allocator,debug_guarded_allocator
provider_count=4
active_registry_built=false
would_build_registry=false
would_select_provider=false
would_consume_proof=false
would_prepare_rollback=false
would_open_activation_gate=false
would_install_hook=false
would_replace_process_allocator=false
would_activate=false
```

Exit code:

- `0`: the registry snapshot diagnostic input is complete and remains
  inactive (`ready_inactive`).
- `2`: file read error, malformed TOML, missing required registry snapshot
  facts, or missing required diagnostics.

Malformed TOML is still diagnostic output when the file can be read. The
`parse_error` field is flattened to one line for stable CLI consumption.

## Ownership

- `src/runtime/allocator_provider_registry.rs` owns registry snapshot facts,
  status, diagnostics, and the inactive report.
- `src/cli/allocator_provider_registry_snapshot.rs` owns explicit file input
  and CLI output formatting.
- `src/main.rs` may early-exit after CLI parsing.
- `src/runner/**` does not own this surface.
- `.inc` remains a reader/emitter only and does not infer allocator provider
  behavior.

## Stop Line

M94 keeps these inactive:

- active runtime provider registry construction;
- provider selection;
- provider proof consumption;
- provider rollback preparation/execution;
- activation gate opening;
- hook activation implementation;
- hidden provider selection environment toggles, including
  `NYASH_ALLOCATOR_PROVIDER`, `HAKO_ALLOCATOR_PROVIDER`, and broad
  `ALLOCATOR_PROVIDER_*` names;
- implicit runtime file-system manifest/report/proof discovery;
- implicit hook plan discovery;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_registry_snapshot_cli_surface_guard.sh
cargo test -q allocator_provider_registry_snapshot -- --nocapture
git diff --check
```
