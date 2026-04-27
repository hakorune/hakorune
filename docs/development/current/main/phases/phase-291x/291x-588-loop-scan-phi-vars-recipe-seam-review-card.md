---
Status: Landed
Date: 2026-04-28
Scope: review the loop_scan_phi_vars_v0 recipe seam and record whether it is a prune target or a shared-vocabulary boundary
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-585-legacy-v0-boundary-inventory-card.md
  - src/mir/builder/control_flow/recipes/loop_scan_phi_vars_v0.rs
  - src/mir/builder/control_flow/facts/loop_scan_phi_vars_v0.rs
  - src/mir/builder/control_flow/facts/loop_scan_phi_vars_v0_shape_routes.rs
  - src/mir/builder/control_flow/facts/loop_scan_phi_vars_v0_helpers.rs
  - src/mir/builder/control_flow/plan/recipe_tree/matcher/loop_scan.rs
---

# 291x-588: Loop Scan Phi Vars Recipe Seam Review

## Goal

Determine whether `loop_scan_phi_vars_v0` should lose its recipe-layer imports
now, or whether those imports are a legitimate shared-vocabulary seam that needs
its own owner move first.

This is inventory-only. It does not change lowering or recipe behavior.

## Evidence

The current dependency is data-only:

- `recipes/loop_scan_phi_vars_v0.rs` defines shared recipe vocabulary:
  `LoopScanPhiVarsV0Recipe`, `LoopScanPhiSegment`, and `NestedLoopRecipe`.
- `facts/loop_scan_phi_vars_v0.rs` stores `LoopScanPhiVarsV0Recipe` and
  `Vec<LoopScanPhiSegment>` inside the extracted facts contract.
- `facts/loop_scan_phi_vars_v0_shape_routes.rs` and
  `facts/loop_scan_phi_vars_v0_helpers.rs` build those recipe values.
- `plan/recipe_tree/matcher/loop_scan.rs` verifies the same
  `LoopScanPhiSegment` vocabulary from facts.

No reverse dependency from recipes back into facts/plan was found, so this is
not a circular boundary. The seam is a shared vocabulary/data contract, not a
behavioral facade.

## Boundaries

- Do not delete the recipe dependency from this card.
- Do not move lowering ownership out of `plan/loop_scan_phi_vars_v0`.
- Treat a future owner move as a shared-vocabulary migration (likely toward
  `recipes/scan_loop_segments.rs`), not a compat-prune card.

## Next Safe Queue

| Order | Size | Target | Reason |
| ---: | --- | --- | --- |
| 1 | L | shared-vocab owner review for `LoopScanPhiSegment` / `NestedLoopRecipe` | recipe seam is data-only, so any move should centralize vocab instead of deleting it |
| 2 | L | `loop_scan_v0` / `loop_scan_methods*_v0` consolidation planning | live segment-routing behavior still needs separate design review |

## Acceptance

- `CURRENT_STATE.toml` points at this review card.
- The seam classification clearly distinguishes data-only shared vocabulary from
  a facade/wrapper prune target.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `git diff --check` passes.

## Result

- Confirmed the `loop_scan_phi_vars_v0` recipe dependency is a shared-vocabulary
  seam, not a circular or behavioral facade.
- Prevented an unsafe blind prune of recipe imports that facts, matcher, and
  family-local lowering still share.
- Fixed the next follow-up shape: shared-vocab owner review first, structural
  consolidation later.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
