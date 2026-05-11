---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M91 allocator provider activation decision closeout inventory.
Related:
  - docs/development/current/main/design/allocator-provider-activation-decision-surface-proposal-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-decision-v0.toml
  - docs/development/current/main/design/allocator-provider-activation-decision-diagnostic-owner-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-decision-diagnostic-report-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-decision-cli-surface-ssot.md
  - tools/checks/k2_wide_allocator_provider_activation_decision_closeout_guard.sh
---

# Allocator Provider Activation Decision Closeout Inventory (SSOT)

## Goal

Close out the M86-M90 activation decision diagnostic ladder as an inventory row
before any later provider activation implementation.

M91 adds no runtime behavior. It verifies that the activation decision proposal,
fixture, diagnostic owner, diagnostic report, and explicit CLI surface artifacts
are present and wired into the guard index.

## Inventory

| Row | Artifact class | Required output |
| --- | --- | --- |
| M86 | activation decision surface proposal | SSOT, card, guard |
| M87 | activation decision fixture contract | fixture, card, guard |
| M88 | diagnostic owner | SSOT, card, guard |
| M89 | diagnostic report | SSOT, runtime report, card, guard |
| M90 | CLI surface | SSOT, explicit CLI route, card, guard |

## Closeout Contract

M91 proves coverage only. It does not authorize:

- provider selection;
- provider proof consumption;
- rollback preparation/execution;
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

The closeout also proves that past M86-M90 guards do not pin
`CURRENT_STATE.latest_card` or `CURRENT_STATE.latest_card_path`. Past guards may
prove their own artifacts and forbidden activation behavior, but only the
current card guard may own the latest-card pointer.

The only accepted CLI surface in this ladder remains explicit input:

```text
hakorune --allocator-provider-activation-decision <ACTIVATION_DECISION_TOML>
```

and the output remains blocked:

```text
activation_decision_allowed=false
would_select_provider=false
would_consume_proof=false
would_prepare_rollback=false
would_open_activation_gate=false
would_install_hook=false
would_replace_process_allocator=false
would_activate=false
```

## Next Row

M91 does not open an activation implementation row. A later row may start the
next provider activation implementation only if it names one owner, keeps proof
consumption/rollback/gate opening/hook activation separate, and changes the
stop line with a dedicated guard.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_activation_decision_closeout_guard.sh
git diff --check
```
