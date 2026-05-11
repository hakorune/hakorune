---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M95 allocator provider activation diagnostic closeout inventory.
Related:
  - docs/development/current/main/design/allocator-provider-activation-implementation-entry-contract-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-implementation-entry-contract-v0.toml
  - docs/development/current/main/design/allocator-provider-registry-snapshot-diagnostic-report-ssot.md
  - docs/development/current/main/design/allocator-provider-diagnostic-inactive-actions-ssot.md
  - docs/development/current/main/design/allocator-provider-registry-snapshot-cli-surface-ssot.md
  - tools/checks/k2_wide_allocator_provider_activation_diagnostic_closeout_guard.sh
---

# Allocator Provider Activation Diagnostic Closeout Inventory (SSOT)

## Goal

Close out the post-decision activation diagnostic entry ladder from M92 through
M94, including the M93B inactive-action cleanup, before any later selection
decision diagnostic report.

M95 is coverage-only. It does not add runtime provider registry construction,
provider selection, proof consumption, rollback preparation, activation gate
opening, hook activation, native activation, or process allocator replacement.

## Inventory

| Row | Artifact class | Required output |
| --- | --- | --- |
| M92 | activation implementation entry contract | SSOT, reserved fixture, card, guard |
| M93 | registry snapshot diagnostic report | SSOT, runtime report, card, guard |
| M93B | diagnostic inactive actions cleanup | SSOT, code-side inactive output source, card, guard |
| M94 | registry snapshot CLI surface | SSOT, explicit CLI route, card, guard |

## Closeout Contract

M95 proves coverage only. It does not authorize:

- active runtime provider registry construction;
- provider selection;
- proof consumption;
- rollback preparation or execution;
- activation gate opening;
- hook activation or native activation;
- provider selection environment toggles, including `NYASH_ALLOCATOR_PROVIDER`,
  `HAKO_ALLOCATOR_PROVIDER`, and broad `ALLOCATOR_PROVIDER_*` names;
- implicit runtime file-system manifest/report/proof discovery;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

The code-side source for false diagnostic outputs remains:

```text
src/runtime/allocator_provider_diagnostic_inactive.rs
```

with:

```text
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

The only accepted registry snapshot CLI surface remains explicit input:

```text
hakorune --allocator-provider-registry-snapshot <REGISTRY_SNAPSHOT_TOML>
```

Past M92-M94/M93B guards must not pin `CURRENT_STATE.latest_card` or
`CURRENT_STATE.latest_card_path`. Past guards may prove their own artifacts and
forbidden activation behavior, but only the current card guard may own the
latest-card pointer.

## Next Row

M95 does not open activation behavior. The next safe row is M96 selection decision diagnostic report:
a runtime report over caller-provided selection decision TOML text that still
must not select a provider or activate anything.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_activation_diagnostic_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
