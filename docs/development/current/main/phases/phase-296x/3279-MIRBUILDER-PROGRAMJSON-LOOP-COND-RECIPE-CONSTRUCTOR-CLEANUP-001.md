# 3279 - MIRBUILDER-PROGRAMJSON-LOOP-COND-RECIPE-CONSTRUCTOR-CLEANUP-001

Status: landed

## Purpose

Replace the top-level Loop manual `cond_recipe` sidecar mutation with the
dedicated `RecipeItemBox.loop_item_with_cond_recipe` constructor.

This is a behavior-preserving cleanup. It does not add a new Loop condition
operator or change the runtime authority boundary.

## Implementation

Owner:

- `lang/src/compiler/mirbuilder/stmt_handlers/loop_stmt_handler.hako`

The handler now uses:

```text
RecipeItemBox.loop_item_with_cond_recipe(cond_facts, cond_recipe, body_seq)
```

only when `BoolRecipeBox.is_valid_compare(cond_recipe) == 1`. The legacy
`RecipeItemBox.loop_item(cond_facts, body_seq)` fallback remains for invalid
recipes, preserving the previous behavior.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_programjson_loop_cond_recipe_constructor_cleanup_gate.sh`

The gate runs one AOT row through `ProgramJsonV0PhaseStateBox.parse` and checks:

- top-level Loop `cond_recipe` is present
- the summary still reports `cmp=Lt`
- manual `loop_item.set("cond_recipe", cond_recipe)` is gone
- the legacy `loop_item` fallback remains

## Claims

- `loop_item_with_cond_recipe_constructor_used = 1`
- `manual_cond_recipe_set_removed = 1`
- `behavior_preserved = 1`

## Non-Claims

- new Loop condition operator
- Rust loop condition Eq/Ne
- CondSkeleton::IfCond
- RecipeMatcher input authority
- BoolRecipe lowering
- MIR compare/branch emission
- route selection
- runtime route switch
- ProgramJSON runtime route authority
- runtime fallback
- Source Selfhost

## Next

`MIRBUILDER-RUST-LOOP-CONDITION-SHAPE-EQ-NE-CANON-001`
