---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M90 allocator provider activation decision diagnostic CLI surface.
Related:
  - docs/development/current/main/design/allocator-provider-activation-decision-diagnostic-report-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-decision-v0.toml
  - src/runtime/allocator_provider_activation_decision.rs
  - src/cli/allocator_provider_activation_decision.rs
  - tools/checks/k2_wide_allocator_provider_activation_decision_cli_surface_guard.sh
---

# Allocator Provider Activation Decision CLI Surface (SSOT)

## Goal

Expose the M89 activation decision report through an explicit diagnostic CLI
surface without selecting or activating an allocator provider.

## Decision

The accepted CLI shape is:

```text
hakorune --allocator-provider-activation-decision <ACTIVATION_DECISION_TOML>
```

The CLI reads only the caller-provided TOML path. It does not search default
locations, read environment variables, select providers, consume proof bundles,
prepare rollback, open the activation gate, activate hooks, install native
allocator hooks, or replace the process allocator.

## Output Contract

The CLI prints stable key/value diagnostics:

```text
diagnostic=[allocator-provider/activation-decision-blocked]
activation_decision_status=ready_blocked
parse_error=
missing_facts=
missing_diagnostics=
operator_intent=diagnose
requested_provider_id=mimalloc
activation_safety_gate_report_path=activation-safety-gate-v0.toml
registry_snapshot_path=registry-snapshot-v0.toml
selection_decision_path=selection-decision-v0.toml
proof_bundle_report_path=proof-bundle-consumption-v0.toml
rollback_preflight_report_path=rollback-preflight-v0.toml
activation_decision_surface_status=reserved_fixture
activation_decision_allowed=false
would_select_provider=false
would_consume_proof=false
would_prepare_rollback=false
would_open_activation_gate=false
would_install_hook=false
would_replace_process_allocator=false
would_activate=false
```

Exit code:

- `0`: the activation decision diagnostic input is complete and remains
  blocked (`ready_blocked`).
- `2`: file read error, malformed TOML, missing required facts, or missing
  required diagnostics.

Malformed TOML is still diagnostic output when the file can be read. The
`parse_error` field is flattened to one line for stable CLI consumption.

## Ownership

- `src/runtime/allocator_provider_activation_decision.rs` owns activation
  decision facts, status, diagnostics, and the blocked report.
- `src/cli/allocator_provider_activation_decision.rs` owns explicit file input
  and CLI output formatting.
- `src/main.rs` may early-exit after CLI parsing.
- `src/runner/**` does not own this surface.
- `.inc` remains a reader/emitter only and does not infer allocator provider
  behavior.

## Stop Line

M90 keeps these inactive:

- provider selection;
- provider proof consumption;
- provider rollback preparation/execution;
- activation gate opening;
- hook activation implementation;
- hidden provider selection environment toggles;
- implicit runtime file-system manifest/report/proof discovery;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_activation_decision_cli_surface_guard.sh
cargo test -q allocator_provider_activation_decision -- --nocapture
git diff --check
```
