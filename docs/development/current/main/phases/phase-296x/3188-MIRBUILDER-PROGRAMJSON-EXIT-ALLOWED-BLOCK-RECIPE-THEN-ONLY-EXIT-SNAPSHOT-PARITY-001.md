# 3188 - MIRBUILDER-PROGRAMJSON-EXIT-ALLOWED-BLOCK-RECIPE-THEN-ONLY-EXIT-SNAPSHOT-PARITY-001

Status: landed

## Scope

Implement `ProgramJsonExitAllowedBlockRecipeThenOnlyExitSnapshotV1` as a
ProgramJSON-fed `ExitAllowedBlockRecipeBox` ThenOnlyExit contract capability.

Covered row:

```text
local_if_then_return_else_local_final_return_var
```

The owner traverses ProgramJSON through `ProgramJsonV0PhaseStateBox`, extends
`IfStmtHandler` to produce `If.then Exit(Return)` and
`If.else Seq([Local])`, projects that recipe item to
`IfThenReturnElseLocal`, and feeds the token to the existing
`ExitAllowedBlockRecipeBox` reducer.

Held:

```text
ExitAll
RecipeBodies materialization
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_exit_allowed_block_recipe_then_only_exit_snapshot_parity_gate.sh
```

Expected guard result:

```text
owner=ProgramJsonExitAllowedBlockRecipeThenOnlyExitSnapshotV1
row_count=1
programjson_traversal_used=1
recipe_root_traversal_used=1
if_stmt_handler_producer_extended=1
exit_allowed_reducer_called=1
then_only_exit_token_projected=1
mir_json_route_green=1
runtime_parity_green=1
route_selection=0
source_selfhost_claim=0
```

## Non-Claims

```text
ExitAll
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
MIRBUILDER-PROGRAMJSON-EXIT-ALLOWED-BLOCK-RECIPE-THEN-ONLY-EXIT-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
