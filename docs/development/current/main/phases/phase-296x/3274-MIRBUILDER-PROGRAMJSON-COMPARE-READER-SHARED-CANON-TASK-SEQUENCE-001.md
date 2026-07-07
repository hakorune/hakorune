# 3274 - MIRBUILDER-PROGRAMJSON-COMPARE-READER-SHARED-CANON-TASK-SEQUENCE-001

Status: landed

## Purpose

Taskize the If/Loop condition symmetry cleanup after the `cond_recipe`
sidecar, verifier validation, and RecipeMatcher-facing observe-only snapshot
have landed.

The issue is not missing `BoolRecipe` vocabulary anymore. The issue is that
If/Loop handlers still parse ProgramJSON `Compare` locally and diverge.

## Decision

Use one shared ProgramJSON compare reader before adding more If/Loop condition
rows.

Selected next:

`MIRBUILDER-PROGRAMJSON-COMPARE-READER-SHARED-CANON-001`

## Ordered Tasks

1. `MIRBUILDER-PROGRAMJSON-COMPARE-READER-SHARED-CANON-001`
   - Add a shared reader for `Var op Int`.
   - Reader-only. Do not change consumers yet.

2. `MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-EQ-BEHAVIOR-PRESERVING-001`
   - Attach `cond_recipe` to existing If `Var == Int` rows.
   - Preserve legacy `cond_facts`.

3. `MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-FIRST-NON-EQ-ROW-001`
   - Add one user-visible non-Eq If condition row through the shared reader.

4. `MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-COND-RECIPE-BRIDGE-001`
   - Attach `cond_recipe` to Loop-body nested If items.

5. `MIRBUILDER-PROGRAMJSON-LOOP-COND-RECIPE-CONSTRUCTOR-CLEANUP-001`
   - Replace manual Loop sidecar `set` with `loop_item_with_cond_recipe`.

6. `MIRBUILDER-RUST-LOOP-CONDITION-SHAPE-EQ-NE-CANON-001`
   - Add Eq/Ne to Rust loop condition shape canon.

7. `MIRBUILDER-CONDSKELETON-IFCOND-CONSULTATION-001`
   - Decide whether If conditions should enter CondProfile/CondSkeleton.

## Non-Claims

- shared compare reader implementation
- If operator expansion
- Loop nested If bridge
- Rust Eq/Ne implementation
- CondSkeleton::IfCond
- RecipeMatcher input authority
- BoolRecipe lowering
- route selection
- runtime route switch
- Source Selfhost
