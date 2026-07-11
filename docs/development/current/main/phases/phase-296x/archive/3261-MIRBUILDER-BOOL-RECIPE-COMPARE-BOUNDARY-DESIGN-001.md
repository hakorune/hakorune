# 3261 - MIRBUILDER-BOOL-RECIPE-COMPARE-BOUNDARY-DESIGN-001

Status: landed

## Scope

Define the data-only lowering-facing bool recipe boundary:

```text
NumericCompareCanonSnapshotV1 semantic fields after symbol resolution
  -> BoolRecipeCompareV1
```

This card does not attach the recipe to `RecipeItem`, does not make
CanonicalLoopFacts consume the snapshot, and does not emit MIR.

## Implementation

Owner:

```text
lang/src/compiler/mirbuilder/recipe/bool_recipe_box.hako
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-bool-recipe-compare-boundary-v0.json
```

Gate:

```text
tools/checks/rust_lifecycle_mirbuilder_bool_recipe_compare_boundary_gate.sh
```

## Boundary

`BoolRecipeCompareV1` uses numeric authority fields:

```text
lhs_symbol_id
cmp_code
rhs_bound.kind_code
rhs_bound.i64 / symbol_id / haystack_id / needle_id
```

Variable names are not MapBox authority in this boundary. The next consume
card must resolve `NumericCompareCanonSnapshotV1` names to symbol ids before
building BoolRecipe.

## Claims

```text
bool_recipe_compare_boundary=1
numeric_compare_canon_fields_consumed_after_symbol_resolution=1
bound_expr_shared=1
analysis_only=1
```

## Non-Claims

```text
raw_variable_name_map_authority=0
recipe_item_attachment=0
canonical_loop_facts_consume=0
recipe_matcher_input_authority=0
mir_cmp_emission=0
branch_emission=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
source_selfhost_claim=0
```

## Verification

```bash
bash tools/checks/rust_lifecycle_mirbuilder_bool_recipe_compare_boundary_gate.sh
```

Expected summary:

```text
boundary_rows=4
bool_recipe_compare_boundary=1
numeric_compare_canon_fields_consumed_after_symbol_resolution=1
summary=ok
```

## Next

```text
MIRBUILDER-CANONICAL-LOOP-FACTS-NUMERIC-COMPARE-CANON-CONSUME-001
```
