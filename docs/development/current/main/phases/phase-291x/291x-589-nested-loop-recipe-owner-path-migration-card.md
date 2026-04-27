---
Status: Landed
Date: 2026-04-28
Scope: redirect loop_scan_phi_vars_v0 family NestedLoopRecipe imports to the shared scan_loop_segments owner
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-588-loop-scan-phi-vars-recipe-seam-review-card.md
  - src/mir/builder/control_flow/recipes/scan_loop_segments.rs
  - src/mir/builder/control_flow/facts/loop_scan_phi_vars_v0_helpers.rs
  - src/mir/builder/control_flow/plan/loop_scan_phi_vars_v0/
---

# 291x-589: Nested Loop Recipe Owner-Path Migration

## Goal

Land the smallest safe shared-vocabulary cleanup from 291x-588 by pointing
`NestedLoopRecipe` users at the real SSOT owner.

This is BoxShape-only cleanup. It does not remove the phi-vars recipe shim or
change any lowering behavior.

## Evidence

291x-588 showed that `NestedLoopRecipe` is actually owned by
`recipes/scan_loop_segments.rs`, while the phi-vars recipe module only re-exported
that type for family-local convenience.

The live importers in the phi-vars family only needed the shared nested-loop
vocabulary and did not depend on any phi-vars-specific recipe wrapper behavior.

## Boundaries

- Redirect only `NestedLoopRecipe` imports.
- Keep `LoopScanPhiSegment` and `LoopScanPhiVarsV0Recipe` on the phi-vars recipe
  surface for now.
- Do not delete `recipes/loop_scan_phi_vars_v0.rs` from this card.

## Acceptance

- No phi-vars family file imports `NestedLoopRecipe` via
  `recipes::loop_scan_phi_vars_v0`.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check --release --bin hakorune` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Rewired the six phi-vars family import sites to
  `recipes::scan_loop_segments::NestedLoopRecipe`.
- Reduced one layer of shared-vocabulary indirection without touching the
  remaining phi-vars-specific recipe aliases.

## Verification

```bash
rg -n "recipes::loop_scan_phi_vars_v0::NestedLoopRecipe" src/mir/builder/control_flow -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo fmt -- --check
cargo check --release --bin hakorune
git diff --check
```
