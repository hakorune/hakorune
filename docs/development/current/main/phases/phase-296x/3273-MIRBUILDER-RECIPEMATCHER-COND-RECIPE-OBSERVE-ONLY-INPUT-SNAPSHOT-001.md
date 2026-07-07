# 3273 - MIRBUILDER-RECIPEMATCHER-COND-RECIPE-OBSERVE-ONLY-INPUT-SNAPSHOT-001

Status: landed

## Purpose

Project a verified RecipeItem `cond_recipe` sidecar into a read-only,
matcher-facing input snapshot.

This is still observe-only. It does not run RecipeMatcher as authority, select
routes, lower MIR, mutate MIR, allocate IDs, or switch runtime routes.

## Implementation

`ProgramJsonRecipeMatcherExecutionBoundaryBox` now exposes:

- `cond_recipe_input_snapshot(recipe_item): MapBox`
- `cond_recipe_input_summary(snapshot): String`

The snapshot carries the compare fields from `BoolRecipeBox` in flat form:

- `lhs_symbol_id`
- `cmp_code`
- `rhs_bound_kind_code`
- `rhs_bound_i64`
- `rhs_bound_symbol_id`

All downstream authority flags remain zero.

## Guard

`tools/checks/rust_lifecycle_mirbuilder_recipematcher_cond_recipe_observe_only_input_snapshot_gate.sh`

The guard verifies:

- 3272 selection is green,
- valid `cond_recipe` projects into a read-only matcher-facing snapshot,
- missing `cond_recipe` fails as `cond_recipe_missing`,
- RecipeMatcher execution, lowering, route selection, runtime switch, and
  Source Selfhost claims remain zero.

## Next

`MIRBUILDER-RECIPEMATCHER-COND-RECIPE-FIRST-PARITY-ROW-SELECTION-001`
