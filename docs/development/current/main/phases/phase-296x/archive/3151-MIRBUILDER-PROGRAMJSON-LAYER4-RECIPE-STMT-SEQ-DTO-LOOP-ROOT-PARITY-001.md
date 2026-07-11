# 3151 - MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-STMT-SEQ-DTO-LOOP-ROOT-PARITY-001

Status: landed

## Scope

Expand `RecipeStmtSeqDtoSnapshotV1` so the root recipe sequence summary can
observe control children and report the covered loop roots as:

```text
Local>Loop>Return
```

This keeps `shape_kind` selection unchanged. The `_recipe_stmt_seq_sig` path
remains stmt-only for non-control shape selection; only the DTO summary path now
uses the root child token helper.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_recipe_stmt_seq_dto_loop_root_parity_gate.sh
```

Expected guard result:

```text
owner=ProgramJsonRecipeStmtSeqDtoSnapshotV1
expanded_rows=6
programjson_traversal_used=1
structured_recipe_dto_constructed=1
recipe_verifier_used=1
recipe_root_seq_scanner_used=1
loop_root_children_supported=1
shape_kind_selection=0
mir_json_route_green=1
runtime_parity_green=1
expanded_loop_payload_prerequisite_green=1
full_recipe_matcher_execution=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
source_selfhost_claim=0
```

## Non-Claims

```text
shape_kind selection
runtime route switch
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
MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-STMT-SEQ-DTO-LOOP-ROOT-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
