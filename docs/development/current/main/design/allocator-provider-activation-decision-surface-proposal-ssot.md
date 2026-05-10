---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M86 allocator provider activation decision surface proposal.
Related:
  - docs/development/current/main/design/allocator-provider-activation-safety-closeout-inventory-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-safety-gate-ssot.md
  - docs/development/current/main/design/allocator-provider-selection-decision-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-ssot.md
  - docs/development/current/main/design/allocator-provider-rollback-preflight-ssot.md
  - tools/checks/k2_wide_allocator_provider_activation_decision_surface_proposal_guard.sh
---

# Allocator Provider Activation Decision Surface Proposal (SSOT)

## Goal

Define the future activation decision surface before adding any implementation.

M86 is proposal-only. It names the explicit input/output contract that a later
row may turn into a fixture, parser, report, or CLI surface. It does not add a
runtime decision parser, a new CLI flag, provider selection, proof consumption,
rollback preparation, hook activation, or process allocator replacement.

## Future Explicit Surface

The proposed future CLI entry is explicit input only:

```text
hakorune --allocator-provider-activation-decision <ACTIVATION_DECISION_TOML>
```

The input path must be caller-provided. The runtime must not discover provider
manifests, proof bundles, activation safety reports, or rollback facts from the
file system or environment.

## Proposed Input Shape

The future `allocator_provider_activation_decision_v0` input should be a pure
decision bundle:

```toml
surface_version = "allocator_provider_activation_decision_v0"
operator_intent = "diagnose"
requested_provider_id = "mimalloc"
activation_safety_gate_report_path = "activation-safety-gate-v0.toml"
registry_snapshot_path = "registry-snapshot-v0.toml"
selection_decision_path = "selection-decision-v0.toml"
proof_bundle_report_path = "proof-bundle-consumption-v0.toml"
rollback_preflight_report_path = "rollback-preflight-v0.toml"
```

The bundle is a handoff shape. It does not grant permission to activate. A
later implementation must fail fast if any referenced diagnostic says the gate
is closed, proof is missing, rollback is not prepared, provider selection is
absent, or operator intent is not an explicitly supported value.

## Proposed Output Shape

The first implementation of this surface must remain diagnostic until a later
activation row explicitly changes the stop line:

```text
activation_decision_surface_status=proposal_only
activation_decision_allowed=false
would_select_provider=false
would_consume_proof=false
would_prepare_rollback=false
would_open_activation_gate=false
would_install_hook=false
would_replace_process_allocator=false
would_activate=false
```

## Stop Line

M86 does not authorize:

- runtime provider registry implementation;
- provider selection implementation;
- runtime proof consumption implementation;
- rollback preparation/execution;
- activation gate opening;
- hook activation implementation;
- provider selection environment toggles;
- implicit manifest/proof/report discovery;
- `#[global_allocator]`;
- `GlobalAlloc`;
- process allocator replacement;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

## Next Row

The next safe row is M87 activation decision fixture contract. It may add only a
reserved TOML fixture plus a guard for the proposed shape above. It must not add
runtime parsing, CLI routing, provider selection, proof consumption, rollback
preparation, hook activation, `#[global_allocator]`, process allocator
replacement, environment discovery, or `.inc` name matching.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_activation_decision_surface_proposal_guard.sh
git diff --check
```
