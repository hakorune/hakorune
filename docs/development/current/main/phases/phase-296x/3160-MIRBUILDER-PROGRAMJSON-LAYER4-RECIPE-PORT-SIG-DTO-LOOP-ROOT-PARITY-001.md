# 3160 - MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-PORT-SIG-DTO-LOOP-ROOT-PARITY-001

Status: landed

## Scope

Prove `RecipePortSigDtoSnapshotV1` over the covered Local>Loop>Return
loop-root rows selected by 3159.

This uses the existing `ProgramJsonRecipePortSigDtoSnapshotBox` owner:

```text
ProgramJSON -> ProgramJsonV0PhaseStateBox.parse/2 -> recipe_root
  -> RecipeVerifierBox.verify/2 -> RecipePortSigBox.snapshot/1
```

No new `.hako` implementation is required for this slice. The point is to
prove the existing structured Recipe DTO metadata path over the loop-root rows
that 3157/3158 established for Seq DTO.

## Covered Rows

```text
loop_if_then_return_new_stringbox_abc_assignment_final_return_var
loop_if_then_return_call_id0_assignment_final_return_var
loop_if_then_return_call_id1_int9_assignment_final_return_var
loop_if_then_return_call_id1_int7_assignment_final_return_var
loop_if_then_return_method_stringbox_length_abc_assignment_final_return_var
loop_if_then_return_method_stringbox_indexof_b_abc_assignment_final_return_var
```

Expected summary for each row:

```text
snapshot_kind=RecipePortSigDtoSnapshotV1;err=0;matched=1;def_count=1;update_count=2
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_recipe_port_sig_dto_loop_root_parity_gate.sh
```

Expected guard result:

```text
owner=ProgramJsonRecipePortSigDtoSnapshotV1
expanded_rows=6
programjson_traversal_used=1
structured_recipe_dto_constructed=1
recipe_verifier_used=1
recipe_port_sig_snapshot_used=1
loop_root_children_supported=1
route_selection=0
mir_json_route_green=1
runtime_parity_green=1
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
MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-PORT-SIG-DTO-LOOP-ROOT-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
