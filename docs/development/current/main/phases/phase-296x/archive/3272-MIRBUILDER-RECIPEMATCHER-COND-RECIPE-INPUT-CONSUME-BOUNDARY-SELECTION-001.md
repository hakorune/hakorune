# 3272 - MIRBUILDER-RECIPEMATCHER-COND-RECIPE-INPUT-CONSUME-BOUNDARY-SELECTION-001

Status: landed

## Purpose

Select the next safe boundary for `cond_recipe` after RecipeVerifier
validate-only consumption.

The selected boundary is not direct RecipeMatcher authority. It is an
observe-only, read-only input snapshot projection from verified `cond_recipe`.

## Decision

Selected:

- `ObserveOnlyCondRecipeMatcherInputSnapshot`

Rejected:

- direct RecipeMatcher `cond_recipe` authority,
- shape-control consumption,
- route-selection consumption.

## Contract

The next implementation may project verified `cond_recipe` into a
matcher-facing snapshot for observation and parity. It must not:

- make `cond_recipe` RecipeMatcher input authority,
- run full RecipeMatcher execution,
- select routes,
- lower to MIR compare or branch,
- mutate MIR,
- allocate IDs,
- switch runtime route authority.

## Guard

`tools/checks/rust_lifecycle_mirbuilder_recipematcher_cond_recipe_input_consume_boundary_selection_guard.sh`

The guard requires 3271 to be green and verifies that current RecipeMatcher and
shape-control code still do not consume `cond_recipe`.

## Next

`MIRBUILDER-RECIPEMATCHER-COND-RECIPE-OBSERVE-ONLY-INPUT-SNAPSHOT-001`
