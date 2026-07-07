# 3259 - MIRBUILDER-NUMERIC-COMPARE-CANON-FOLLOWON-TASK-SEQUENCE-001

Status: landed

## Scope

Taskize the post-3258 numeric compare canon sequence.

The final design separates raw compare readers from loop facts, recipe, and
lowering:

```text
source AST / ProgramJSON Compare
  -> NumericCompareCanonSnapshotV1
  -> CanonicalLoopFacts
  -> BoolRecipe::Compare
  -> MIR Cmp
  -> Branch
```

## Selected Sequence

```text
1. MIRBUILDER-PROGRAMJSON-LOOP-CONDITION-NUMERIC-COMPARE-CANON-PARITY-001
2. MIRBUILDER-BOOL-RECIPE-COMPARE-BOUNDARY-DESIGN-001
3. MIRBUILDER-CANONICAL-LOOP-FACTS-NUMERIC-COMPARE-CANON-CONSUME-001
4. MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-PARITY-001
```

## Next Card Contract

The next implementation card is Option A:

```text
ProgramJSON Compare
  -> NumericCompareCanonSnapshotV1
```

It must mirror Rust `ConditionShape::VarCompareBound` authority and stay
analysis-only.

Allowed claims:

```text
numeric_compare_canon_snapshot_v1=1
programjson_compare_to_numeric_compare_canon=1
rust_oracle_parity_for_numeric_compare_canon=1
bound_expr_shared=1
analysis_only=1
```

Forbidden claims:

```text
canonical_loop_facts_consume=0
recipe_matcher_input_authority=0
bool_recipe_lowering=0
mir_cmp_emission=0
branch_emission=0
route_selection=0
runtime_route_switch=0
programjson_runtime_route_authority=0
source_selfhost_claim=0
```

## Design Guardrails

```text
raw_AST_rewrite=0
raw_ProgramJSON_rewrite=0
i <= 3 to i < 4 rewrite=0
CanonicalLoopFacts reads raw Compare=0
BoolRecipe carries ASTNode/ProgramJSON offsets=0
variable_variable_reversed_without_context_claim=0
literal_only_bound_kind_design=0
```

`n >= i` remains unclaimed until a context-aware stage can use loop-var or
update-target context.
