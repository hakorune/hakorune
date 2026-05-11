---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M84 allocator provider activation safety diagnostic CLI surface.
Related:
  - docs/development/current/main/design/allocator-provider-activation-safety-diagnostic-report-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-safety-gate-v0.toml
  - src/runtime/allocator_provider_registry.rs
  - src/cli/allocator_provider_activation_safety.rs
  - tools/checks/k2_wide_allocator_provider_activation_safety_cli_surface_guard.sh
---

# Allocator Provider Activation Safety CLI Surface (SSOT)

## Goal

Expose the M83 activation safety report through an explicit diagnostic CLI
surface without opening the activation gate or adding allocator activation.

## Decision

The accepted CLI shape is:

```text
hakorune --allocator-provider-activation-safety-gate <ACTIVATION_SAFETY_GATE_TOML>
```

The CLI reads only the path explicitly passed by the caller. It does not search
default locations, read environment variables, consume provider proofs, prepare
rollback, open the activation gate, activate hooks, select a provider, install
a registry, or replace the process allocator.

## Output Contract

The CLI prints stable key/value diagnostics:

```text
diagnostic=[allocator-provider/activation-safety-blocked]
activation_safety_status=ready_gate_closed
parse_error=
missing_facts=
missing_diagnostics=
rollback_target_provider_id=native_mimalloc
activation_target_provider_id=native_mimalloc
safety_status=reserved_gate_closed
activation_gate_open=false
would_open_activation_gate=false
would_activate_hook=false
would_activate=false
```

Exit code:

- `0`: the activation safety diagnostic is complete and the gate remains
  closed (`ready_gate_closed`).
- `2`: file read error, malformed TOML, missing required activation safety
  facts, or missing required diagnostics.

Malformed TOML is still diagnostic output when the file can be read. The
`parse_error` field is flattened to one line for stable CLI consumption.

## Ownership

- `src/runtime/allocator_provider_registry.rs` owns activation safety facts,
  status, diagnostics, and the gate-closed report.
- `src/cli/allocator_provider_activation_safety.rs` owns explicit file input
  and CLI output formatting.
- `src/main.rs` may early-exit after CLI parsing.
- `src/runner/**` does not own this surface.
- `.inc` remains a reader/emitter only and does not infer allocator provider
  behavior.

## Stop Line

M84 keeps these inactive:

- activation gate opening;
- runtime provider selection implementation;
- runtime provider registry implementation;
- runtime proof consumption implementation;
- provider rollback preparation/execution;
- hook activation implementation;
- provider selection environment toggles, including `NYASH_ALLOCATOR_PROVIDER`,
  `HAKO_ALLOCATOR_PROVIDER`, and broad `ALLOCATOR_PROVIDER_*` names;
- implicit runtime file-system manifest discovery;
- implicit provider proof discovery;
- implicit hook plan discovery;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_activation_safety_cli_surface_guard.sh
cargo test -q allocator_provider_activation_safety -- --nocapture
git diff --check
```
