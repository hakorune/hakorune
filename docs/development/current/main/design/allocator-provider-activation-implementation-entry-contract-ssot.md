---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M92 allocator provider activation implementation entry contract.
Related:
  - docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-decision-closeout-inventory-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-implementation-entry-contract-v0.toml
  - tools/checks/k2_wide_allocator_provider_activation_implementation_entry_contract_guard.sh
---

# Allocator Provider Activation Implementation Entry Contract (SSOT)

## Goal

Name one activation implementation owner and one activation entry before any
provider activation behavior is added.

M92 is docs/fixture/guard only. It does not create an active provider registry,
select a provider, consume proofs, prepare rollback, open the activation gate,
install allocator hooks, or replace the process allocator.

## Decision

The future activation implementation owner is:

```text
src/runtime/allocator_provider_activation.rs
```

The future entry name is:

```text
allocator_provider_activation_attempt
```

This owner is intentionally separate from the existing diagnostic owners:

- `src/runtime/allocator_provider_registry.rs` owns registry snapshot and
  activation safety diagnostics.
- `src/runtime/allocator_provider_activation_decision.rs` owns activation
  decision diagnostics.
- `src/runtime/allocator_provider_activation.rs` will own future activation
  attempt orchestration only after a later guarded row creates that behavior.

M92 does not require the owner source file to exist. It only reserves the owner
and entry names so later rows do not spread activation logic across diagnostic
parsers.

## Reserved Fixture

The contract is represented by:

```text
docs/development/current/main/design/allocator-provider-activation-implementation-entry-contract-v0.toml
```

Required fixture facts:

```text
activation_implementation_owner_named
activation_attempt_entry_named
activation_decision_report_explicit
registry_snapshot_report_explicit
selection_decision_report_explicit
proof_bundle_report_explicit
rollback_preflight_report_explicit
activation_safety_gate_report_explicit
fail_fast_activation_attempt_diagnostic_named
no_hidden_environment_toggle
no_implicit_manifest_discovery
no_implicit_report_discovery
no_provider_selection_implementation
no_proof_consumption_implementation
no_rollback_preparation_implementation
no_activation_gate_opening
no_hook_activation_implementation
no_process_allocator_replacement
```

The reserved fail-fast diagnostic root is:

```text
[allocator-provider/activation-implementation-entry-missing]
```

## Stop Line

M92 changes only the named owner/entry boundary. These remain inactive:

- active runtime provider registry or provider selection implementation;
- proof consumption implementation;
- rollback preparation or execution;
- activation gate opening;
- hook activation implementation;
- provider selection environment toggles, including `NYASH_ALLOCATOR_PROVIDER`,
  `HAKO_ALLOCATOR_PROVIDER`, and broad `ALLOCATOR_PROVIDER_*` names;
- implicit runtime file-system manifest/report/proof discovery;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

All M92 reports and fixtures keep:

```text
activation_implementation_status = "owner_entry_reserved"
would_build_registry = false
would_select_provider = false
would_consume_proof = false
would_prepare_rollback = false
would_open_activation_gate = false
would_install_hook = false
would_replace_process_allocator = false
would_activate = false
```

## Next Ladder

M92 opens the post-decision implementation ladder without adding activation:

| Row | Task | Output | Must Not Add |
| --- | --- | --- | --- |
| M92 | activation implementation entry contract | this SSOT + reserved fixture + guard | runtime activation code |
| M93 | registry snapshot diagnostic report | runtime report over caller-provided registry snapshot TOML text | active registry |
| M94 | registry snapshot CLI surface | explicit CLI over caller-provided registry snapshot TOML path | implicit discovery |
| M95 | activation diagnostic closeout inventory | coverage guard for M92-M94/M93B artifacts | runtime activation |
| M96 | selection decision diagnostic report | runtime report over caller-provided selection decision TOML text | provider selection |
| M97 | selection decision CLI surface | explicit CLI over caller-provided selection decision TOML path | activation |

Proof consumption, rollback preparation, gate opening, hook activation, and
process allocator replacement each require their own later guarded row.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_activation_implementation_entry_contract_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
