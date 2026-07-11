# 3182 - MIRBUILDER-PROGRAMJSON-NO-EXIT-BLOCK-RECIPE-JOIN-THEN-ELSE-SNAPSHOT-PARITY-001

Status: landed

## Scope

Implement `ProgramJsonNoExitBlockRecipeJoinThenElseSnapshotV1` as a
ProgramJSON-fed `NoExitBlockRecipeBox` JoinThenElse contract capability.

Covered row:

```text
local_if_then_local_else_print_final_return_var
```

The owner traverses ProgramJSON through `ProgramJsonV0PhaseStateBox`, extends
`IfStmtHandler` to produce `If.then Seq([Local])` and
`If.else Seq([Print])`, projects that recipe item to
`IfThenLocalElsePrint`, and feeds the token to the existing
`NoExitBlockRecipeBox` reducer.

Held:

```text
additional ExitAllowed if-modes
RecipeBodies materialization
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_no_exit_block_recipe_join_then_else_snapshot_parity_gate.sh
```

Expected guard result:

```text
owner=ProgramJsonNoExitBlockRecipeJoinThenElseSnapshotV1
row_count=1
programjson_traversal_used=1
recipe_root_traversal_used=1
if_stmt_handler_producer_extended=1
no_exit_reducer_called=1
join_then_else_token_projected=1
mir_json_route_green=1
runtime_parity_green=1
route_selection=0
source_selfhost_claim=0
```

## Non-Claims

```text
additional ExitAllowed if-modes
RecipeBodies materialization
full RecipeMatcher execution
runtime route switch
route selection
MIR lowering
MIR mutation
ID allocation
ProgramJSON full parser
Source Selfhost
```

## Next

```text
MIRBUILDER-PROGRAMJSON-NO-EXIT-BLOCK-RECIPE-JOIN-THEN-ELSE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
