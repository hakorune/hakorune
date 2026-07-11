# 3137 - MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-STMT-SEQ-DTO-PARITY-001

Status: landed

## Scope

Add `ProgramJsonRecipeStmtSeqDtoSnapshotV1` as the next Layer4 ProgramJSON
Recipe DTO capability after the Recipe PortSig retire-candidate checkpoint.

This owner consumes `ProgramJsonV0PhaseStateBox.parse/2`, verifies
`recipe_root` through `RecipeVerifierBox.verify/2`, and asks
`MirJsonV0ShapeRecipeSeq` for the stmt-only sequence summary.  It deliberately
stops before `shape_kind` selection, RecipeMatcher execution, route selection,
MIR lowering, MIR mutation, and ID allocation.

The supporting cleanup in `MirJsonV0ShapeRecipeSeq` removes the AOT-unstable
`array_len` dependency from the sequence summary path and preserves raw tokens
for `same_token` checks.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_recipe_stmt_seq_dto_parity_gate.sh
```

Expected guard result:

```text
owner=ProgramJsonRecipeStmtSeqDtoSnapshotV1
programjson_traversal_used=1
structured_recipe_dto_constructed=1
recipe_verifier_used=1
recipe_stmt_seq_scanner_used=1
shape_kind_selection=0
mir_json_route_green=1
runtime_parity_green=1
runtime_route_switch=0
full_recipe_matcher_execution=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
source_selfhost_claim=0
```

Regression check:

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_seq_recipe_dto_parity_gate.sh
```

## Non-Claims

```text
shape_kind selection
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
MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-STMT-SEQ-DTO-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
