---
Status: Landed
Date: 2026-04-27
Scope: Close GenericMethodRoute decision record split lane
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-291x/291x-451-next-lane-selection-card.md
  - docs/development/current/main/phases/phase-291x/291x-452-generic-method-route-decision-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-453-generic-method-route-decision-record-card.md
---

# 291x-454: GenericMethodRoute Decision Closeout

## Goal

Close the `GenericMethodRoute` decision record split lane and return the phase
to next-lane selection.

This card is closeout only. No code changed.

## Result

- `291x-451` selected the lane.
- `291x-452` inventoried decided backend metadata that still lived as flat
  fields on `GenericMethodRoute`.
- `291x-453` landed `GenericMethodRouteDecision` and preserved existing route
  matching, JSON output, helper symbols, and lowering tiers.

## Verification

The implementation card passed:

```bash
cargo test -q generic_method_route
cargo test -q build_mir_json_root_emits_generic_method_routes
cargo test -q map_lookup_fusion
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

`tools/checks/dev_gate.sh quick` emitted the existing chip8 release-artifact
sync warning, but the profile finished `ok`.

## Next

Choose the next compiler-cleanliness lane. Do not continue `.inc` migration,
MapGet lowering, or Stage-B work by momentum.
