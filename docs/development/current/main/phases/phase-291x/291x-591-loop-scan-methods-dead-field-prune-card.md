---
Status: Landed
Date: 2026-04-28
Scope: prune unused recipe fields from loop_scan_methods_v0 and loop_scan_methods_block_v0
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/recipes/loop_scan_methods_v0.rs
  - src/mir/builder/control_flow/recipes/loop_scan_methods_block_v0.rs
  - src/mir/builder/control_flow/facts/loop_scan_methods_v0.rs
  - src/mir/builder/control_flow/facts/loop_scan_methods_block_v0.rs
  - src/mir/builder/control_flow/facts/loop_scan_methods_block_v0_recipe_builder.rs
  - src/mir/builder/control_flow/facts/loop_scan_methods_block_v0_shape_routes.rs
---

# 291x-591: Loop Scan Methods Dead Field Prune

## Goal

Remove unused recipe evidence fields from the two scan-methods recipe structs,
while keeping the live segment-routing contract intact.

This is BoxShape-only cleanup. It does not change matching, route selection, or
lowering behavior.

## Evidence

The live methods pipelines only read `facts.recipe.segments`.

The following fields had no live readers and only remained as retained evidence:

- `LoopScanMethodsV0Recipe.next_i_var`
- `LoopScanMethodsV0Recipe.body`
- `LoopScanMethodsBlockV0Recipe.next_i_var`
- `LoopScanMethodsBlockV0Recipe.body`

For the block variant, `LoopScanMethodsBlockShapeMatch.next_i_var` also became a
pure pass-through value once the recipe stopped storing it.

## Boundaries

- Keep the `segments` vectors and the existing segment vocabulary untouched.
- Remove only dead evidence/pass-through fields.
- Do not alter any nested-loop lowering logic.

## Acceptance

- No live code reads the removed methods-recipe fields.
- `LoopScanMethodsBlockShapeMatch` no longer carries dead pass-through state.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check --release --bin hakorune` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Reduced both methods recipe structs to their live `segments` payload.
- Simplified methods facts/builders so they no longer manufacture dead evidence.
- Removed the dead `next_i_var` pass-through from the block shape-match struct.

## Verification

```bash
rg -n "next_i_var|\\.body\\b" src/mir/builder/control_flow/{facts,recipes,plan/loop_scan_methods_v0,plan/loop_scan_methods_block_v0} -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo fmt -- --check
cargo check --release --bin hakorune
git diff --check
```
