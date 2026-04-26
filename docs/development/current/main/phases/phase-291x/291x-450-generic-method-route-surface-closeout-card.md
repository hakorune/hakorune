---
Status: Landed
Date: 2026-04-27
Scope: Close GenericMethodRoute surface/decision split lane
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-291x/291x-447-next-lane-selection-card.md
  - docs/development/current/main/phases/phase-291x/291x-448-generic-method-route-surface-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-449-generic-method-route-surface-record-card.md
---

# 291x-450: GenericMethodRoute Surface Closeout

## Goal

Close the `GenericMethodRoute` surface/decision split lane and return the
phase to next-lane selection.

This card is closeout only. No code changed.

## Result

- `291x-447` selected the lane.
- `291x-448` inventoried raw surface compatibility vs decided metadata.
- `291x-449` landed `GenericMethodRouteSurface` and preserved JSON behavior.

## Verification

The implementation card passed:

```bash
cargo test -q generic_method_route
cargo test -q build_mir_json_root_emits_generic_method_routes
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

## Next

Choose the next compiler-cleanliness lane. Do not continue `.inc` migration,
MapGet lowering, or Stage-B work by momentum.
