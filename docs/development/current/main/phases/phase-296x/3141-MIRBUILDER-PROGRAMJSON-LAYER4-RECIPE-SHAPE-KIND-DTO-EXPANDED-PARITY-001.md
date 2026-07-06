# 3141 - MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-SHAPE-KIND-DTO-EXPANDED-PARITY-001

Status: landed

## Scope

Expand `ProgramJsonRecipeShapeKindDtoSnapshotV1` with the existing
Return-only shape kinds that were already present in the Recipe shape-kind
SSOT:

```text
phase16_return_newbox_stringbox_abc
phase13_return_call_id0
phase15_return_call_id1_int9
phase18_return_call_id1_int7
phase14_return_boxcall_stringbox_length_abc
phase17_return_boxcall_stringbox_indexof_b_abc
```

This card also fixes the Return handler expanded path to preserve dynamic
tokens with `BoxHelpers.same_token` instead of raw string equality or `"" +`
token coercion.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_recipe_shape_kind_dto_expanded_parity_gate.sh
```

Expected guard result:

```text
owner=ProgramJsonRecipeShapeKindDtoSnapshotV1
expanded_rows=6
programjson_traversal_used=1
structured_recipe_dto_constructed=1
recipe_verifier_used=1
recipe_stmt_seq_scanner_used=1
shape_kind_selection=1
route_selection=0
mir_json_route_green=1
runtime_parity_green=1
runtime_route_switch=0
full_recipe_matcher_execution=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
source_selfhost_claim=0
```

## Non-Claims

```text
runtime route switch
full ASTNode projector retirement
full RecipeMatcher execution
route selection
MIR lowering
MIR mutation
ID allocation
ProgramJSON full parser
Source Selfhost
```

## Next

```text
MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-SHAPE-KIND-DTO-EXPANDED-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
