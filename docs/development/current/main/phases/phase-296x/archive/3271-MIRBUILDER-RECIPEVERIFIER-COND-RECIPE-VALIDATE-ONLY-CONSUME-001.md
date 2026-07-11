# 3271 - MIRBUILDER-RECIPEVERIFIER-COND-RECIPE-VALIDATE-ONLY-CONSUME-001

Status: landed

## Purpose

Make `RecipeVerifierBox` the first non-diagnostic consumer of the optional
RecipeItem `cond_recipe` sidecar.

This is a validate-only boundary. The verifier may reject malformed
`cond_recipe` values, but it must not make the sidecar RecipeMatcher input
authority, lowering input, route-selection input, or runtime authority.

## Implementation

- `RecipeVerifierBox` imports `BoolRecipeBox`.
- `If` and `Loop` verification now call `_verify_cond_recipe`.
- Missing `cond_recipe` remains valid for legacy RecipeItems.
- Present `cond_recipe` must pass `BoolRecipeBox.is_valid_compare`.
- Invalid sidecars fail fast with:
  `[recipe_verifier] invalid cond_recipe`

## Guard

`tools/checks/rust_lifecycle_mirbuilder_recipeverifier_cond_recipe_validate_only_consume_gate.sh`

The guard verifies:

- the 3270 consume-boundary selection prerequisite is green,
- legacy RecipeItems without `cond_recipe` still verify,
- valid `cond_recipe` sidecars verify without changing port-sig counts,
- malformed `Loop` and `If` sidecars are rejected,
- RecipeMatcher and shape-control consumers still do not read `cond_recipe`.

## Claims

Claimed:

- `recipeverifier_cond_recipe_validate_only_consume = 1`
- `malformed_cond_recipe_rejected = 1`
- `legacy_recipeitem_without_cond_recipe_still_valid = 1`
- `valid_cond_recipe_port_sig_unchanged = 1`

Explicitly not claimed:

- RecipeMatcher input authority
- BoolRecipe lowering
- MIR compare or branch emission
- route selection
- runtime route switch
- ProgramJSON runtime authority
- runtime fallback
- Source Selfhost

## Next

`MIRBUILDER-RECIPEMATCHER-COND-RECIPE-INPUT-CONSUME-BOUNDARY-SELECTION-001`

Select the next boundary before any RecipeMatcher input consume implementation.
