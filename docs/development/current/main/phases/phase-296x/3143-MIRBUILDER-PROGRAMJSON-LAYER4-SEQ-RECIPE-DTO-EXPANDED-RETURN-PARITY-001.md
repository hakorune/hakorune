# 3143 - MIRBUILDER-PROGRAMJSON-LAYER4-SEQ-RECIPE-DTO-EXPANDED-RETURN-PARITY-001

Status: landed

## Scope

Expand `ProgramJsonSeqRecipeDtoSnapshotV1` with the Return-only rows proven by
the Recipe shape-kind expanded checkpoint:

```text
return_new_stringbox_abc
return_call_id0
return_call_id1_int9
return_call_id1_int7
return_method_stringbox_length_abc
return_method_stringbox_indexof_b_abc
```

This is parent Seq DTO parity only. It does not switch runtime routes, execute
RecipeMatcher, select backend routes, lower MIR, mutate MIR, allocate IDs, or
claim Source Selfhost.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_seq_recipe_dto_expanded_return_parity_gate.sh
```

Expected guard result:

```text
owner=ProgramJsonSeqRecipeDtoSnapshotV1
expanded_rows=6
programjson_traversal_used=1
structured_recipe_dto_constructed=1
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
MIRBUILDER-PROGRAMJSON-LAYER4-SEQ-RECIPE-DTO-EXPANDED-RETURN-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
