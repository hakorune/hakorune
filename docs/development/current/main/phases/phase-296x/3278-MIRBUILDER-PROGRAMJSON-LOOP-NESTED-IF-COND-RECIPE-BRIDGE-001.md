# 3278 - MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-COND-RECIPE-BRIDGE-001

Status: landed

## Purpose

Attach optional `cond_recipe` sidecars to Loop-body nested If RecipeItems while
preserving the existing `VarLtInt` legacy `cond_facts`.

## Implementation

Owners:

- `lang/src/compiler/mirbuilder/stmt_handlers/loop_stmt_handler.hako`
- `lang/src/compiler/mirbuilder/stmt_handlers/loop_nested_if_cond_recipe_bridge_box.hako`

The bridge box owns the sidecar attachment:

```text
LoopNestedIfCondRecipeBridgeBox.if_item(...)
```

It uses `ProgramJsonCompareReaderBox.read_var_int_compare` and falls back to the
legacy `RecipeItemBox.if_item` when the read recipe is invalid.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_programjson_loop_nested_if_cond_recipe_bridge_gate.sh`

The gate runs one AOT row through `ProgramJsonV0PhaseStateBox.parse` and checks:

- loop body nested If keeps `VarLtInt` legacy `cond_facts`
- nested If `cond_recipe` is present
- `BoolRecipe` summary reports `cmp=Lt`

## Claims

- `loop_nested_if_cond_recipe = 1`
- `shared_compare_reader_used = 1`
- `legacy_cond_facts_preserved = 1`

## Non-Claims

- Loop nested If operator expansion
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

`MIRBUILDER-PROGRAMJSON-LOOP-COND-RECIPE-CONSTRUCTOR-CLEANUP-001`
