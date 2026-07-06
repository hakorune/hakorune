# 3153 - MIRBUILDER-PROGRAMJSON-LAYER4-NEXT-RECIPE-DTO-CAPABILITY-SELECTION-001

Status: landed

## Scope

Select the next concrete ProgramJSON Layer4 Recipe DTO capability after the
covered `RecipeStmtSeqDtoSnapshotV1` loop-root rows were marked as a scoped
Rust ASTNode projector retire-candidate.

This is a selection card only. It does not implement the parity gate, switch
runtime routes, execute RecipeMatcher, select backend routes, lower MIR, mutate
MIR, allocate IDs, or claim Source Selfhost.

## Selected Capability

```text
ProgramJsonRecipeShapeKindDtoLoopRootV1
```

Next card:

```text
MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-SHAPE-KIND-DTO-LOOP-ROOT-PARITY-001
```

Reason:

```text
3151 proved RecipeStmtSeqDtoSnapshotV1 can observe covered Local>Loop>Return
root recipe sequences. ProgramJsonRecipeShapeKindDtoSnapshotBox is the next
parent DTO still limited to stmt-only sequence signatures. The covered
LoopRecipeDtoSnapshotV1 rows already carry the Rust oracle shape_kind token:
phase21_local_loop_if_varltint_then_return_int_body_inc_return_var_or_int.
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_next_recipe_dto_capability_selection_guard.sh
```

Expected guard result:

```text
selected_capability=ProgramJsonRecipeShapeKindDtoLoopRootV1
selected_next_card=MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-SHAPE-KIND-DTO-LOOP-ROOT-PARITY-001
source_rows=6
must_construct_structured_recipe_dto=1
must_use_recipe_verifier=1
must_use_recipe_root_sequence_scanner=1
must_select_shape_kind=1
implementation_done=0
parity_gate_green=0
source_selfhost_claim=0
```

## Non-Claims

```text
implementation done
parity gate green
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
MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-SHAPE-KIND-DTO-LOOP-ROOT-PARITY-001
```
