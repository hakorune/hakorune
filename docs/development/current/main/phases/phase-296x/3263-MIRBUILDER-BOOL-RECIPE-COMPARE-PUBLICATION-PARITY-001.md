# 3263 - MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-PARITY-001

Status: landed

## Scope

Publish a read-only `BoolRecipeCompareV1` from the ProgramJSON
CanonicalLoopFacts numeric-compare consume snapshot.

Path:

```text
ProgramJSON
  -> CanonicalLoopFacts NumericCompare consume snapshot
  -> BoolRecipeCompareV1 publication
```

This does not attach the recipe to `RecipeItem`, execute `RecipeMatcher`, or
emit MIR Compare/Branch instructions.

## Implementation

Owner:

```text
lang/src/compiler/mirbuilder/program_json_bool_recipe_compare_publication.hako
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-bool-recipe-compare-publication-parity-v0.json
```

Gate:

```text
tools/checks/rust_lifecycle_mirbuilder_bool_recipe_compare_publication_parity_gate.sh
```

## Claims

```text
bool_recipe_compare_publication_parity=1
read_only_bool_recipe_compare_publication=1
canonical_loop_facts_numeric_compare_consume_required=1
analysis_only=1
```

## Non-Claims

```text
recipe_item_attachment=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
mir_cmp_emission=0
branch_emission=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
runtime_fallback=0
source_selfhost_claim=0
```

## Verification

```bash
bash tools/checks/rust_lifecycle_mirbuilder_bool_recipe_compare_publication_parity_gate.sh
```

Expected summary:

```text
publication_rows=1
bool_recipe_compare_publication_parity=1
read_only_bool_recipe_compare_publication=1
summary=ok
```

## Next

```text
MIRBUILDER-RECIPEITEM-CONDITION-SLOT-BOOL-RECIPE-BRIDGE-SELECTION-001
```
