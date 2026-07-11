# 3154 - MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-SHAPE-KIND-DTO-LOOP-ROOT-PARITY-001

Status: landed

## Scope

Expand `RecipeShapeKindDtoSnapshotV1` to cover the selected
`ProgramJsonRecipeShapeKindDtoLoopRootV1` capability.

Covered root recipe sequence:

```text
Local>Loop>Return
```

Covered shape kind:

```text
phase21_local_loop_if_varltint_then_return_int_body_inc_return_var_or_int
```

This keeps the claim at the DTO snapshot layer. It does not switch the runtime
route, execute RecipeMatcher, select backend routes, lower MIR, mutate MIR,
allocate IDs, or claim Source Selfhost.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_recipe_shape_kind_dto_loop_root_parity_gate.sh
```

Expected guard result:

```text
owner=ProgramJsonRecipeShapeKindDtoSnapshotV1
expanded_rows=6
programjson_traversal_used=1
structured_recipe_dto_constructed=1
recipe_verifier_used=1
recipe_root_seq_scanner_used=1
loop_root_children_supported=1
shape_kind_selection=1
route_selection=0
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
MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-SHAPE-KIND-DTO-LOOP-ROOT-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
