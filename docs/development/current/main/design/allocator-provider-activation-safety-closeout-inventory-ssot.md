---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M85 allocator provider activation safety closeout inventory.
Related:
  - docs/development/current/main/design/allocator-provider-activation-entry-contract-ssot.md
  - docs/development/current/main/design/allocator-provider-registry-snapshot-ssot.md
  - docs/development/current/main/design/allocator-provider-selection-decision-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-ssot.md
  - docs/development/current/main/design/allocator-provider-rollback-preflight-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-safety-gate-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-safety-diagnostic-owner-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-safety-diagnostic-report-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-safety-cli-surface-ssot.md
  - tools/checks/k2_wide_allocator_provider_activation_safety_closeout_guard.sh
---

# Allocator Provider Activation Safety Closeout Inventory (SSOT)

## Goal

Close out the M76-M84 activation safety diagnostic ladder as an inventory row
before any later allocator activation decision.

M85 adds no runtime behavior. It only verifies that the activation-entry,
registry snapshot, selection decision, proof bundle, rollback preflight,
activation safety gate, diagnostic owner, diagnostic report, and explicit CLI
surface artifacts are present and wired into the guard index.

## Inventory

| Row | Artifact class | Required output |
| --- | --- | --- |
| M76 | activation entry contract | SSOT, fixture, card, guard |
| M77 | registry snapshot | SSOT, fixture, card, guard |
| M78 | selection decision | SSOT, fixture, card, guard |
| M79 | proof bundle consumption | SSOT, fixture, card, guard |
| M80 | rollback preflight | SSOT, fixture, card, guard |
| M81 | activation safety gate | SSOT, fixture, card, guard |
| M82 | diagnostic owner | SSOT, card, guard |
| M83 | diagnostic report | SSOT, runtime report, card, guard |
| M84 | CLI surface | SSOT, explicit CLI route, card, guard |

## Closeout Contract

M85 proves coverage only. It does not authorize:

- activation gate opening;
- runtime provider registry implementation;
- runtime provider selection implementation;
- runtime proof consumption implementation;
- rollback preparation/execution;
- hook activation implementation;
- provider selection environment toggles;
- implicit runtime file-system manifest discovery;
- implicit provider proof discovery;
- implicit hook plan discovery;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

The closeout also proves that past M76-M84 guards do not pin
`CURRENT_STATE.latest_card` or `CURRENT_STATE.latest_card_path`. Past guards may
prove their own artifacts and forbidden activation behavior, but only the
current card guard may own the latest-card pointer.

The only accepted CLI surface in this ladder remains explicit input:

```text
hakorune --allocator-provider-activation-safety-gate <ACTIVATION_SAFETY_GATE_TOML>
```

and the output remains gate-closed:

```text
activation_gate_open=false
would_open_activation_gate=false
would_activate_hook=false
would_activate=false
```

## Next Row

M85 does not open an activation implementation row. A later row may propose the
next activation decision surface only if it is docs-first, fail-fast, and keeps
runtime activation inactive until a separate implementation row explicitly
changes the stop line.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_activation_safety_closeout_guard.sh
git diff --check
```
