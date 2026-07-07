# 3284 - MIRBUILDER-BOOL-RECIPE-COMPARE-LOWERING-OBSERVE-ONLY-PILOT-001

Status: landed

## Purpose

Project `BoolRecipe::Compare` into a read-only lowering intent snapshot without
emitting MIR.

## Implementation

Added:

```text
lang/src/compiler/mirbuilder/bool_recipe_compare_lowering_intent_snapshot.hako
```

The owner consumes the existing `BoolRecipeComparePublicationV1` boundary and
publishes:

```text
BoolRecipeCompareLoweringIntentSnapshotV1
```

The snapshot preserves:

- `lhs_symbol_id`
- `cmp_code`
- `mir_compare_op_code`
- `rhs_bound_kind_code`
- `rhs_bound_i64`
- `rhs_bound_symbol_id`

It keeps all execution-side claims at zero.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_bool_recipe_compare_lowering_observe_only_pilot_gate.sh`

The gate runs the `.hako` owner through AOT/EXE and verifies the `var_le_literal`
row.

## Claims

- `bool_recipe_compare_lowering_intent_snapshot = 1`
- `observe_only_lowering_intent = 1`
- `analysis_only = 1`

## Non-Claims

- BoolRecipe lowering executed
- MIR Compare emission
- Branch emission
- BasicBlock mutation
- ValueId allocation
- route selection
- runtime route switch
- ProgramJSON runtime route authority
- runtime fallback
- Source Selfhost

## Next

`MIRBUILDER-PROGRAMJSON-IF-LOOP-COMPARE-OPERATOR-EXPANSION-SELECTION-001`
