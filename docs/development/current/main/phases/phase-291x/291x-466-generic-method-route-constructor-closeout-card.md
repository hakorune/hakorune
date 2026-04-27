---
Status: Landed
Date: 2026-04-27
Scope: Close GenericMethodRoute constructor SSOT lane
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-291x/291x-463-next-lane-selection-card.md
  - docs/development/current/main/phases/phase-291x/291x-464-generic-method-route-constructor-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-465-generic-method-route-constructor-card.md
---

# 291x-466: GenericMethodRoute Constructor Closeout

## Goal

Close the `GenericMethodRoute` constructor SSOT lane and return the phase to
next-lane selection.

This card is closeout only. No code changed.

## Result

- `291x-463` selected the lane.
- `291x-464` inventoried direct `GenericMethodRoute` construction sites.
- `291x-465` landed `GenericMethodRoute::new(...)` as the route record
  assembly entry and made the route fields private.

## Verification

The implementation card passed:

```bash
rg -n "GenericMethodRoute \\{" src/mir src/runner -g '*.rs'
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
